use std::{
    cmp::Reverse,
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        BTreeSet, BinaryHeap,
    },
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
    ops::{Add, AddAssign, Sub},
    sync::Arc,
    vec,
};

use fxhash::{FxHashMap, FxHashSet};

use crate::{
    Action, ConstraintSet, Heuristic, Interval, LimitValues, SearchNode, Solution, State, Task,
    TransitionSystem,
};

/// Implementation of the Safe Interval Path Planning algorithm that computes
/// the optimal sequence of actions to complete a given task in a given transition system,
/// while avoiding conflicts with other agents in the same environment.
pub struct SafeIntervalPathPlanning<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: Debug + Hash + Eq + Clone,
    A: Copy,
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
        + LimitValues,
    DC: Debug + Hash + Copy + Default + PartialEq + Eq + PartialOrd + Ord,
    H: Heuristic<TS, S, A, C, DC>,
{
    transition_system: Arc<TS>,
    queue: BinaryHeap<Reverse<SearchNode<SippState<S, C, DC>, C, DC>>>,
    distance: FxHashMap<Arc<SippState<S, C, DC>>, C>,
    closed: FxHashSet<Arc<SippState<S, C, DC>>>,
    parent: FxHashMap<Arc<SippState<S, C, DC>>, (Action<A, DC>, Arc<SippState<S, C, DC>>)>,
    goal_intervals: BTreeSet<Interval<C, DC>>,
    goal_horizon: C,
    safe_intervals: Vec<Interval<C, DC>>,
    stats: SippStats,
    _phantom: PhantomData<(A, H)>,
}

