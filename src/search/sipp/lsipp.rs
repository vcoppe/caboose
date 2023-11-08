use std::{collections::HashMap, fmt::Debug, hash::Hash, marker::PhantomData, sync::Arc};

use chrono::Duration;

use crate::{
    DifferentialHeuristic, GeneralizedSippConfig, Heuristic, Interval, SafeIntervalPathPlanning,
    SippConfig, SippState, SippTask, Solution, State, Task, Time, TransitionSystem,
};

/// Implementation of Safe Interval Path Planning algorithm that supports landmarks
/// (or positive constraints) to visit before aiming for the goal state.
pub struct SafeIntervalPathPlanningWithLandmarks<TS, S, A, H>
where
    TS: TransitionSystem<S, A, Duration>,
    S: State + Debug + Copy + Hash + Eq,
    A: Debug + Copy,
    H: Heuristic<TS, S, A, Time, Duration>,
{
    transition_system: Arc<TS>,
    sipp: SafeIntervalPathPlanning<TS, S, A, DifferentialHeuristic<TS, S, A, H>>,
    solutions: Vec<Solution<Arc<SippState<S>>, A, Time>>,
    parent: HashMap<(Arc<SippState<S>>, Time), (A, Arc<SippState<S>>, Time)>,
}

impl<TS, S, A, H> SafeIntervalPathPlanningWithLandmarks<TS, S, A, H>
where
    TS: TransitionSystem<S, A, Duration>,
    S: State + Debug + Copy + Hash + Eq,
    A: Debug + Copy,
    H: Heuristic<TS, S, A, Time, Duration>,
{
    fn new(transition_system: Arc<TS>) -> Self {
        Self {
            transition_system: transition_system.clone(),
            sipp: SafeIntervalPathPlanning::new(transition_system),
            parent: HashMap::new(),
            solutions: vec![],
        }
    }

    fn init(&mut self) {
        self.solutions.clear();
        self.parent.clear();
    }

    fn solve(
        &mut self,
        config: &LSippConfig<TS, S, A, H>,
    ) -> Option<Solution<Arc<SippState<S>>, A, Time>> {
        self.init();

        if config.landmarks.is_empty() {
            // No landmarks, just solve the task with SIPP
            self.sipp.solve(&mut SippConfig::new(
                config.initial_time,
                config.task.clone(),
                Default::default(),
                self.get_heuristic(config, config.task.clone()),
            ))
        } else {
            // Solve the task with landmarks
            self.solve_with_landmarks(config)
        }
    }

    fn solve_with_landmarks(
        &mut self,
        config: &LSippConfig<TS, S, A, H>,
    ) -> Option<Solution<Arc<SippState<S>>, A, Time>> {
        self.to_first_landmark(&config);
        self.between_landmarks(&config);
        self.to_goal(&config);
        self.get_solution()
    }

    // Go from the initial state to the first landmark
    fn to_first_landmark(&mut self, config: &LSippConfig<TS, S, A, H>) {
        let task = Arc::new(Task::new(
            config.task.initial_state(),
            config.landmarks[0].clone(),
        ));
        let config = self.sipp.to_generalized(
            &mut SippConfig::new(
                config.initial_time,
                task.clone(),
                config.intervals[0],
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
    fn between_landmarks(&mut self, config: &LSippConfig<TS, S, A, H>) {
        for (i, (landmark, interval)) in config
            .landmarks
            .iter()
            .zip(config.intervals.iter())
            .enumerate()
            .skip(1)
        {
            let task = Arc::new(Task::new(config.landmarks[i - 1].clone(), landmark.clone()));
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
                        safe_interval: *interval,
                        internal_state: landmark.clone(),
                    }),
                    task.clone(),
                )),
                self.get_heuristic(config, task),
                false,
            );

            self.solutions = self.sipp.solve_generalized(&config);

            self.store_parents();
        }
    }

    // Go from the last landmark to the goal state
    fn to_goal(&mut self, config: &LSippConfig<TS, S, A, H>) {
        let task = Arc::new(Task::new(
            config.landmarks.last().unwrap().clone(),
            config.task.goal_state(),
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
                    internal_state: config.task.goal_state(),
                }),
                task.clone(),
            )),
            self.get_heuristic(config, task),
            true,
        );

        self.solutions = self.sipp.solve_generalized(&config);

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
    fn get_solution(&self) -> Option<Solution<Arc<SippState<S>>, A, Time>> {
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
        config: &LSippConfig<TS, S, A, H>,
        task: Arc<Task<S>>,
    ) -> Arc<DifferentialHeuristic<TS, S, A, H>> {
        Arc::new(DifferentialHeuristic::new(
            task,
            config.task.goal_state(),
            config.heuristic.clone(),
        ))
    }
}

