use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
    ops::{Add, Div, Sub},
    sync::Arc,
    time::{Duration, Instant},
    vec,
};

use fxhash::FxHashMap;
use parking_lot::{Condvar, Mutex};
use tuple::{A2, T2};

use crate::{
    search::{Conflict, ConflictType, Constraint, ConstraintSet, ConstraintType, LandmarkSet},
    Heuristic, HeuristicBuilder, Interval, LSippConfig, LSippStats, LimitValues, Move,
    ReverseResumableAStar, RraStats, SafeIntervalPathPlanningWithLandmarks, SippState, Solution,
    State, Task, TransitionSystem,
};

struct Critical<S, A, C, DC>
where
    S: Debug + State + Eq + Hash + Clone,
    C: Debug + Default + Copy + Ord + LimitValues + Sub<C, Output = DC>,
    DC: Default + Copy + Ord,
{
    queue: BinaryHeap<Reverse<Arc<CbsNode<S, A, C, DC>>>>,
    ongoing: usize,
    best: Option<Arc<CbsNode<S, A, C, DC>>>,
    stats: CbsStats,
}

struct Shared<TS, S, A, C, DC>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: Debug + State + Eq + Hash + Clone,
    C: Debug
        + Eq
        + PartialOrd
        + Ord
        + Add<DC, Output = C>
        + Sub<C, Output = DC>
        + Copy
        + Default
        + LimitValues,
    DC: Default + Copy + Ord,
{
    transition_system: Arc<TS>,
    critical: Mutex<Critical<S, A, C, DC>>,
    monitor: Condvar,
}

/// Implementation of the Conflict-Based Search algorithm that plans collision-free paths for a set of agents.
pub struct ConflictBasedSearch<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC> + Send + Sync,
    S: Debug + State + Eq + Hash + Clone + Send + Sync,
    A: Debug + Copy + Send + Sync,
    C: Debug
        + Hash
        + Eq
        + PartialOrd
        + Ord
        + Add<DC, Output = C>
        + Sub<DC, Output = C>
        + Sub<C, Output = DC>
        + Copy
        + Default
        + LimitValues
        + Send
        + Sync,
    DC: Debug
        + Hash
        + Ord
        + Sub<DC, Output = DC>
        + Div<f64, Output = DC>
        + Copy
        + Default
        + Send
        + Sync,
    H: Heuristic<TS, S, A, C, DC> + Send + Sync,
{
    shared: Shared<TS, S, A, C, DC>,
    _phantom: PhantomData<H>,
}