impl<TS, S, A, C, DC, H> SafeIntervalPathPlanning<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: State + Debug + Hash + Eq + Clone,
    A: Copy,
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
        + LimitValues,
    DC: Debug + Hash + Copy + Default + PartialEq + Eq + PartialOrd + Ord,
    H: Heuristic<TS, S, A, C, DC>,
{
    /// Creates a new instance of the Safe Interval Path Planning algorithm.
    pub fn new(transition_system: Arc<TS>) -> Self {
        SafeIntervalPathPlanning {
            transition_system,
            queue: BinaryHeap::new(),
            distance: FxHashMap::default(),
            closed: FxHashSet::default(),
            parent: FxHashMap::default(),
            goal_intervals: BTreeSet::default(),
            goal_horizon: C::max_value(),
            safe_intervals: vec![],
            stats: SippStats::default(),
            _phantom: PhantomData,
        }
    }

    /// Transforms the configuration into a generalized configuration, if any
    /// safe intervals exist for the initial state.
    pub fn to_generalized(
        &mut self,
        config: &SippConfig<TS, S, A, C, DC, H>,
    ) -> Option<GeneralizedSippConfig<TS, S, A, C, DC, H>> {
        let initial_time = config.task.initial_cost;

        // Find the safe interval in which the initial time is contained
        Self::get_safe_intervals(
            &config.constraints,
            &config.task.initial_state,
            &Interval::new(initial_time, initial_time),
            config.precision,
            &mut self.safe_intervals,
        );

        if self.safe_intervals.is_empty() {
            return None;
        }

        let initial_state = Arc::new(SippState {
            safe_interval: self.safe_intervals.pop().unwrap(),
            internal_state: config.task.initial_state.clone(),
        });

        let sipp_task = SippTask::new(
            vec![initial_time],
            vec![initial_state],
            config.task.goal_state.clone(),
            config.interval,
            config.task.clone(),
        );

        Some(GeneralizedSippConfig::new(
            sipp_task,
            config.constraints.clone(),
            config.heuristic.clone(),
            config.precision,
        ))
    }

    /// Attempts to solve the given configuration, and returns the optimal solution if any.
    pub fn solve(
        &mut self,
        config: &SippConfig<TS, S, A, C, DC, H>,
    ) -> Option<Solution<Arc<SippState<S, C, DC>>, A, C, DC>> {
        self.to_generalized(config)
            .and_then(|config| self.solve_generalized(&config).pop())
    }
    /// Attempts to solve the given generalized configuration, and returns the optimal solution if any.
    pub fn solve_generalized(
        &mut self,
        config: &GeneralizedSippConfig<TS, S, A, C, DC, H>,
    ) -> Vec<Solution<Arc<SippState<S, C, DC>>, A, C, DC>> {
        if !self.init(config) {
            return vec![];
        }

        self.find_paths(config)
            .iter()
            .map(|g| self.get_solution(config, g))
            .collect()
    }

    /// Initializes the search algorithm by clearing the data structures
    /// and enqueueing the initial states.
    fn init(&mut self, config: &GeneralizedSippConfig<TS, S, A, C, DC, H>) -> bool {
        self.queue.clear();
        self.distance.clear();
        self.closed.clear();
        self.parent.clear();
        self.goal_intervals.clear();
        self.goal_horizon = C::min_value();

        // Enqueue the initial nodes
        for (initial_time, initial_state) in config
            .task
            .initial_times
            .iter()
            .zip(config.task.initial_states.iter())
        {
            let initial_node = SearchNode {
                state: initial_state.clone(),
                cost: *initial_time,
                heuristic: DC::default(),
            };

            self.distance
                .insert(initial_node.state.clone(), initial_node.cost);
            self.queue.push(Reverse(initial_node));
        }

        // Find the safe intervals at the goal state
        Self::get_safe_intervals(
            &config.constraints,
            &config.task.goal_state,
            &config.task.goal_interval,
            config.precision,
            &mut self.safe_intervals,
        );
        if self.safe_intervals.is_empty() {
            return false;
        }
        self.safe_intervals.drain(..).for_each(|i| {
            self.goal_horizon = self.goal_horizon.max(i.end);
            self.goal_intervals.insert(i);
        });

        self.stats.searches += 1;

        true
    }

    /// Finds all shortest paths from the initial states to any reachable safe interval
    /// at the goal state.
    fn find_paths(
        &mut self,
        config: &GeneralizedSippConfig<TS, S, A, C, DC, H>,
    ) -> Vec<SearchNode<SippState<S, C, DC>, C, DC>> {
        let mut goals = vec![];

        while let Some(Reverse(current)) = self.queue.pop() {
            if current.cost > self.distance[current.state.as_ref()] {
                // A better path has already been found
                continue;
            }

            if current.cost + current.heuristic >= self.goal_horizon {
                // The remaining safe intervals at the goal state are not reachable in time
                continue;
            }

            if config.task.is_goal(&current)
                && self.goal_intervals.remove(&current.state.safe_interval)
            {
                // A path to the goal has been found
                goals.push(current.clone());
                if self.goal_intervals.is_empty() {
                    break;
                }
                self.goal_horizon = self.goal_intervals.last().unwrap().end;
            }

            // Expand the current state and enqueue its successors
            self.expand(config, &current);

            self.closed.insert(current.state.clone()); // Mark the state as closed because it has been expanded
            self.stats.expanded += 1;
        }

        goals
    }

    /// Generates the reachable successors of the given search node.
    fn expand(
        &mut self,
        config: &GeneralizedSippConfig<TS, S, A, C, DC, H>,
        current: &SearchNode<SippState<S, C, DC>, C, DC>,
    ) {
        for action in self
            .transition_system
            .actions_from(&current.state.internal_state)
        {
            let successor_state = self
                .transition_system
                .transition(&current.state.internal_state, action);
            let transition_cost = self
                .transition_system
                .transition_cost(&current.state.internal_state, action);

            let heuristic = config.heuristic.get_heuristic(&successor_state);
            if heuristic.is_none() {
                continue; // Goal state is not reachable from this state
            }
            let heuristic = heuristic.unwrap();

            if current.cost + transition_cost + heuristic >= self.goal_horizon {
                // The remaining safe intervals at the goal state are not reachable in time
                continue;
            }

            let action_constraints = config
                .constraints
                .get_action_constraints(&current.state.internal_state, &successor_state);

            // Try to reach any of the safe intervals of the destination state
            // and add the corresponding successors to the queue if a better path has been found
            Self::get_safe_intervals(
                &config.constraints,
                &successor_state,
                &Interval::new(current.cost + transition_cost, C::max_value()),
                config.precision,
                &mut self.safe_intervals,
            );
            for safe_interval in self.safe_intervals.drain(..) {
                let mut successor_cost = current.cost + transition_cost;

                if successor_cost + config.precision > safe_interval.end {
                    // Cannot reach this safe interval in time
                    continue;
                }

                if successor_cost < safe_interval.start {
                    // Would arrive too early
                    if !self
                        .transition_system
                        .can_wait_at(&current.state.internal_state)
                    {
                        // Cannot wait at the current state
                        continue;
                    }
                    successor_cost = safe_interval.start; // Try to depart later to arrive at the right time
                    if successor_cost - transition_cost + config.precision
                        > current.state.safe_interval.end
                    {
                        // Cannot depart that late from the current safe interval
                        continue;
                    }
                }

                // Check collision along the action
                if let Some(action_constraints) = action_constraints {
                    let mut i = action_constraints
                        .partition_point(|c| c.interval.end < successor_cost - transition_cost);

                    let mut collision = false;
                    while i < action_constraints.len()
                        && successor_cost - transition_cost >= action_constraints[i].interval.start
                        && successor_cost - transition_cost <= action_constraints[i].interval.end
                    {
                        let collision_interval = &action_constraints[i].interval;

                        if successor_cost - transition_cost + config.precision
                            >= collision_interval.start
                        {
                            // Collision detected
                            if !self
                                .transition_system
                                .can_wait_at(&current.state.internal_state)
                            {
                                // Cannot wait at the current state
                                collision = true;
                                break;
                            }
                            successor_cost = collision_interval.end + transition_cost; // Try to depart later

                            if successor_cost - transition_cost + config.precision
                                > current.state.safe_interval.end
                                || successor_cost + config.precision > safe_interval.end
                            {
                                // Cannot depart that late from the current safe interval
                                collision = true;
                                break;
                            }
                        }

                        i += 1;
                    }

                    if collision {
                        continue;
                    }
                }

                if successor_cost + heuristic >= self.goal_horizon {
                    // The remaining safe intervals at the goal state are not reachable in time
                    continue;
                }

                let successor_state = Arc::new(SippState {
                    safe_interval,
                    internal_state: successor_state.clone(),
                });

                let successor = SearchNode {
                    state: successor_state,
                    cost: successor_cost,
                    heuristic,
                };

                let improved = match self.distance.entry(successor.state.clone()) {
                    Occupied(mut e) => {
                        if successor_cost < *e.get() {
                            *e.get_mut() = successor_cost;
                            true
                        } else {
                            false
                        }
                    }
                    Vacant(e) => {
                        e.insert(successor_cost);
                        true
                    }
                };

                if improved {
                    self.parent.insert(
                        successor.state.clone(),
                        (Action::new(*action, transition_cost), current.state.clone()),
                    );
                    self.queue.push(Reverse(successor))
                }
            }
        }
    }

    /// Computes the safe intervals for the given state, given a set of constraints,
    /// and that overlap with the given interval.
    fn get_safe_intervals(
        constraints: &Arc<ConstraintSet<S, C, DC>>,
        state: &S,
        range: &Interval<C, DC>,
        precision: DC,
        safe_intervals: &mut Vec<Interval<C, DC>>,
    ) {
        if let Some(state_constraints) = constraints.get_state_constraints(state) {
            let mut current = Interval::default();
            for constraint in state_constraints.iter() {
                current.end = constraint.interval.start;
                if current.start + precision < current.end && current.overlaps(range) {
                    safe_intervals.push(current);
                }
                current.start = constraint.interval.end;
            }
            current.end = Interval::default().end;
            if current.start + precision < current.end && current.overlaps(range) {
                safe_intervals.push(current);
            }
        } else {
            safe_intervals.push(Interval::default());
        }
    }

    /// Reconstructs the solution from the given goal search node.
    fn get_solution(
        &self,
        config: &GeneralizedSippConfig<TS, S, A, C, DC, H>,
        goal: &SearchNode<SippState<S, C, DC>, C, DC>,
    ) -> Solution<Arc<SippState<S, C, DC>>, A, C, DC> {
        let mut solution = Solution::default();
        let mut current = goal.state.clone();

        // Check if we need to wait for the landmark to begin
        if self.distance[&current] < config.task.goal_interval.start {
            solution
                .steps
                .push((current.clone(), config.task.goal_interval.start));
            solution.actions.push(Action::wait(
                config.task.goal_interval.start - self.distance[&current],
            ));
        }

        solution
            .steps
            .push((current.clone(), self.distance[&current]));

        while let Some((action, parent)) = self.parent.get(&current) {
            if self.distance[parent] + action.cost + config.precision < self.distance[&current] {
                solution
                    .steps
                    .push((parent.clone(), self.distance[&current] - action.cost));
                solution.actions.push(*action);

                // Insert wait action
                solution.steps.push((parent.clone(), self.distance[parent]));
                solution.actions.push(Action::wait(action.cost));
            } else {
                solution.steps.push((parent.clone(), self.distance[parent]));
                solution.actions.push(*action);
            }

            current = parent.clone();
        }

        solution.steps.reverse();
        solution.actions.reverse();

        solution.cost = solution.steps.last().unwrap().1;

        solution
    }

    /// Returns the statistics of the search algorithm.
    pub fn get_stats(&self) -> SippStats {
        self.stats
    }
}

