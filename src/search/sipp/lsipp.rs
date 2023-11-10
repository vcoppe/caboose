use std::{
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
    ops::{Add, Sub},
    sync::Arc,
};

use crate::{
    ConstraintSet, DifferentialHeuristic, GeneralizedSippConfig, Heuristic, Interval, LandmarkSet,
    LimitValues, SafeIntervalPathPlanning, SippConfig, SippState, SippTask, Solution, State, Task,
    TransitionSystem,
};

/// Implementation of Safe Interval Path Planning algorithm that supports landmarks
/// (or positive constraints) to visit before aiming for the goal state.
pub struct SafeIntervalPathPlanningWithLandmarks<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: State + Debug + Copy + Hash + Eq,
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
    solutions: Vec<Solution<Arc<SippState<S, C>>, A, C>>,
    parent: HashMap<(Arc<SippState<S, C>>, C), (A, Arc<SippState<S, C>>, C)>,
}

impl<TS, S, A, C, DC, H> SafeIntervalPathPlanningWithLandmarks<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: State + Debug + Copy + Hash + Eq,
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
            parent: HashMap::new(),
            solutions: vec![],
        }
    }

    fn init(&mut self) {
        self.solutions.clear();
        self.parent.clear();
    }

    /// Attempts to solve the given configuration, and returns the solution if any.
    pub fn solve(
        &mut self,
        config: &LSippConfig<TS, S, A, C, DC, H>,
    ) -> Option<Solution<Arc<SippState<S, C>>, A, C>> {
        self.init();

        if config.landmarks.is_empty() {
            // No landmarks, just solve the task with SIPP
            self.sipp.solve(&mut SippConfig::new(
                config.task.clone(),
                Default::default(),
                config.constraints.clone(),
                self.get_heuristic(config, config.task.clone()),
            ))
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
                Arc::new(SippTask::new(
                    self.solutions
                        .iter()
                        .map(|s| *s.costs.last().unwrap())
                        .collect(),
                    self.solutions
                        .iter()
                        .map(|s| s.states.last().unwrap().clone())
                        .collect(),
                    Arc::new(SippState {
                        safe_interval: landmark.interval.clone(),
                        internal_state: landmark.state.clone(),
                    }),
                    task.clone(),
                )),
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
            Arc::new(SippTask::new(
                self.solutions
                    .iter()
                    .map(|s| *s.costs.last().unwrap())
                    .collect(),
                self.solutions
                    .iter()
                    .map(|s| s.states.last().unwrap().clone())
                    .collect(),
                Arc::new(SippState {
                    safe_interval: Interval::default(),
                    internal_state: config.task.goal_state.clone(),
                }),
                task.clone(),
            )),
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
    fn get_solution(&self) -> Option<Solution<Arc<SippState<S, C>>, A, C>> {
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

        Some(solution)
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
}

/// Input configuration for the Safe Interval Path Planning algorithm with landmarks.
pub struct LSippConfig<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: State + Debug + Copy + Hash + Eq,
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
    landmarks: Arc<LandmarkSet<S, C>>,
    constraints: Arc<ConstraintSet<S, C>>,
    /// A set of pivot states.
    pivots: Arc<Vec<Arc<S>>>,
    /// A set of heuristics to those pivot states.
    heuristic_to_pivots: Arc<Vec<Arc<H>>>,
    _phantom: PhantomData<(TS, A)>,
}

impl<TS, S, A, C, DC, H> LSippConfig<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: State + Debug + Copy + Hash + Eq,
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
        landmarks: Arc<LandmarkSet<S, C>>,
        constraints: Arc<ConstraintSet<S, C>>,
        heuristic: Arc<H>,
    ) -> Self {
        Self {
            task: task.clone(),
            landmarks,
            constraints,
            pivots: Arc::new(vec![task.goal_state.clone()]),
            heuristic_to_pivots: Arc::new(vec![heuristic]),
            _phantom: PhantomData::default(),
        }
    }

    pub fn new_with_pivots(
        task: Arc<Task<S, C>>,
        landmarks: Arc<LandmarkSet<S, C>>,
        constraints: Arc<ConstraintSet<S, C>>,
        pivots: Arc<Vec<Arc<S>>>,
        heuristic_to_pivots: Arc<Vec<Arc<H>>>,
    ) -> Self {
        Self {
            task,
            landmarks,
            constraints,
            pivots,
            heuristic_to_pivots,
            _phantom: PhantomData::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::{Duration, Local, TimeZone};

    use crate::{
        Constraint, Graph, GraphNodeId, Interval, LSippConfig, MyDuration, MyTime,
        ReverseResumableAStar, SafeIntervalPathPlanningWithLandmarks, SimpleHeuristic, SimpleState,
        SimpleWorld, Task,
    };

    fn simple_graph(size: usize) -> Arc<Graph> {
        let mut graph = Graph::new();
        for x in 0..size {
            for y in 0..size {
                graph.add_node((x as f64, y as f64), 1.0);
            }
        }
        for x in 0..size {
            for y in 0..size {
                let node_id = GraphNodeId(x + y * size);
                if x > 0 {
                    graph.add_edge(node_id, GraphNodeId(x - 1 + y * size), 1.0, 1.0);
                }
                if y > 0 {
                    graph.add_edge(node_id, GraphNodeId(x + (y - 1) * size), 1.0, 1.0);
                }
                if x < size - 1 {
                    graph.add_edge(node_id, GraphNodeId(x + 1 + y * size), 1.0, 1.0);
                }
                if y < size - 1 {
                    graph.add_edge(node_id, GraphNodeId(x + (y + 1) * size), 1.0, 1.0);
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
        let initial_time = MyTime(Local.with_ymd_and_hms(2000, 01, 01, 10, 0, 0).unwrap());
        let mut solver = SafeIntervalPathPlanningWithLandmarks::new(transition_system.clone());

        for x in 0..size {
            for y in 0..size {
                let task = Arc::new(Task::new(
                    Arc::new(SimpleState(GraphNodeId(x + size * y))),
                    Arc::new(SimpleState(GraphNodeId(size * size - 1))),
                    initial_time,
                ));
                let mut config = LSippConfig::new(
                    task.clone(),
                    Default::default(),
                    Default::default(),
                    Arc::new(ReverseResumableAStar::new(
                        transition_system.clone(),
                        task.clone(),
                        Arc::new(SimpleHeuristic::new(transition_system.clone(), task)),
                    )),
                );
                assert_eq!(
                    *solver.solve(&mut config).unwrap().costs.last().unwrap(),
                    initial_time
                        + MyDuration(Duration::seconds(((size - x - 1) + (size - y - 1)) as i64))
                );
            }
        }
    }

    #[test]
    fn test_with_landmarks() {
        let size = 10;
        let graph = simple_graph(size);
        let transition_system = Arc::new(SimpleWorld::new(graph));
        let initial_time = MyTime(Local.with_ymd_and_hms(2000, 01, 01, 10, 0, 0).unwrap());
        let mut solver = SafeIntervalPathPlanningWithLandmarks::new(transition_system.clone());

        let task = Arc::new(Task::new(
            Arc::new(SimpleState(GraphNodeId(0))),
            Arc::new(SimpleState(GraphNodeId(size * size - 1))),
            initial_time,
        ));
        let mut config = LSippConfig::new(
            task.clone(),
            Arc::new(vec![
                Arc::new(Constraint::new_state_constraint(
                    0,
                    Arc::new(SimpleState(GraphNodeId(size - 1))),
                    Interval::default(),
                )),
                Arc::new(Constraint::new_state_constraint(
                    0,
                    Arc::new(SimpleState(GraphNodeId(size * (size - 1)))),
                    Interval::default(),
                )),
            ]),
            Default::default(),
            Arc::new(ReverseResumableAStar::new(
                transition_system.clone(),
                task.clone(),
                Arc::new(SimpleHeuristic::new(transition_system.clone(), task)),
            )),
        );
        assert_eq!(
            *solver.solve(&mut config).unwrap().costs.last().unwrap(),
            initial_time + MyDuration(Duration::seconds((4 * (size - 1)) as i64))
        );
    }
}
