use std::{
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
    ops::{Add, Sub},
    rc::Rc,
    sync::Arc,
};

use fxhash::FxHashMap;

use crate::{
    Action, ConstraintSet, DifferentialHeuristic, GeneralizedSippConfig, Heuristic, Interval,
    LandmarkSet, LimitValues, SafeIntervalPathPlanning, SippConfig, SippState, SippStats, SippTask,
    Solution, State, Task, TransitionSystem,
};

/// Implementation of Safe Interval Path Planning algorithm that supports landmarks
/// (or positive constraints) to visit before aiming for the goal state.
pub struct SafeIntervalPathPlanningWithLandmarks<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: State + Debug + Hash + Eq + Clone,
    A: Debug + Copy,
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
    DC: Ord + Sub<DC, Output = DC> + Copy + Default,
    H: Heuristic<TS, S, A, C, DC>,
{
    sipp: SafeIntervalPathPlanning<TS, S, A, C, DC, DifferentialHeuristic<TS, S, A, C, DC, H>>,
    solutions: Vec<Solution<Rc<SippState<S, C>>, A, C, DC>>,
    parent: FxHashMap<(Rc<SippState<S, C>>, C), (Action<A, DC>, Rc<SippState<S, C>>, C)>,
    stats: LSippStats,
}