impl<TS, S, A, C, DC, H> ConflictBasedSearch<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC> + Send + Sync,
    S: Debug + State + Eq + Hash + Clone + Send + Sync,
    A: Debug + Copy + Send + Sync,
    C: Debug
        + Hash
        + Eq
        + PartialOrd
        + Ord
        + Add<DC, Output = C>
        + Sub<DC, Output = C>
        + Sub<C, Output = DC>
        + Copy
        + Default
        + LimitValues
        + Send
        + Sync,
    DC: Debug
        + Hash
        + Ord
        + Sub<DC, Output = DC>
        + Div<f64, Output = DC>
        + Copy
        + Default
        + Send
        + Sync,
    H: Heuristic<TS, S, A, C, DC> + Send + Sync,
{
    /// Creates a new instance of the Conflict-Based Search algorithm.
    ///
    /// # Arguments
    ///
    /// * `transition_system` - The transition system in which the agents navigate.
    pub fn new(transition_system: Arc<TS>) -> Self {
        Self {
            shared: Shared {
                transition_system,
                critical: Mutex::new(Critical {
                    queue: BinaryHeap::new(),
                    ongoing: 0,
                    best: None,
                    stats: CbsStats::default(),
                }),
                monitor: Condvar::new(),
            },
            _phantom: PhantomData,
        }
    }

    fn init(
        shared: &Shared<TS, S, A, C, DC>,
        config: &CbsConfig<TS, S, A, C, DC, H>,
        lsipp: &mut SafeIntervalPathPlanningWithLandmarks<
            TS,
            S,
            A,
            C,
            DC,
            ReverseResumableAStar<TS, S, A, C, DC, H>,
        >,
    ) {
        {
            let mut critical = shared.critical.lock();
            critical.queue.clear();
            critical.best = None;
            critical.stats = CbsStats::default();
        }

        if let Some(root) = Self::get_root(config, lsipp) {
            Self::enqueue(shared, config, root, lsipp);
        }
    }

    fn get_root(
        config: &CbsConfig<TS, S, A, C, DC, H>,
        lsipp: &mut SafeIntervalPathPlanningWithLandmarks<
            TS,
            S,
            A,
            C,
            DC,
            ReverseResumableAStar<TS, S, A, C, DC, H>,
        >,
    ) -> Option<CbsNode<S, A, C, DC>> {
        let mut root = CbsNode::default();

        // Solve each task independently
        for (agent, task) in config.tasks.iter().enumerate() {
            if config.frozen.contains_key(&agent) {
                let solution = config.frozen[&agent].clone();
                root.total_cost = solution.cost + root.total_cost - task.initial_cost;
                root.solutions.push(solution);
                continue;
            }

            let config = LSippConfig::new_with_pivots(
                task.clone(),
                Default::default(),
                Default::default(),
                config.pivots.clone(),
                config.heuristic_to_pivots.clone(),
                config.precision,
            );

            if let Some(solution) = lsipp.solve(&config) {
                root.total_cost = solution.cost + root.total_cost - task.initial_cost;
                root.solutions.push(solution);
            } else {
                return None;
            }
        }

        Some(root)
    }

    fn enqueue(
        shared: &Shared<TS, S, A, C, DC>,
        config: &CbsConfig<TS, S, A, C, DC, H>,
        mut node: CbsNode<S, A, C, DC>,
        lsipp: &mut SafeIntervalPathPlanningWithLandmarks<
            TS,
            S,
            A,
            C,
            DC,
            ReverseResumableAStar<TS, S, A, C, DC, H>,
        >,
    ) {
        if Self::compute_conflicts(shared, config, &mut node, lsipp) {
            let mut critical = shared.critical.lock();
            critical.queue.push(Reverse(Arc::new(node)));
        }
    }

    /// Applies the Conflict-Based Search algorithm to the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - A configuration describing the problem to solve.
    pub fn solve(
        &mut self,
        config: &CbsConfig<TS, S, A, C, DC, H>,
    ) -> Option<Vec<Solution<Arc<SippState<S, C, DC>>, A, C, DC>>> {
        let start = Instant::now();
        std::thread::scope(|s| {
            for i in 0..config.n_threads {
                let shared = &self.shared;

                let mut lsipp =
                    SafeIntervalPathPlanningWithLandmarks::new(shared.transition_system.clone());

                if i == 0 {
                    Self::init(shared, config, &mut lsipp);
                }

                s.spawn(move || {
                    loop {
                        if let Some(time_limit) = &config.time_limit {
                            if start.elapsed() > *time_limit {
                                break;
                            }
                        }
                        match Self::get_workload(shared) {
                            WorkLoad::Complete => break,
                            WorkLoad::Starvation => continue,
                            WorkLoad::WorkItem { node } => {
                                Self::branch_on(shared, config, node, &mut lsipp);
                                let mut critical = shared.critical.lock();
                                critical.ongoing -= 1;
                                shared.monitor.notify_all();
                            }
                        }
                    }

                    let mut critical = shared.critical.lock();
                    critical.stats.lsipp_stats += lsipp.get_stats();
                });
            }
        });

        let mut critical = self.shared.critical.lock();
        critical.queue.clear();
        critical.stats.elapsed = start.elapsed();
        critical.stats.rra_stats = config
            .heuristic_to_pivots
            .iter()
            .map(|h| h.get_stats())
            .sum();
        critical.best.as_ref().map(|n| {
            n.get_solutions(config.n_agents)
                .iter()
                .map(|sol| (*sol).clone())
                .collect()
        })
    }

    fn get_workload(shared: &Shared<TS, S, A, C, DC>) -> WorkLoad<S, A, C, DC> {
        let mut critical = shared.critical.lock();

        while let Some(Reverse(node)) = critical.queue.pop() {
            // Check if the node is still relevant
            if let Some(best) = &critical.best {
                if node.total_cost >= best.total_cost {
                    critical.queue.clear();
                    return WorkLoad::Starvation;
                }
            }

            if node.conflicts.is_empty() {
                // No conflicts, we have a solution
                critical.best = Some(node);
            } else {
                // Node must be further expanded
                critical.ongoing += 1;
                critical.stats.expanded += 1;
                return WorkLoad::WorkItem { node };
            }
        }

        // Everything is processed
        if critical.ongoing == 0 {
            WorkLoad::Complete
        } else {
            // Wait for other thread to push new nodes
            shared.monitor.wait(&mut critical);
            WorkLoad::Starvation
        }
    }

    /// Branches on the conflict with the highest priority, creating two successor nodes (if feasible).
    fn branch_on(
        shared: &Shared<TS, S, A, C, DC>,
        config: &CbsConfig<TS, S, A, C, DC, H>,
        node: Arc<CbsNode<S, A, C, DC>>,
        lsipp: &mut SafeIntervalPathPlanningWithLandmarks<
            TS,
            S,
            A,
            C,
            DC,
            ReverseResumableAStar<TS, S, A, C, DC, H>,
        >,
    ) {
        // Find the conflict with the highest priority
        let conflict = node.conflicts.iter().min().unwrap();

        // Get the agents involved in the conflict
        let agents = T2(conflict.moves.0.agent, conflict.moves.1.agent);

        // Get the current solutions
        let current_solutions = node.get_solutions(config.n_agents);

        // Create the successor nodes, the new constraints and compute the new solutions
        let (mut successors, mut solutions, constraints) =
            Self::get_successors(shared, config, &node, conflict, lsipp);

        let mut landmark_added = false;
        for (i, (successor, solution)) in successors.drain(..).zip(solutions.drain(..)).enumerate()
        {
            if let (Some(mut successor), Some(solution)) = (successor, solution) {
                // Set the real parent of the successor node (minimal clone is used in plan_new_paths)
                successor.parent = Some(node.clone());

                // Update the total cost of the successor node
                successor.total_cost =
                    node.total_cost - (current_solutions[agents[i]].cost - solution.cost);

                // Add the solution to the successor node
                successor.solutions.push(solution);

                // Try to add a landmark to the successor node (given by the negative constraint of the other branch)
                if let Some(other_constraint) = &constraints[1 - i] {
                    if !landmark_added && other_constraint.type_ == ConstraintType::Action {
                        // Transform action constraint in two landmarks
                        let from = Constraint::new_state_constraint(
                            agents[1 - i],
                            other_constraint.state.clone(),
                            other_constraint.interval,
                        );
                        let to = Constraint::new_state_constraint(
                            agents[1 - i],
                            other_constraint.next.as_ref().unwrap().clone(),
                            Interval::new(
                                from.interval.start
                                    + (conflict.moves[1 - i].interval.end
                                        - conflict.moves[1 - i].interval.start),
                                from.interval.end
                                    + (conflict.moves[1 - i].interval.end
                                        - conflict.moves[1 - i].interval.start),
                            ),
                        );

                        if !successor.contains_landmark(T2(&from, &to)) {
                            successor.landmark = Some(T2(Arc::new(from), Arc::new(to)));
                            landmark_added = true;
                        }
                    }
                }

                if successor.conflicting_constraints(agents[i]) {
                    continue;
                }

                Self::enqueue(shared, config, successor, lsipp);
            }
        }
    }

    /// Computes the successor nodes, the new constraints and the new solutions for the given conflict.
    fn get_successors(
        shared: &Shared<TS, S, A, C, DC>,
        config: &CbsConfig<TS, S, A, C, DC, H>,
        node: &CbsNode<S, A, C, DC>,
        conflict: &Conflict<S, A, C, DC>,
        lsipp: &mut SafeIntervalPathPlanningWithLandmarks<
            TS,
            S,
            A,
            C,
            DC,
            ReverseResumableAStar<TS, S, A, C, DC, H>,
        >,
    ) -> (
        Vec<Option<CbsNode<S, A, C, DC>>>,
        Vec<Option<Solution<Arc<SippState<S, C, DC>>, A, C, DC>>>,
        A2<Option<Arc<Constraint<S, C, DC>>>>,
    ) {
        // Get the agents involved in the conflict
        let agents = T2(conflict.moves.0.agent, conflict.moves.1.agent);

        // Check if the agents are already frozen
        let frozen: T2<bool, bool> = T2(
            config.frozen.contains_key(&agents[0]),
            config.frozen.contains_key(&agents[1]),
        );

        // Get one constraint for each agent from the transition system to avoid the conflict
        let constraints = match (frozen[0], frozen[1]) {
            (true, true) => T2(None, None),
            (false, true) => T2(
                Some(Arc::new(
                    Self::get_constraints(shared, config, &conflict.moves).0,
                )),
                None,
            ),
            (true, false) => T2(
                None,
                Some(Arc::new(
                    Self::get_constraints(shared, config, &conflict.moves).1,
                )),
            ),
            (false, false) => {
                let constraints = Self::get_constraints(shared, config, &conflict.moves);
                T2(Some(Arc::new(constraints.0)), Some(Arc::new(constraints.1)))
            }
        };

        // Get a minimal clone of the current node to allow retrieving the constraints in the successor nodes
        // without needing to store the current node in an Arc
        let minimal_clone = Arc::new(node.get_minimal_clone());

        // Create a successor nodes for each new constraint
        let successors = vec![
            constraints[0]
                .as_ref()
                .map(|c| CbsNode::new(minimal_clone.clone(), c.clone())),
            constraints[1]
                .as_ref()
                .map(|c| CbsNode::new(minimal_clone, c.clone())),
        ];

        // Get all the constraints for each agent
        let constraint_sets = (
            successors[0]
                .as_ref()
                .map(|succ| succ.get_constraints(agents[0])),
            successors[1]
                .as_ref()
                .map(|succ| succ.get_constraints(agents[1])),
        );

        // Compute a new path for each agent, taking into account the new constraint
        let solutions = vec![
            constraint_sets.0.and_then(|cs| {
                lsipp.solve(&LSippConfig::new_with_pivots(
                    config.tasks[agents[0]].clone(),
                    cs.0.clone(),
                    cs.1,
                    config.pivots.clone(),
                    config.heuristic_to_pivots.clone(),
                    config.precision,
                ))
            }),
            constraint_sets.1.and_then(|cs| {
                lsipp.solve(&LSippConfig::new_with_pivots(
                    config.tasks[agents[1]].clone(),
                    cs.0,
                    cs.1,
                    config.pivots.clone(),
                    config.heuristic_to_pivots.clone(),
                    config.precision,
                ))
            }),
        ];

        (successors, solutions, constraints)
    }

    /// Returns a constraint that ensures that the first move will not collide with the second move anymore, and vice-versa.
    /// If the first move considered is stationary, i.e. from == to, then the constraint is a state constraint.
    /// Otherwise, the constraint is an action constraint.
    fn get_constraints(
        shared: &Shared<TS, S, A, C, DC>,
        config: &CbsConfig<TS, S, A, C, DC, H>,
        moves: &A2<Move<S, A, C, DC>>,
    ) -> A2<Constraint<S, C, DC>> {
        if moves[0].action.is_none() && moves[1].action.is_none() {
            let interval = Interval::new(
                moves[0].interval.start.max(moves[1].interval.start) - config.precision,
                moves[0].interval.end.min(moves[1].interval.end) + config.precision,
            );
            T2(
                Constraint::new_state_constraint(moves[0].agent, moves[0].from.clone(), interval),
                Constraint::new_state_constraint(moves[1].agent, moves[1].from.clone(), interval),
            )
        } else if moves[0].action.is_some() && moves[1].action.is_some() {
            T2(
                Constraint::new_action_constraint(
                    moves[0].agent,
                    moves[0].from.clone(),
                    moves[0].to.clone(),
                    Interval::new(
                        moves[0].interval.start - config.precision,
                        Self::earliest_non_colliding_time(shared, config, T2(&moves[0], &moves[1]))
                            + config.precision,
                    ),
                ),
                Constraint::new_action_constraint(
                    moves[1].agent,
                    moves[1].from.clone(),
                    moves[1].to.clone(),
                    Interval::new(
                        moves[1].interval.start - config.precision,
                        Self::earliest_non_colliding_time(shared, config, T2(&moves[1], &moves[0]))
                            + config.precision,
                    ),
                ),
            )
        } else {
            let swap = moves[0].action.is_none();
            let moves = if swap {
                T2(&moves[1], &moves[0])
            } else {
                T2(&moves[0], &moves[1])
            };

            let first_constraint = if moves[1].interval.end == C::max_value() {
                // The second agent stays at the conflicting position forever,
                // so the first agent will never be able to move to that position
                let interval =
                    Interval::new(moves[0].interval.start - config.precision, C::max_value());
                Constraint::new_action_constraint(
                    moves[0].agent,
                    moves[0].from.clone(),
                    moves[0].to.clone(),
                    interval,
                )
            } else {
                Constraint::new_action_constraint(
                    moves[0].agent,
                    moves[0].from.clone(),
                    moves[0].to.clone(),
                    Interval::new(
                        moves[0].interval.start - config.precision,
                        Self::earliest_non_colliding_time(shared, config, T2(moves[0], moves[1]))
                            + config.precision,
                    ),
                )
            };

            let collision_delta = if moves[1].interval.end == C::max_value() {
                let shortened_move = Move::new(
                    moves[1].agent,
                    moves[1].from.clone(),
                    moves[1].to.clone(),
                    moves[1].action,
                    Interval::new(
                        moves[1].interval.start,
                        moves[1].interval.start + (moves[0].interval.end - moves[0].interval.start),
                    ),
                );
                shortened_move.interval.end
                    - (Self::earliest_non_colliding_time(
                        shared,
                        config,
                        T2(moves[0], &shortened_move),
                    ) + config.precision)
            } else {
                moves[1].interval.end - first_constraint.interval.end
            };

            let second_constraint = Constraint::new_state_constraint(
                moves[1].agent,
                moves[1].from.clone(),
                Interval::new(
                    moves[0].interval.start + collision_delta,
                    moves[0].interval.end + collision_delta + config.precision + config.precision,
                ),
            );

            if swap {
                T2(second_constraint, first_constraint)
            } else {
                T2(first_constraint, second_constraint)
            }
        }
    }

    fn earliest_non_colliding_time(
        shared: &Shared<TS, S, A, C, DC>,
        config: &CbsConfig<TS, S, A, C, DC, H>,
        moves: A2<&Move<S, A, C, DC>>,
    ) -> C {
        let mut lo = moves[0].interval.start;
        let mut hi = moves[1].interval.end; // Starting the move after the second agent has finished its move is always okay

        let mut delayed_move = moves[0].clone();
        while hi > lo + config.precision {
            let mid = lo + (hi - lo) / 2.0;
            if mid <= lo || mid >= hi {
                // Can happen due to floating point precision
                break;
            }

            delayed_move.interval.start = mid;
            delayed_move.interval.end = mid + (moves[0].interval.end - moves[0].interval.start);

            if shared
                .transition_system
                .conflict(T2(&delayed_move, moves[1]))
            {
                lo = mid;
            } else {
                hi = mid;
            }
        }

        hi
    }

    /// Computes the conflicts between the solutions of the given node, and returns true if
    /// all of them can be avoided.
    fn compute_conflicts(
        shared: &Shared<TS, S, A, C, DC>,
        config: &CbsConfig<TS, S, A, C, DC, H>,
        node: &mut CbsNode<S, A, C, DC>,
        lsipp: &mut SafeIntervalPathPlanningWithLandmarks<
            TS,
            S,
            A,
            C,
            DC,
            ReverseResumableAStar<TS, S, A, C, DC, H>,
        >,
    ) -> bool {
        let solutions = node.get_solutions(config.n_agents);

        let mut conflicts = vec![];
        if let Some(parent) = &node.parent {
            let agent = node.constraint.as_ref().unwrap().agent;

            // Get conflicts from the parent node that do not involve the given agent
            parent
                .conflicts
                .iter()
                .filter(|c| c.moves.0.agent != agent && c.moves.1.agent != agent)
                .for_each(|c| {
                    conflicts.push(c.clone());
                });

            // Compute conflicts between the given agent and all other agents
            for other in 0..config.n_agents {
                if other == agent {
                    continue;
                }

                if let Some((conflict, avoidable)) =
                    Self::get_conflict(shared, config, node, &solutions, T2(agent, other), lsipp)
                {
                    if !avoidable {
                        return false;
                    }
                    conflicts.push(Arc::new(conflict));
                }
            }
        } else {
            // Root node, compute conflicts between each pair of solutions
            for i in 0..config.n_agents {
                for j in i + 1..config.n_agents {
                    if let Some((conflict, avoidable)) =
                        Self::get_conflict(shared, config, node, &solutions, T2(i, j), lsipp)
                    {
                        if !avoidable {
                            return false;
                        }
                        conflicts.push(Arc::new(conflict));
                    }
                }
            }
        }

        node.conflicts = conflicts;

        true
    }

    /// Returns the first conflict between the given solutions, if any, and whether it can be avoided.
    fn get_conflict(
        shared: &Shared<TS, S, A, C, DC>,
        config: &CbsConfig<TS, S, A, C, DC, H>,
        node: &CbsNode<S, A, C, DC>,
        solutions: &[&Solution<Arc<SippState<S, C, DC>>, A, C, DC>],
        agents: A2<usize>,
        lsipp: &mut SafeIntervalPathPlanningWithLandmarks<
            TS,
            S,
            A,
            C,
            DC,
            ReverseResumableAStar<TS, S, A, C, DC, H>,
        >,
    ) -> Option<(Conflict<S, A, C, DC>, bool)> {
        let mut conflict = None;

        // Iterate through both solutions and find moves overlapping in C
        let mut index = T2(0, 0);
        let mut intervals = T2(Interval::default(), Interval::default());
        loop {
            // Compute the interval of each move
            for k in 0..=1 {
                intervals[k].start = solutions[agents[k]].steps[index[k]].1;
                intervals[k].end = if index[k] < solutions[agents[k]].actions.len() {
                    solutions[agents[k]].steps[index[k] + 1].1
                } else {
                    C::max_value()
                };
            }

            // Ignore moves with no duration
            if intervals[0].start == intervals[0].end {
                index[0] += 1;
                continue;
            } else if intervals[1].start == intervals[1].end {
                index[1] += 1;
                continue;
            }

            // Check if the intervals overlap
            if intervals[0].overlaps(&intervals[1]) {
                // Check if the moves lead to a conflict
                let moves = T2(
                    Move::new(
                        agents[0],
                        solutions[agents[0]].steps[index[0]]
                            .0
                            .internal_state
                            .clone(),
                        solutions[agents[0]]
                            .steps
                            .get(index[0] + 1)
                            .map(|s| s.0.internal_state.clone())
                            .unwrap_or(
                                solutions[agents[0]].steps[index[0]]
                                    .0
                                    .internal_state
                                    .clone(),
                            ),
                        solutions[agents[0]]
                            .actions
                            .get(index[0])
                            .and_then(|a| a.action),
                        intervals[0],
                    ),
                    Move::new(
                        agents[1],
                        solutions[agents[1]].steps[index[1]]
                            .0
                            .internal_state
                            .clone(),
                        solutions[agents[1]]
                            .steps
                            .get(index[1] + 1)
                            .map(|s| s.0.internal_state.clone())
                            .unwrap_or(
                                solutions[agents[1]].steps[index[1]]
                                    .0
                                    .internal_state
                                    .clone(),
                            ),
                        solutions[agents[1]]
                            .actions
                            .get(index[1])
                            .and_then(|a| a.action),
                        intervals[1],
                    ),
                );

                if shared.transition_system.conflict(T2(&moves.0, &moves.1)) {
                    conflict = Some(Conflict::new(moves));
                    break;
                }
            }

            if index[0] < solutions[agents[0]].actions.len() && intervals[0].end <= intervals[1].end
            {
                index[0] += 1;
            } else if index[1] < solutions[agents[1]].actions.len() {
                index[1] += 1;
            } else {
                break;
            }
        }

        if let Some(mut conflict) = conflict {
            // Determine conflict type by trying to avoid it
            let (_, new_solutions, _) =
                Self::get_successors(shared, config, node, &conflict, lsipp);

            if let (None, None) = (&new_solutions[0], &new_solutions[1]) {
                return Some((conflict, false));
            } else if let (Some(solution), None) = (&new_solutions[0], &new_solutions[1]) {
                conflict.overcost = solution.cost - solutions[agents[0]].cost;
                if config.frozen.contains_key(&agents[1]) {
                    conflict.type_ = ConflictType::Frozen;
                } else {
                    conflict.type_ = ConflictType::Cardinal;
                }
            } else if let (None, Some(solution)) = (&new_solutions[0], &new_solutions[1]) {
                conflict.overcost = solution.cost - solutions[agents[1]].cost;
                if config.frozen.contains_key(&agents[0]) {
                    conflict.type_ = ConflictType::Frozen;
                } else {
                    conflict.type_ = ConflictType::Cardinal;
                }
            } else if let (Some(solution1), Some(solution2)) =
                (&new_solutions[0], &new_solutions[1])
            {
                let overcost1 = solution1.cost - solutions[agents[0]].cost;
                let overcost2 = solution2.cost - solutions[agents[1]].cost;
                if overcost1 > DC::default() && overcost2 > DC::default() {
                    conflict.overcost = overcost1.min(overcost2);
                    conflict.type_ = ConflictType::Cardinal;
                } else if overcost1 > DC::default() || overcost2 > DC::default() {
                    conflict.overcost = overcost1.max(overcost2);
                    conflict.type_ = ConflictType::SemiCardinal;
                } else {
                    conflict.type_ = ConflictType::NonCardinal;
                }
            }

            return Some((conflict, true));
        }

        None
    }

    /// Returns the statistics of the search algorithm.
    pub fn get_stats(&mut self) -> CbsStats {
        self.shared.critical.lock().stats
    }
}