/// Input configuration for the Safe Interval Path Planning algorithm.
pub struct SippConfig<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: State + Debug + Hash + Eq + Clone,
    A: Copy,
    C: Eq
        + PartialOrd
        + Ord
        + Add<DC, Output = C>
        + Sub<C, Output = DC>
        + Copy
        + Default
        + LimitValues,
    DC: PartialEq + Eq + PartialOrd + Ord + Copy,
    H: Heuristic<TS, S, A, C, DC>,
{
    task: Arc<Task<S, C>>,
    interval: Interval<C, DC>,
    constraints: Arc<ConstraintSet<S, C, DC>>,
    heuristic: Arc<H>,
    precision: DC,
    _phantom: PhantomData<(TS, S, A)>,
}

impl<TS, S, A, C, DC, H> SippConfig<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: State + Debug + Hash + Eq + Clone,
    A: Copy,
    C: Eq
        + PartialOrd
        + Ord
        + Add<DC, Output = C>
        + Sub<C, Output = DC>
        + Copy
        + Default
        + LimitValues,
    DC: PartialEq + Eq + PartialOrd + Ord + Copy,
    H: Heuristic<TS, S, A, C, DC>,
{
    pub fn new(
        task: Arc<Task<S, C>>,
        interval: Interval<C, DC>,
        constraints: Arc<ConstraintSet<S, C, DC>>,
        heuristic: Arc<H>,
        precision: DC,
    ) -> Self {
        SippConfig {
            task,
            interval,
            constraints,
            heuristic,
            precision,
            _phantom: PhantomData,
        }
    }
}