impl<TS, S, A, C, DC, H> SafeIntervalPathPlanningWithLandmarks<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: State + Debug + Hash + Eq + Clone,
    A: Debug + Copy,
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
    DC: Ord + Sub<DC, Output = DC> + Copy + Default,
    H: Heuristic<TS, S, A, C, DC>,
{
    /// Creates a new instance of the Safe Interval Path Planning algorithm with landmarks.
    pub fn new(transition_system: Arc<TS>) -> Self {
        Self {
            sipp: SafeIntervalPathPlanning::new(transition_system),
            parent: FxHashMap::default(),
            solutions: vec![],
            stats: LSippStats::default(),
        }
    }

    fn init(&mut self) {
        self.solutions.clear();
        self.parent.clear();

        self.stats.searches += 1;
    }

    /// Attempts to solve the given configuration, and returns the solution if any.
    pub fn solve(
        &mut self,
        config: &LSippConfig<TS, S, A, C, DC, H>,
    ) -> Option<Solution<Arc<SippState<S, C>>, A, C, DC>> {
        self.init();

        if config.landmarks.is_empty() {
            // No landmarks, just solve the task with SIPP
            self.sipp
                .solve(&mut SippConfig::new(
                    config.task.clone(),
                    Default::default(),
                    config.constraints.clone(),
                    self.get_heuristic(config, config.task.clone()),
                ))
                .map(Self::map_to_arcs)
        } else {
            // Solve the task with landmarks
            self.to_first_landmark(&config);
            self.between_landmarks(&config);
            self.to_goal(&config);
            self.get_solution()
        }
    }

    // Go from the initial state to the first landmark
    fn to_first_landmark(&mut self, config: &LSippConfig<TS, S, A, C, DC, H>) {
        let task = Arc::new(Task::new(
            config.task.initial_state.clone(),
            config.landmarks[0].state.clone(),
            config.task.initial_cost,
        ));
        let config = self.sipp.to_generalized(
            &mut SippConfig::new(
                task.clone(),
                config.landmarks[0].interval.clone(),
                config.constraints.clone(),
                self.get_heuristic(config, task),
            ),
            false,
        );

        if config.is_none() {
            return;
        }

        let config = config.unwrap();

        self.solutions = self.sipp.solve_generalized(&config);

        self.store_parents();
    }

    // Connect all landmarks sequentially
    fn between_landmarks(&mut self, config: &LSippConfig<TS, S, A, C, DC, H>) {
        for (i, landmark) in config.landmarks.iter().enumerate().skip(1) {
            let task = Arc::new(Task::new(
                config.landmarks[i - 1].state.clone(),
                landmark.state.clone(),
                config.task.initial_cost,
            ));
            let config = GeneralizedSippConfig::new(
                SippTask::new(
                    self.solutions
                        .iter()
                        .map(|s| *s.costs.last().unwrap())
                        .collect(),
                    self.solutions
                        .iter()
                        .map(|s| s.states.last().unwrap().clone())
                        .collect(),
                    SippState {
                        safe_interval: landmark.interval.clone(),
                        internal_state: landmark.state.clone(),
                    },
                    task.clone(),
                ),
                config.constraints.clone(),
                self.get_heuristic(config, task),
                false,
            );

            self.solutions = self.sipp.solve_generalized(&config);

            self.store_parents();
        }
    }

    // Go from the last landmark to the goal state
    fn to_goal(&mut self, config: &LSippConfig<TS, S, A, C, DC, H>) {
        let task = Arc::new(Task::new(
            config.landmarks[config.landmarks.len() - 1].state.clone(),
            config.task.goal_state.clone(),
            config.task.initial_cost,
        ));
        let config = GeneralizedSippConfig::new(
            SippTask::new(
                self.solutions
                    .iter()
                    .map(|s| *s.costs.last().unwrap())
                    .collect(),
                self.solutions
                    .iter()
                    .map(|s| s.states.last().unwrap().clone())
                    .collect(),
                SippState {
                    safe_interval: Interval::default(),
                    internal_state: config.task.goal_state.clone(),
                },
                task.clone(),
            ),
            config.constraints.clone(),
            self.get_heuristic(config, task),
            false, // we want a solution that reaches an [t,+inf) safe interval
        );

        self.solutions = self.sipp.solve_generalized(&config);
        if !self.solutions.is_empty() {
            self.solutions = vec![self.solutions.pop().unwrap()];
        }

        self.store_parents();
    }

    /// Stores the parent of each state in the solutions
    fn store_parents(&mut self) {
        for solution in self.solutions.iter() {
            for (i, (state, cost)) in solution
                .states
                .iter()
                .zip(solution.costs.iter())
                .enumerate()
                .skip(1)
            {
                self.parent.insert(
                    (state.clone(), *cost),
                    (
                        solution.actions[i - 1],
                        solution.states[i - 1].clone(),
                        solution.costs[i - 1],
                    ),
                );
            }
        }
    }

    /// Returns the solution to the given task, if any.
    fn get_solution(&self) -> Option<Solution<Arc<SippState<S, C>>, A, C, DC>> {
        if self.solutions.is_empty() {
            return None;
        }

        let solution_to_goal = &self.solutions[0];

        let mut solution = Solution::default();
        let mut current_state = solution_to_goal.states.last().unwrap().clone();
        let mut current_cost = *solution_to_goal.costs.last().unwrap();

        solution.states.push(current_state.clone());
        solution.costs.push(current_cost);

        while let Some((action, parent_state, parent_cost)) =
            self.parent.get(&(current_state, current_cost))
        {
            current_state = parent_state.clone();
            current_cost = *parent_cost;
            solution.states.push(current_state.clone());
            solution.costs.push(current_cost);
            solution.actions.push(*action);
        }

        solution.states.reverse();
        solution.costs.reverse();
        solution.actions.reverse();

        Some(Self::map_to_arcs(solution))
    }

    fn map_to_arcs(
        solution: Solution<Rc<SippState<S, C>>, A, C, DC>,
    ) -> Solution<Arc<SippState<S, C>>, A, C, DC> {
        Solution {
            states: solution
                .states
                .into_iter()
                .map(|s| Arc::new(s.as_ref().clone()))
                .collect::<Vec<_>>(),
            costs: solution.costs,
            actions: solution.actions,
        }
    }

    fn get_heuristic(
        &self,
        config: &LSippConfig<TS, S, A, C, DC, H>,
        task: Arc<Task<S, C>>,
    ) -> Arc<DifferentialHeuristic<TS, S, A, C, DC, H>> {
        Arc::new(DifferentialHeuristic::new(
            task,
            config.pivots.clone(),
            config.heuristic_to_pivots.clone(),
        ))
    }

    /// Returns the statistics of the search algorithm.
    pub fn get_stats(&mut self) -> LSippStats {
        self.stats.sipp_stats = self.sipp.get_stats();
        self.stats
    }
}