/// Input configuration for the Conflict-Based Search algorithm.
pub struct CbsConfig<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: Debug + State + Eq + Hash + Clone,
    C: Eq
        + PartialOrd
        + Ord
        + Add<DC, Output = C>
        + Sub<C, Output = DC>
        + Copy
        + Default
        + LimitValues,
    DC: Copy,
    H: Heuristic<TS, S, A, C, DC>,
{
    /// The number of agents to consider.
    pub n_agents: usize,
    /// The task that each agent needs to perform.
    pub tasks: Vec<Arc<Task<S, C>>>,
    /// A set of frozen agents and their already planned paths.
    frozen: FxHashMap<usize, Solution<Arc<SippState<S, C, DC>>, A, C, DC>>,
    /// A set of pivot states.
    pivots: Arc<Vec<S>>,
    /// A set of heuristics to those pivot states.
    heuristic_to_pivots: Arc<Vec<Arc<ReverseResumableAStar<TS, S, A, C, DC, H>>>>,
    /// The precision to use when computing collisions and constraints.
    precision: DC,
    /// The number of threads to use.
    n_threads: usize,
    /// The time limit for the search.
    pub time_limit: Option<Duration>,
    _phantom: PhantomData<(TS, A)>,
}

impl<TS, S, A, C, DC, H> CbsConfig<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: Debug + State + Eq + Hash + Clone,
    C: Eq
        + PartialOrd
        + Ord
        + Add<DC, Output = C>
        + Sub<C, Output = DC>
        + Copy
        + Default
        + LimitValues,
    DC: Copy,
    H: Heuristic<TS, S, A, C, DC>,
{
    /// Creates a new configuration for the Conflict-Based Search algorithm.
    ///
    /// # Arguments
    ///
    /// * `transition_system` - The transition system in which the agents navigate
    /// * `tasks` - The task that each agent needs to perform
    /// * `precision` - The precision to use when computing collisions and constraints
    /// * `n_threads` - The number of threads to use
    /// * `time_limit` - The time limit for the search
    pub fn new(
        transition_system: Arc<TS>,
        tasks: Vec<Arc<Task<S, C>>>,
        precision: DC,
        n_threads: usize,
        time_limit: Option<Duration>,
    ) -> Self
    where
        H: HeuristicBuilder<TS, S, A, C, DC>,
    {
        let pivots = Arc::new(tasks.iter().map(|t| t.goal_state.clone()).collect());
        let heuristic_to_pivots = Arc::new(
            tasks
                .iter()
                .map(|t| {
                    Arc::new(ReverseResumableAStar::new(
                        transition_system.clone(),
                        t.clone(),
                        H::build(transition_system.clone(), Arc::new(t.reverse())),
                    ))
                })
                .collect(),
        );
        Self {
            n_agents: tasks.len(),
            tasks,
            frozen: FxHashMap::default(),
            pivots,
            heuristic_to_pivots,
            precision,
            n_threads,
            time_limit,
            _phantom: PhantomData,
        }
    }

    /// Creates a new configuration for the Conflict-Based Search algorithm.
    ///
    /// # Arguments
    ///
    /// * `tasks` - The task that each agent needs to perform
    /// * `pivots` - A set of pivot states for which heuristics have been initialized
    /// * `heuristic_to_pivots` - A set of heuristics to the pivot states
    /// * `precision` - The precision to use when computing collisions and constraints
    /// * `n_threads` - The number of threads to use
    /// * `time_limit` - The time limit for the search
    pub fn new_with_pivots(
        tasks: Vec<Arc<Task<S, C>>>,
        pivots: Arc<Vec<S>>,
        heuristic_to_pivots: Arc<Vec<Arc<ReverseResumableAStar<TS, S, A, C, DC, H>>>>,
        precision: DC,
        n_threads: usize,
        time_limit: Option<Duration>,
    ) -> Self {
        Self {
            n_agents: tasks.len(),
            tasks,
            frozen: FxHashMap::default(),
            pivots,
            heuristic_to_pivots,
            precision,
            n_threads,
            time_limit,
            _phantom: PhantomData,
        }
    }

    /// Adds a frozen agent and its already planned path to the configuration.
    ///
    /// # Arguments
    ///
    /// * `agent` - The new frozen agent
    /// * `solution` - The already planned path for the agent
    pub fn add_frozen(
        &mut self,
        agent: usize,
        solution: Solution<Arc<SippState<S, C, DC>>, A, C, DC>,
    ) {
        self.frozen.insert(agent, solution);
    }
}