/// Input configuration for the Generalized Safe Interval Path Planning algorithm.
pub struct GeneralizedSippConfig<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: State + Debug + Hash + Eq + Clone,
    A: Copy,
    C: Eq
        + PartialOrd
        + Ord
        + Add<DC, Output = C>
        + Sub<C, Output = DC>
        + Copy
        + Default
        + LimitValues,
    DC: Copy + PartialEq + Eq + PartialOrd + Ord,
    H: Heuristic<TS, S, A, C, DC>,
{
    task: SippTask<S, C, DC>,
    constraints: Arc<ConstraintSet<S, C, DC>>,
    heuristic: Arc<H>,
    precision: DC,
    _phantom: PhantomData<(TS, S, A)>,
}

impl<TS, S, A, C, DC, H> GeneralizedSippConfig<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: State + Debug + Hash + Eq + Clone,
    A: Copy,
    C: Eq
        + PartialOrd
        + Ord
        + Add<DC, Output = C>
        + Sub<C, Output = DC>
        + Copy
        + Default
        + LimitValues,
    DC: Copy + PartialEq + Eq + PartialOrd + Ord,
    H: Heuristic<TS, S, A, C, DC>,
{
    pub fn new(
        task: SippTask<S, C, DC>,
        constraints: Arc<ConstraintSet<S, C, DC>>,
        heuristic: Arc<H>,
        precision: DC,
    ) -> Self {
        GeneralizedSippConfig {
            task,
            constraints,
            heuristic,
            precision,
            _phantom: PhantomData,
        }
    }
}