/// Input configuration for the Safe Interval Path Planning algorithm with landmarks.
pub struct LSippConfig<TS, S, A, C, DC, H>
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
    task: Arc<Task<S, C>>,
    constraints: Arc<ConstraintSet<S, C>>,
    landmarks: LandmarkSet<S, C>,
    /// A set of pivot states.
    pivots: Arc<Vec<S>>,
    /// A set of heuristics to those pivot states.
    heuristic_to_pivots: Arc<Vec<Arc<H>>>,
    _phantom: PhantomData<(TS, A)>,
}

impl<TS, S, A, C, DC, H> LSippConfig<TS, S, A, C, DC, H>
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
        task: Arc<Task<S, C>>,
        constraints: Arc<ConstraintSet<S, C>>,
        landmarks: LandmarkSet<S, C>,
        heuristic: Arc<H>,
    ) -> Self {
        Self {
            task: task.clone(),
            constraints,
            landmarks,
            pivots: Arc::new(vec![task.goal_state.clone()]),
            heuristic_to_pivots: Arc::new(vec![heuristic]),
            _phantom: PhantomData::default(),
        }
    }

    pub fn new_with_pivots(
        task: Arc<Task<S, C>>,
        constraints: Arc<ConstraintSet<S, C>>,
        landmarks: LandmarkSet<S, C>,
        pivots: Arc<Vec<S>>,
        heuristic_to_pivots: Arc<Vec<Arc<H>>>,
    ) -> Self {
        Self {
            task,
            constraints,
            landmarks,
            pivots,
            heuristic_to_pivots,
            _phantom: PhantomData::default(),
        }
    }
}

/// Statistics of the Safe Interval Path Planning algorithm with landmarks.
#[derive(Debug, Default, Clone, Copy)]
pub struct LSippStats {
    pub searches: usize,
    pub sipp_stats: SippStats,
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ordered_float::OrderedFloat;

    use crate::{
        Constraint, Graph, GraphNodeId, Interval, LSippConfig, ReverseResumableAStar,
        SafeIntervalPathPlanningWithLandmarks, SimpleHeuristic, SimpleState, SimpleWorld, Task,
    };

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
        let mut solver = SafeIntervalPathPlanningWithLandmarks::new(transition_system.clone());

        for x in 0..size {
            for y in 0..size {
                let task = Arc::new(Task::new(
                    SimpleState(GraphNodeId(x + size * y)),
                    SimpleState(GraphNodeId(size * size - 1)),
                    OrderedFloat(0.0),
                ));
                let config = LSippConfig::new(
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
                assert_eq!(after.sipp_stats.searches, before.sipp_stats.searches + 1);
            }
        }
    }

    #[test]
    fn test_with_landmarks() {
        let size = 10;
        let graph = simple_graph(size);
        let transition_system = Arc::new(SimpleWorld::new(graph));
        let mut solver = SafeIntervalPathPlanningWithLandmarks::new(transition_system.clone());

        let task = Arc::new(Task::new(
            SimpleState(GraphNodeId(0)),
            SimpleState(GraphNodeId(size * size - 1)),
            OrderedFloat(0.0),
        ));
        let config = LSippConfig::new(
            task.clone(),
            Default::default(),
            vec![
                Arc::new(Constraint::new_state_constraint(
                    0,
                    SimpleState(GraphNodeId(size - 1)),
                    Interval::default(),
                )),
                Arc::new(Constraint::new_state_constraint(
                    0,
                    SimpleState(GraphNodeId(size * (size - 1))),
                    Interval::default(),
                )),
            ],
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
            OrderedFloat((4 * (size - 1)) as f32)
        );
        assert_eq!(after.searches, before.searches + 1);
        assert_eq!(after.sipp_stats.searches, before.sipp_stats.searches + 3);
    }
}