/// A node in the Conflict-Based Search tree.
#[derive(Debug)]
struct CbsNode<S, A, C, DC>
where
    S: Debug + State + Eq + Hash + Clone,
    C: Debug + Ord + Default + LimitValues + Copy + Sub<C, Output = DC>,
    DC: PartialEq + Eq + PartialOrd + Ord + Default + Copy,
{
    pub total_cost: DC,
    parent: Option<Arc<Self>>,
    solutions: Vec<Solution<Arc<SippState<S, C, DC>>, A, C, DC>>,
    pub conflicts: Vec<Arc<Conflict<S, A, C, DC>>>,
    constraint: Option<Arc<Constraint<S, C, DC>>>,
    landmark: Option<A2<Arc<Constraint<S, C, DC>>>>,
}

impl<S, A, C, DC> Default for CbsNode<S, A, C, DC>
where
    S: Debug + State + Eq + Hash + Clone,
    C: Debug + Ord + Default + LimitValues + Copy + Sub<C, Output = DC>,
    DC: PartialEq + Eq + PartialOrd + Ord + Default + Copy,
{
    fn default() -> Self {
        Self {
            total_cost: DC::default(),
            parent: None,
            solutions: vec![],
            conflicts: vec![],
            constraint: None,
            landmark: None,
        }
    }
}