/// State wrapper for the Safe Interval Path Planning algorithm that extends
/// a given state definition with a safe interval.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct SippState<S, C, DC>
where
    S: Debug + Eq,
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues + Sub<C, Output = DC> + Copy,
{
    pub safe_interval: Interval<C, DC>,
    pub internal_state: S,
}

/// Task wrapper for the Safe Interval Path Planning algorithm that extends
/// a given task definition with all SIPP states that correspong to it.
pub struct SippTask<S, C, DC>
where
    S: State + Debug + Hash + Eq + Clone,
    C: Eq
        + PartialOrd
        + Ord
        + Add<DC, Output = C>
        + Sub<C, Output = DC>
        + Copy
        + Default
        + LimitValues,
    DC: Copy,
{
    initial_times: Vec<C>,
    initial_states: Vec<Arc<SippState<S, C, DC>>>,
    goal_state: S,
    goal_interval: Interval<C, DC>,
    internal_task: Arc<Task<S, C>>,
    _phantom: PhantomData<DC>,
}

impl<S, C, DC> SippTask<S, C, DC>
where
    S: State + Debug + Hash + Eq + Clone,
    C: Eq
        + PartialOrd
        + Ord
        + Add<DC, Output = C>
        + Sub<C, Output = DC>
        + Copy
        + Default
        + LimitValues,
    DC: Copy,
{
    pub fn new(
        initial_times: Vec<C>,
        initial_states: Vec<Arc<SippState<S, C, DC>>>,
        goal_state: S,
        goal_interval: Interval<C, DC>,
        internal_task: Arc<Task<S, C>>,
    ) -> Self {
        SippTask {
            initial_times,
            initial_states,
            goal_state,
            goal_interval,
            internal_task,
            _phantom: PhantomData,
        }
    }

    fn is_goal(&self, state: &SearchNode<SippState<S, C, DC>, C, DC>) -> bool {
        self.internal_task
            .is_goal_state(&state.state.internal_state)
    }
}

/// Statistics of the Safe Interval Path Planning algorithm.
#[derive(Debug, Clone, Copy, Default)]
pub struct SippStats {
    pub searches: usize,
    pub expanded: usize,
}

