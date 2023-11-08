use std::{collections::HashMap, fmt::Debug, hash::Hash, sync::Arc};

use chrono::Duration;

use crate::{
    GeneralizedSippConfig, Heuristic, Interval, SafeIntervalPathPlanning, SippConfig, SippState,
    SippTask, Solution, Task, Time, TransitionSystem,
};

/// Implementation of Safe Interval Path Planning algorithm that supports landmarks
/// (or positive constraints) to visit before aiming for the goal state.
pub struct SafeIntervalPathPlanningWithLandmarks<TS, S, A, H>
where
    TS: TransitionSystem<S, A, Duration>,
    S: Debug + Copy + Hash + Eq,
    A: Copy,
    H: Heuristic<TS, S, A, Time, Duration>,
{
    transition_system: Arc<TS>,
    sipp: SafeIntervalPathPlanning<TS, S, A, H>,
    heuristic: Arc<H>,
    solutions: Vec<Solution<Arc<SippState<S>>, A, Time>>,
    parent: HashMap<Arc<SippState<S>>, (A, Arc<SippState<S>>)>,
}

impl<TS, S, A, H> SafeIntervalPathPlanningWithLandmarks<TS, S, A, H>
where
    TS: TransitionSystem<S, A, Duration>,
    S: Debug + Copy + Hash + Eq,
    A: Copy,
    H: Heuristic<TS, S, A, Time, Duration>,
{
    fn new(transition_system: Arc<TS>, heuristic: Arc<H>) -> Self {
        Self {
            transition_system: transition_system.clone(),
            sipp: SafeIntervalPathPlanning::new(transition_system),
            heuristic,
            parent: HashMap::new(),
            solutions: vec![],
        }
    }

    fn init(&mut self) {
        self.solutions.clear();
        self.parent.clear();
    }

    fn solve(&mut self, task: LSippTask<S>) -> Option<Solution<Arc<SippState<S>>, A, Time>> {
        self.init();

        if task.landmarks.is_empty() {
            // No landmarks, just solve the task with SIPP
            self.sipp.solve(&mut SippConfig::new(
                task.initial_time,
                task.internal_task,
                self.heuristic.clone(),
            ))
        } else {
            // Solve the task with landmarks
            self.solve_with_landmarks(task)
        }
    }

    fn solve_with_landmarks(
        &mut self,
        task: LSippTask<S>,
    ) -> Option<Solution<Arc<SippState<S>>, A, Time>> {
        self.to_first_landmark(&task);
        self.between_landmarks(&task);
        self.to_goal(&task);
        self.get_solution()
    }

    // Go from the initial state to the first landmark
    fn to_first_landmark(&mut self, task: &LSippTask<S>) {
        let config = self.sipp.to_generalized(
            &mut SippConfig::new(
                task.initial_time,
                Arc::new(Task::new(
                    task.internal_task.initial_state(),
                    task.landmarks[0].clone(),
                )),
                self.heuristic.clone(), // TODO: use a heuristic specific to each goal
            ),
            false,
        );

        if config.is_none() {
            return;
        }
        let mut config = config.unwrap();

        self.solutions = self.sipp.solve_generalized(&mut config);

        self.store_parents();
    }

    // Connect all landmarks sequentially
    fn between_landmarks(&mut self, task: &LSippTask<S>) {
        for (i, landmark) in task.landmarks.iter().enumerate().skip(1) {
            let mut config = GeneralizedSippConfig::new(
                Arc::new(SippTask::new(
                    self.solutions.iter().map(|s| s.cost).collect(),
                    self.solutions
                        .iter()
                        .map(|s| s.states.last().unwrap().clone())
                        .collect(),
                    Arc::new(SippState {
                        safe_interval: Interval::default(),
                        internal_state: landmark.clone(),
                    }),
                    Arc::new(Task::new(task.landmarks[i - 1].clone(), landmark.clone())),
                )),
                self.heuristic.clone(), // TODO: use a heuristic specific to each goal
                false,
            );

            self.solutions = self.sipp.solve_generalized(&mut config);

            self.store_parents();
        }
    }

    // Go from the last landmark to the goal state
    fn to_goal(&mut self, task: &LSippTask<S>) {
        let mut config = GeneralizedSippConfig::new(
            Arc::new(SippTask::new(
                self.solutions.iter().map(|s| s.cost).collect(),
                self.solutions
                    .iter()
                    .map(|s| s.states.last().unwrap().clone())
                    .collect(),
                Arc::new(SippState {
                    safe_interval: Interval::default(),
                    internal_state: task.internal_task.goal_state(),
                }),
                Arc::new(Task::new(
                    task.landmarks.last().unwrap().clone(),
                    task.internal_task.goal_state(),
                )),
            )),
            self.heuristic.clone(), // TODO: use a heuristic specific to each goal
            true,
        );

        self.solutions = self.sipp.solve_generalized(&mut config);

        self.store_parents();
    }

    /// Stores the parent of each state in the solutions
    fn store_parents(&mut self) {
        for solution in self.solutions.iter() {
            for (i, state) in solution.states.iter().enumerate().skip(1) {
                self.parent.insert(
                    state.clone(),
                    (solution.actions[i - 1], solution.states[i - 1].clone()),
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
        let mut current = solution_to_goal.states.last().unwrap().clone();

        solution.cost = solution_to_goal.cost;
        solution.states.push(current.clone());

        while let Some((action, parent)) = self.parent.get(&current) {
            current = parent.clone();
            solution.actions.push(*action);
            solution.states.push(current.clone());
        }

        solution.actions.reverse();
        solution.states.reverse();

        Some(solution)
    }
}

/// Task wrapper for the Safe Interval Path Planning algorithm with landmarks
/// that extends a given task definition with a set of intermediate landmarks to visit.
pub struct LSippTask<S>
where
    S: Debug + Hash + Eq,
{
    pub initial_time: Time,
    pub landmarks: Vec<Arc<S>>,
    pub intervals: Vec<Interval>,
    pub internal_task: Arc<Task<S>>,
}