impl<S, A, C, DC> CbsNode<S, A, C, DC>
where
    S: Debug + State + Eq + Hash + Clone,
    C: Debug + Ord + Default + LimitValues + Copy + Sub<C, Output = DC>,
    DC: PartialEq + Eq + PartialOrd + Ord + Default + Copy,
{
    pub fn new(parent: Arc<Self>, constraint: Arc<Constraint<S, C, DC>>) -> Self {
        Self {
            total_cost: parent.total_cost,
            parent: Some(parent),
            solutions: vec![],
            conflicts: vec![],
            constraint: Some(constraint),
            landmark: None,
        }
    }

    pub fn get_minimal_clone(&self) -> Self {
        Self {
            total_cost: self.total_cost,
            parent: self.parent.clone(),
            solutions: vec![],
            conflicts: vec![],
            constraint: self.constraint.clone(),
            landmark: self.landmark.clone(),
        }
    }

    pub fn get_constraints(
        &self,
        agent: usize,
    ) -> (Arc<ConstraintSet<S, C, DC>>, LandmarkSet<S, C, DC>) {
        let mut constraints = ConstraintSet::default();
        let mut landmarks = LandmarkSet::default();

        let mut current = self;
        loop {
            if let Some(constraint) = &current.constraint {
                if constraint.agent == agent {
                    constraints.add(constraint);
                }
            }
            if let Some(T2(from, to)) = &current.landmark {
                if from.agent == agent {
                    landmarks.push(from.clone());
                    landmarks.push(to.clone());
                }
            }

            if let Some(parent) = &current.parent {
                current = parent;
            } else {
                break;
            }
        }

        landmarks.sort_unstable();
        constraints.unify();

        (Arc::new(constraints), landmarks)
    }

    pub fn get_constraints_alt(
        &self,
        agent: usize,
    ) -> (ConstraintSet<S, C, DC>, Vec<A2<Arc<Constraint<S, C, DC>>>>) {
        let mut constraints = ConstraintSet::default();
        let mut landmarks = vec![];

        let mut current = self;
        loop {
            if let Some(constraint) = &current.constraint {
                if constraint.agent == agent {
                    constraints.add(constraint);
                }
            }
            if let Some(T2(from, to)) = &current.landmark {
                if from.agent == agent {
                    landmarks.push(T2(from.clone(), to.clone()));
                }
            }

            if let Some(parent) = &current.parent {
                current = parent;
            } else {
                break;
            }
        }

        constraints.unify();

        (constraints, landmarks)
    }

    pub fn get_solutions(
        &self,
        n_agents: usize,
    ) -> Vec<&Solution<Arc<SippState<S, C, DC>>, A, C, DC>> {
        let mut found = 0;
        let mut solutions = vec![None; n_agents];

        let mut current = self;
        loop {
            if let Some(constraint) = &current.constraint {
                let agent = constraint.agent;
                if solutions[agent].is_none() {
                    solutions[agent] = Some(&current.solutions[0]);
                    found += 1;
                }
            } else {
                for (agent, solution) in solutions.iter_mut().enumerate() {
                    if solution.is_none() {
                        *solution = Some(&current.solutions[agent]);
                        found += 1;
                    }
                }
            }

            if found == n_agents {
                break;
            }

            if let Some(parent) = &current.parent {
                current = parent;
            } else {
                break;
            }
        }

        solutions.into_iter().map(|s| s.unwrap()).collect()
    }

    fn contains_landmark(
        &self,
        landmark: T2<&Constraint<S, C, DC>, &Constraint<S, C, DC>>,
    ) -> bool {
        let mut current = self;

        loop {
            if let Some(T2(from, to)) = &current.landmark {
                if from.state == landmark.0.state
                    && to.state == landmark.1.state
                    && (from.interval.contains(&landmark.0.interval)
                        || landmark.0.interval.contains(&from.interval))
                    && (to.interval.contains(&landmark.1.interval)
                        || landmark.1.interval.contains(&to.interval))
                {
                    return true;
                }
            }

            if let Some(parent) = current.parent.as_ref() {
                current = parent;
            } else {
                break;
            }
        }

        false
    }

    fn conflicting_constraints(&self, agent: usize) -> bool {
        let (constraints, landmarks) = self.get_constraints_alt(agent);

        for landmark in landmarks.iter() {
            if let Some(constraint_set) =
                constraints.get_action_constraints(&landmark.0.state, &landmark.1.state)
            {
                for constraint in constraint_set {
                    if constraint.interval.contains(&landmark.0.interval) {
                        return true;
                    }
                }
            }
            if let Some(constraint_set) = constraints.get_state_constraints(&landmark.0.state) {
                for constraint in constraint_set {
                    if constraint.interval.contains(&landmark.0.interval) {
                        return true;
                    }
                }
            }
            if let Some(constraint_set) = constraints.get_state_constraints(&landmark.1.state) {
                for constraint in constraint_set {
                    if constraint.interval.contains(&landmark.1.interval) {
                        return true;
                    }
                }
            }
        }

        false
    }
}