impl AddAssign for SippStats {
    fn add_assign(&mut self, rhs: Self) {
        self.searches += rhs.searches;
        self.expanded += rhs.expanded;
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ordered_float::OrderedFloat;

    use crate::{
        search::sipp::sipp::SippConfig, Constraint, ConstraintSet, Graph, GraphEdgeId, GraphNodeId,
        Interval, MyTime, ReverseResumableAStar, SimpleEdgeData, SimpleHeuristic, SimpleNodeData,
        SimpleState, SimpleWorld, Task,
    };

    use super::SafeIntervalPathPlanning;

    fn simple_graph(size: usize) -> Arc<Graph<SimpleNodeData, SimpleEdgeData>> {
        let mut graph = Graph::new();
        for x in 0..size {
            for y in 0..size {
                graph.add_node((x as f64, y as f64));
            }
        }
        for x in 0..size {
            for y in 0..size {
                let node_id = GraphNodeId(x + y * size);
                if x > 0 {
                    graph.add_edge(node_id, GraphNodeId(x - 1 + y * size), 1.0);
                }
                if y > 0 {
                    graph.add_edge(node_id, GraphNodeId(x + (y - 1) * size), 1.0);
                }
                if x < size - 1 {
                    graph.add_edge(node_id, GraphNodeId(x + 1 + y * size), 1.0);
                }
                if y < size - 1 {
                    graph.add_edge(node_id, GraphNodeId(x + (y + 1) * size), 1.0);
                }
            }
        }
        Arc::new(graph)
    }

    #[test]
    fn test_simple() {
        let size = 10;
        let graph = simple_graph(size);
        let transition_system = Arc::new(SimpleWorld::new(graph, 0.4));
        let mut solver = SafeIntervalPathPlanning::new(transition_system.clone());

        for x in 0..size {
            for y in 0..size {
                let task = Arc::new(Task::new(
                    SimpleState(GraphNodeId(x + size * y)),
                    SimpleState(GraphNodeId(size * size - 1)),
                    OrderedFloat(0.0),
                ));
                let config = SippConfig::new(
                    task.clone(),
                    Default::default(),
                    Default::default(),
                    Arc::new(ReverseResumableAStar::new(
                        transition_system.clone(),
                        task.clone(),
                        SimpleHeuristic::new(transition_system.clone(), Arc::new(task.reverse())),
                    )),
                    1e-6.into(),
                );
                let before = solver.get_stats();
                let solution = solver.solve(&config).unwrap();
                let after = solver.get_stats();
                assert_eq!(
                    solution.cost,
                    OrderedFloat(((size - x - 1) + (size - y - 1)) as f64)
                );
                assert_eq!(after.searches, before.searches + 1);
                // Check that the perfect heuristic works
                assert_eq!(after.expanded - before.expanded, solution.actions.len());
            }
        }
    }

    #[test]
    fn test_safe_intervals() {
        let state = SimpleState(GraphNodeId(0));

        let times = vec![
            OrderedFloat(10.0),
            OrderedFloat(11.0),
            OrderedFloat(12.0),
            OrderedFloat(13.0),
        ];

        let mut constraints = ConstraintSet::default();
        constraints.add(&Arc::new(Constraint::new_state_constraint(
            0,
            state.clone(),
            Interval::new(times[0], times[1]),
        )));
        constraints.add(&Arc::new(Constraint::new_state_constraint(
            0,
            state.clone(),
            Interval::new(times[2], times[3]),
        )));

        let mut safe_intervals = vec![];

        SafeIntervalPathPlanning::<
            SimpleWorld,
            SimpleState,
            GraphEdgeId,
            MyTime,
            MyTime,
            SimpleHeuristic,
        >::get_safe_intervals(
            &Arc::new(constraints),
            &state,
            &Interval::default(),
            OrderedFloat(1e-6),
            &mut safe_intervals,
        );

        assert_eq!(safe_intervals.len(), 3);
        assert_eq!(safe_intervals[0].end, times[0]);
        assert_eq!(safe_intervals[1].start, times[1]);
        assert_eq!(safe_intervals[1].end, times[2]);
        assert_eq!(safe_intervals[2].start, times[3]);
    }

    #[test]
    fn test_with_constraints() {
        let size = 10;
        let graph = simple_graph(size);
        let transition_system = Arc::new(SimpleWorld::new(graph, 0.4));
        let mut solver = SafeIntervalPathPlanning::new(transition_system.clone());

        let task = Arc::new(Task::new(
            SimpleState(GraphNodeId(0)),
            SimpleState(GraphNodeId(size * size - 1)),
            OrderedFloat(0.0),
        ));

        let times = vec![
            OrderedFloat(2.0),
            OrderedFloat(8.0),
            OrderedFloat(12.0),
            OrderedFloat(18.0),
        ];

        let mut constraints = ConstraintSet::default();
        for k in 0..size {
            for l in vec![3, 6] {
                for state in vec![
                    SimpleState(GraphNodeId(l + size * k)),
                    SimpleState(GraphNodeId(k + size * l)),
                ] {
                    constraints.add(&Arc::new(Constraint::new_state_constraint(
                        0,
                        state.clone(),
                        Interval::new(times[0], times[1]),
                    )));
                    constraints.add(&Arc::new(Constraint::new_state_constraint(
                        0,
                        state.clone(),
                        Interval::new(times[2], times[3]),
                    )));
                }
            }
        }

        let config = SippConfig::new(
            task.clone(),
            Default::default(),
            Arc::new(constraints),
            Arc::new(ReverseResumableAStar::new(
                transition_system.clone(),
                task.clone(),
                SimpleHeuristic::new(transition_system.clone(), Arc::new(task.reverse())),
            )),
            1e-6.into(),
        );

        let solution = solver.solve(&config).unwrap();

        assert_eq!(solution.cost, OrderedFloat(24.0));
    }
}