/// Input configuration for the Safe Interval Path Planning algorithm with landmarks.
pub struct LSippConfig<TS, S, A, H>
where
    TS: TransitionSystem<S, A, Duration>,
    S: State + Debug + Copy + Hash + Eq,
    A: Copy,
    H: Heuristic<TS, S, A, Time, Duration>,
{
    initial_time: Time,
    landmarks: Vec<Arc<S>>,
    intervals: Vec<Interval>,
    task: Arc<Task<S>>,
    heuristic: Arc<H>,
    _phantom: PhantomData<(TS, A)>,
}

impl<TS, S, A, H> LSippConfig<TS, S, A, H>
where
    TS: TransitionSystem<S, A, Duration>,
    S: State + Debug + Copy + Hash + Eq,
    A: Copy,
    H: Heuristic<TS, S, A, Time, Duration>,
{
    pub fn new(
        initial_time: Time,
        landmarks: Vec<Arc<S>>,
        intervals: Vec<Interval>,
        task: Arc<Task<S>>,
        heuristic: Arc<H>,
    ) -> Self {
        Self {
            initial_time,
            landmarks,
            intervals,
            task,
            heuristic,
            _phantom: PhantomData::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::Duration;

    use crate::{
        Graph, GraphNodeId, Interval, LSippConfig, ReverseResumableAStar,
        SafeIntervalPathPlanningWithLandmarks, SimpleHeuristic, SimpleState, SimpleWorld, Task,
        Time,
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
        let initial_time = Time::MIN_UTC.into();
        let mut solver = SafeIntervalPathPlanningWithLandmarks::new(transition_system.clone());

        for x in 0..size {
            for y in 0..size {
                let task = Arc::new(Task::new(
                    Arc::new(SimpleState(GraphNodeId(x + size * y))),
                    Arc::new(SimpleState(GraphNodeId(size * size - 1))),
                ));
                let mut config = LSippConfig::new(
                    initial_time,
                    vec![],
                    vec![],
                    task.clone(),
                    Arc::new(ReverseResumableAStar::new(
                        transition_system.clone(),
                        task.clone(),
                        Arc::new(SimpleHeuristic::new(transition_system.clone(), task)),
                    )),
                );
                assert_eq!(
                    *solver.solve(&mut config).unwrap().costs.last().unwrap(),
                    initial_time
                        + Duration::milliseconds((((size - x - 1) + (size - y - 1)) * 1000) as i64)
                );
            }
        }
    }

    #[test]
    fn test_with_landmarks() {
        let size = 10;
        let graph = simple_graph(size);
        let transition_system = Arc::new(SimpleWorld::new(graph));
        let initial_time = Time::MIN_UTC.into();
        let mut solver = SafeIntervalPathPlanningWithLandmarks::new(transition_system.clone());

        let task = Arc::new(Task::new(
            Arc::new(SimpleState(GraphNodeId(0))),
            Arc::new(SimpleState(GraphNodeId(size * size - 1))),
        ));
        let mut config = LSippConfig::new(
            initial_time,
            vec![
                Arc::new(SimpleState(GraphNodeId(size - 1))),
                Arc::new(SimpleState(GraphNodeId(size * (size - 1)))),
            ],
            vec![Interval::default(); 2],
            task.clone(),
            Arc::new(ReverseResumableAStar::new(
                transition_system.clone(),
                task.clone(),
                Arc::new(SimpleHeuristic::new(transition_system.clone(), task)),
            )),
        );
        assert_eq!(
            *solver.solve(&mut config).unwrap().costs.last().unwrap(),
            initial_time + Duration::milliseconds((4 * (size - 1) * 1000) as i64)
        );
    }
}