// TODO: add high-level heuristic to ordering
impl<S, A, C, DC> PartialEq for CbsNode<S, A, C, DC>
where
    S: Debug + State + Eq + Hash + Clone,
    C: Debug + Ord + Default + LimitValues + Copy + Sub<C, Output = DC>,
    DC: PartialEq + Eq + PartialOrd + Ord + Default + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.total_cost == other.total_cost
    }
}

impl<S, A, C, DC> Eq for CbsNode<S, A, C, DC>
where
    S: Debug + State + Eq + Hash + Clone,
    C: Debug + Ord + Default + LimitValues + Copy + Sub<C, Output = DC>,
    DC: PartialEq + Eq + PartialOrd + Ord + Default + Copy,
{
}

impl<S, A, C, DC> PartialOrd for CbsNode<S, A, C, DC>
where
    S: Debug + State + Eq + Hash + Clone,
    C: Debug + Ord + Default + LimitValues + Copy + Sub<C, Output = DC>,
    DC: PartialEq + Eq + PartialOrd + Ord + Default + Copy,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<S, A, C, DC> Ord for CbsNode<S, A, C, DC>
where
    S: Debug + State + Eq + Hash + Clone,
    C: Debug + Ord + Default + LimitValues + Copy + Sub<C, Output = DC>,
    DC: PartialEq + Eq + PartialOrd + Ord + Default + Copy,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_cost.cmp(&other.total_cost)
    }
}

