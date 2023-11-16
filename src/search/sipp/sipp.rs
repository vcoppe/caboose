use std::{
    cmp::Reverse,
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        BinaryHeap,
    },
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
    ops::{Add, Sub},
    rc::Rc,
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
    C: Hash
        + Eq
        + PartialOrd
        + Ord
        + Add<DC, Output = C>
        + Sub<DC, Output = C>
        + Sub<C, Output = DC>
        + Copy
        + Default
        + LimitValues,
    DC: PartialOrd + Copy + Default,
    H: Heuristic<TS, S, A, C, DC>,
{
    transition_system: Arc<TS>,
    queue: BinaryHeap<Reverse<SearchNode<SippState<S, C>, C, DC>>>,
    distance: FxHashMap<Rc<SippState<S, C>>, C>,
    closed: FxHashSet<Rc<SippState<S, C>>>,
    parent: FxHashMap<Rc<SippState<S, C>>, (Action<A, DC>, Rc<SippState<S, C>>)>,
    stats: SippStats,
    _phantom: PhantomData<(A, H)>,
}

impl<TS, S, A, C, DC, H> SafeIntervalPathPlanning<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: State + Debug + Hash + Eq + Clone,
    A: Copy,
    C: Hash
        + Eq
        + PartialOrd
        + Ord
        + Add<DC, Output = C>
        + Sub<DC, Output = C>
        + Sub<C, Output = DC>
        + Copy
        + Default
        + LimitValues,
    DC: PartialOrd + Copy + Default,
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
            stats: SippStats::default(),
            _phantom: PhantomData::default(),
        }
    }

    // Transforms the configuration into a generalized configuration, if any
    // safe intervals exist for the initial state.
    pub fn to_generalized(
        &self,
        config: &SippConfig<TS, S, A, C, DC, H>,
        single_path: bool,
    ) -> Option<GeneralizedSippConfig<TS, S, A, C, DC, H>> {
        let initial_time = config.task.initial_cost;

        let safe_intervals =
            Self::get_safe_intervals(&config.constraints, &config.task.initial_state);
        let safe_interval = safe_intervals
            .iter()
            .find(|interval| initial_time >= interval.start && initial_time < interval.end);

        if safe_interval.is_none() {
            return None;
        }

        let initial_state = Rc::new(SippState {
            safe_interval: *safe_interval.unwrap(),
            internal_state: config.task.initial_state.clone(),
        });

        let goal_state = SippState {
            safe_interval: config.interval,
            internal_state: config.task.goal_state.clone(),
        };

        let sipp_task = SippTask::new(
            vec![initial_time],
            vec![initial_state],
            goal_state,
            config.task.clone(),
        );

        Some(GeneralizedSippConfig::new(
            sipp_task,
            config.constraints.clone(),
            config.heuristic.clone(),
            single_path,
        ))
    }

    /// Attempts to solve the given configuration, and returns the optimal solution if any.
    pub fn solve(
        &mut self,
        config: &SippConfig<TS, S, A, C, DC, H>,
    ) -> Option<Solution<Rc<SippState<S, C>>, A, C, DC>> {
        self.to_generalized(config, true)
            .map(|config| self.solve_generalized(&config).pop())
            .flatten()
    }
    /// Attempts to solve the given generalized configuration, and returns the optimal solution if any.
    pub fn solve_generalized(
        &mut self,
        config: &GeneralizedSippConfig<TS, S, A, C, DC, H>,
    ) -> Vec<Solution<Rc<SippState<S, C>>, A, C, DC>> {
        self.init(config);
        self.find_paths(config)
            .iter()
            .map(|g| self.get_solution(g))
            .collect()
    }

    /// Initializes the search algorithm by clearing the data structures
    /// and enqueueing the initial states.
    fn init(&mut self, config: &GeneralizedSippConfig<TS, S, A, C, DC, H>) {
        self.queue.clear();
        self.distance.clear();
        self.closed.clear();
        self.parent.clear();

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

        self.stats.searches += 1;
    }

    /// Finds all shortest paths from the initial states to any reachable safe interval
    /// at the goal state.
    fn find_paths(
        &mut self,
        config: &GeneralizedSippConfig<TS, S, A, C, DC, H>,
    ) -> Vec<SearchNode<SippState<S, C>, C, DC>> {
        let mut goals = vec![];

        while let Some(Reverse(current)) = self.queue.pop() {
            if current.cost > self.distance[current.state.as_ref()] {
                // A better path has already been found
                continue;
            }

            if config.task.is_goal(&current) {
                // A path to the goal has been found
                goals.push(current.clone());
                if config.single_path {
                    break;
                }
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
        current: &SearchNode<SippState<S, C>, C, DC>,
    ) {
        for action in self
            .transition_system
            .actions_from(&current.state.internal_state)
        {
            let successor_state = self
                .transition_system
                .transition(&current.state.internal_state, &action);
            let transition_cost = self
                .transition_system
                .transition_cost(&current.state.internal_state, &action);

            let heuristic = config.heuristic.get_heuristic(&successor_state);
            if heuristic.is_none() {
                continue; // Goal state is not reachable from this state
            }
            let heuristic = heuristic.unwrap();

            if current.cost + transition_cost + heuristic
                >= config.task.goal_state.safe_interval.end
            {
                // The goal state is not reachable in time
                continue;
            }

            let action_constraints = config
                .constraints
                .get_action_constraints(&current.state.internal_state, &successor_state);

            // Try to reach any of the safe intervals of the destination state
            // and add the corresponding successors to the queue if a better path has been found
            for safe_interval in
                Self::get_safe_intervals(&config.constraints, &successor_state).drain(..)
            {
                let mut successor_cost = current.cost + transition_cost;

                if successor_cost >= safe_interval.end {
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
                    if successor_cost - transition_cost >= current.state.safe_interval.end {
                        // Cannot depart that late from the current safe interval
                        continue;
                    }
                }

                // Check collision along the action
                if let Some(collision_interval) = action_constraints
                    .map(|col| {
                        col.iter()
                            .find(|c| c.interval.end > successor_cost - transition_cost)
                            .map(|c| c.interval)
                    })
                    .flatten()
                {
                    if successor_cost - transition_cost >= collision_interval.start {
                        // Collision detected
                        if !self
                            .transition_system
                            .can_wait_at(&current.state.internal_state)
                        {
                            // Cannot wait at the current state
                            continue;
                        }
                        successor_cost = collision_interval.end + transition_cost; // Try to depart later

                        if successor_cost - transition_cost >= current.state.safe_interval.end
                            || successor_cost >= safe_interval.end
                        {
                            continue;
                        }
                    }
                }

                if successor_cost + heuristic >= config.task.goal_state.safe_interval.end {
                    // The goal state is not reachable in time
                    continue;
                }

                let successor_state = Rc::new(SippState {
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

    /// Returns the safe intervals for the given state, given a set of constraints.
    fn get_safe_intervals(constraints: &Arc<ConstraintSet<S, C>>, state: &S) -> Vec<Interval<C>> {
        if let Some(state_constraints) = constraints.get_state_constraints(state) {
            let mut safe_intervals = vec![];

            let mut current = Interval::default();
            for constraint in state_constraints.iter() {
                current.end = constraint.interval.start;
                if current.start < current.end {
                    safe_intervals.push(current);
                }
                current.start = constraint.interval.end;
            }
            current.end = Interval::default().end;
            if current.start < current.end {
                safe_intervals.push(current);
            }

            safe_intervals
        } else {
            vec![Interval::default()]
        }
    }

    /// Reconstructs the solution from the given goal search node.
    fn get_solution(
        &self,
        goal: &SearchNode<SippState<S, C>, C, DC>,
    ) -> Solution<Rc<SippState<S, C>>, A, C, DC> {
        let mut solution = Solution::default();
        let mut current = goal.state.clone();

        solution.states.push(current.clone());
        solution.costs.push(self.distance[&current]);

        while let Some((action, parent)) = self.parent.get(&current) {
            if self.distance[&current] - self.distance[parent] > action.cost {
                solution.states.push(parent.clone());
                solution.costs.push(self.distance[&current] - action.cost);
                solution.actions.push(*action);

                // Insert wait action
                solution.states.push(parent.clone());
                solution.costs.push(self.distance[parent]);
                solution.actions.push(Action::wait(action.cost));
            } else {
                solution.states.push(parent.clone());
                solution.costs.push(self.distance[parent]);
                solution.actions.push(*action);
            }

            current = parent.clone();
        }

        solution.costs.reverse();
        solution.actions.reverse();
        solution.states.reverse();

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
    H: Heuristic<TS, S, A, C, DC>,
{
    task: Arc<Task<S, C>>,
    interval: Interval<C>,
    constraints: Arc<ConstraintSet<S, C>>,
    heuristic: Arc<H>,
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
    H: Heuristic<TS, S, A, C, DC>,
{
    pub fn new(
        task: Arc<Task<S, C>>,
        interval: Interval<C>,
        constraints: Arc<ConstraintSet<S, C>>,
        heuristic: Arc<H>,
    ) -> Self {
        SippConfig {
            task,
            interval,
            constraints,
            heuristic,
            _phantom: PhantomData::default(),
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
    DC: Copy,
    H: Heuristic<TS, S, A, C, DC>,
{
    task: SippTask<S, C, DC>,
    constraints: Arc<ConstraintSet<S, C>>,
    heuristic: Arc<H>,
    single_path: bool,
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
    DC: Copy,
    H: Heuristic<TS, S, A, C, DC>,
{
    pub fn new(
        task: SippTask<S, C, DC>,
        constraints: Arc<ConstraintSet<S, C>>,
        heuristic: Arc<H>,
        single_path: bool,
    ) -> Self {
        GeneralizedSippConfig {
            task,
            constraints,
            heuristic,
            single_path,
            _phantom: PhantomData::default(),
        }
    }
}

/// State wrapper for the Safe Interval Path Planning algorithm that extends
/// a given state definition with a safe interval.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct SippState<S, C>
where
    S: Debug + Eq,
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues,
{
    pub safe_interval: Interval<C>,
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
    initial_states: Vec<Rc<SippState<S, C>>>,
    goal_state: SippState<S, C>,
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
        initial_states: Vec<Rc<SippState<S, C>>>,
        goal_state: SippState<S, C>,
        internal_task: Arc<Task<S, C>>,
    ) -> Self {
        SippTask {
            initial_times,
            initial_states,
            goal_state,
            internal_task,
            _phantom: PhantomData::default(),
        }
    }

    fn is_goal(&self, state: &SearchNode<SippState<S, C>, C, DC>) -> bool {
        state.cost >= self.goal_state.safe_interval.start
            && state.cost < self.goal_state.safe_interval.end
            && self
                .internal_task
                .is_goal_state(&state.state.internal_state)
    }
}

/// Statistics of the Safe Interval Path Planning algorithm.
#[derive(Debug, Clone, Copy, Default)]
pub struct SippStats {
    pub searches: usize,
    pub expanded: usize,
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ordered_float::OrderedFloat;

    use crate::{
        search::sipp::sipp::SippConfig, Constraint, ConstraintSet, Graph, GraphEdgeId, GraphNodeId,
        Interval, MyTime, ReverseResumableAStar, SimpleHeuristic, SimpleState, SimpleWorld, Task,
    };

    use super::SafeIntervalPathPlanning;

    fn simple_graph(size: usize) -> Arc<Graph> {
        let mut graph = Graph::new();
        for x in 0..size {
            for y in 0..size {
                graph.add_node((x as f32, y as f32));
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
        let transition_system = Arc::new(SimpleWorld::new(graph));
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
                );
                let before = solver.get_stats();
                let solution = solver.solve(&config).unwrap();
                let after = solver.get_stats();
                assert_eq!(
                    *solution.costs.last().unwrap(),
                    OrderedFloat(((size - x - 1) + (size - y - 1)) as f32)
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

        let safe_intervals = SafeIntervalPathPlanning::<
            SimpleWorld,
            SimpleState,
            GraphEdgeId,
            MyTime,
            MyTime,
            SimpleHeuristic,
        >::get_safe_intervals(&Arc::new(constraints), &state);

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
        let transition_system = Arc::new(SimpleWorld::new(graph));
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
        );

        let solution = solver.solve(&config).unwrap();

        assert_eq!(*solution.costs.last().unwrap(), OrderedFloat(24.0));
    }
}