enum WorkLoad<S, A, C, DC>
where
    S: Debug + State + Eq + Hash + Clone,
    C: Debug + Default + Copy + Ord + LimitValues + Sub<C, Output = DC>,
    DC: Default + Copy + Ord,
{
    Complete,
    Starvation,
    WorkItem { node: Arc<CbsNode<S, A, C, DC>> },
}

/// Statistics of the Conflict-Based Search algorithm.
#[derive(Debug, Default, Clone, Copy)]
pub struct CbsStats {
    /// The number of CBS nodes expanded.
    pub expanded: usize,
    /// The time elapsed during the search.
    pub elapsed: Duration,
    /// Statistics of the low-level search algorithm.
    pub lsipp_stats: LSippStats,
    /// Statistics of the RRA* algorithm used as a heuristic.
    pub rra_stats: RraStats,
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ordered_float::OrderedFloat;

    use crate::{
        simple_graph, GraphEdgeId, GraphNodeId, SimpleHeuristic, SimpleState, SimpleWorld, Task,
    };

    use super::{CbsConfig, ConflictBasedSearch};

    #[test]
    fn test_simple() {
        let size = 10;
        let graph = simple_graph(size);
        let transition_system = Arc::new(SimpleWorld::new(graph, 0.4));

        let tasks = vec![
            Arc::new(Task::new(
                SimpleState(GraphNodeId(0)),
                SimpleState(GraphNodeId(9)),
                OrderedFloat(0.0),
            )),
            Arc::new(Task::new(
                SimpleState(GraphNodeId(9)),
                SimpleState(GraphNodeId(0)),
                OrderedFloat(0.0),
            )),
        ];

        let config: CbsConfig<
            SimpleWorld,
            SimpleState,
            GraphEdgeId,
            OrderedFloat<f64>,
            OrderedFloat<f64>,
            SimpleHeuristic,
        > = CbsConfig::new(
            transition_system.clone(),
            tasks,
            OrderedFloat(1e-6),
            1,
            None,
        );

        let mut solver = ConflictBasedSearch::new(transition_system.clone());

        let solutions = solver.solve(&config).unwrap();

        assert_eq!(
            solutions
                .iter()
                .map(|sol| sol.cost)
                .sum::<OrderedFloat<f64>>(),
            OrderedFloat(20.0)
        );
    }

    #[test]
    fn test_frozen() {
        let size = 10;
        let graph = simple_graph(size);
        let transition_system = Arc::new(SimpleWorld::new(graph, 0.4));

        let mut tasks = vec![Arc::new(Task::new(
            SimpleState(GraphNodeId(0)),
            SimpleState(GraphNodeId(9)),
            OrderedFloat(0.0),
        ))];

        let config: CbsConfig<
            SimpleWorld,
            SimpleState,
            GraphEdgeId,
            OrderedFloat<f64>,
            OrderedFloat<f64>,
            SimpleHeuristic,
        > = CbsConfig::new(
            transition_system.clone(),
            tasks.clone(),
            OrderedFloat(1e-6),
            1,
            None,
        );

        let mut solver = ConflictBasedSearch::new(transition_system.clone());

        let mut solutions = solver.solve(&config).unwrap();

        assert_eq!(solutions[0].cost, OrderedFloat(9.0));

        tasks.push(Arc::new(Task::new(
            SimpleState(GraphNodeId(9)),
            SimpleState(GraphNodeId(0)),
            OrderedFloat(0.0),
        )));

        let mut config = CbsConfig::new(
            transition_system.clone(),
            tasks,
            OrderedFloat(1e-6),
            1,
            None,
        );
        config.add_frozen(0, solutions.pop().unwrap());

        let solutions = solver.solve(&config).unwrap();

        assert_eq!(solutions[0].cost, OrderedFloat(9.0));
        assert_eq!(solutions[1].cost, OrderedFloat(11.0));

        assert_eq!(solutions[0].steps, config.frozen[&0].steps);
    }
}
