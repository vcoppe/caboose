use std::{fmt::Debug, hash::Hash, marker::PhantomData, sync::Arc};

use chrono::Duration;

use crate::{
    Heuristic, ReverseResumableAStar, SafeIntervalPathPlanningWithLandmarks, Solution, State, Task,
    Time, TransitionSystem,
};

/// Implementation of the Conflict-Based Search algorithm.
struct ConflictBasedSearch<TS, S, A, H>
where
    TS: TransitionSystem<S, A, Duration>,
    S: Debug + State + Eq + Hash + Copy,
    A: Debug + Copy,
    H: Heuristic<TS, S, A, Time, Duration>,
{
    transition_system: Arc<TS>,
    lsipp: SafeIntervalPathPlanningWithLandmarks<
        TS,
        S,
        A,
        ReverseResumableAStar<TS, S, A, Time, Duration, H>,
    >,
    heuristics: Vec<Arc<ReverseResumableAStar<TS, S, A, Time, Duration, H>>>,
}

impl<TS, S, A, H> ConflictBasedSearch<TS, S, A, H>
where
    TS: TransitionSystem<S, A, Duration>,
    S: Debug + State + Eq + Hash + Copy,
    A: Debug + Copy,
    H: Heuristic<TS, S, A, Time, Duration>,
{
    pub fn new(transition_system: Arc<TS>) -> Self {
        let lsipp = SafeIntervalPathPlanningWithLandmarks::new(transition_system.clone());
        Self {
            transition_system,
            lsipp,
            heuristics: vec![],
        }
    }

    pub fn init(&mut self, config: &CbsConfig<TS, S, A, H>) {
        self.heuristics.clear();

        // Create a ReverseResumableAStar heuristic for each task
        for (task, heuristic) in config.tasks.iter().zip(config.heuristics.iter()) {
            self.heuristics.push(Arc::new(ReverseResumableAStar::new(
                self.transition_system.clone(),
                task.clone(),
                heuristic.clone(),
            )));
        }
    }

    pub fn solve(&mut self, config: &CbsConfig<TS, S, A, H>) -> Option<Solution<S, A, Time>> {
        todo!()
    }
}

/// Input configuration for the Conflict-Based Search algorithm.
struct CbsConfig<TS, S, A, H>
where
    TS: TransitionSystem<S, A, Duration>,
    S: State + Eq + Hash,
    H: Heuristic<TS, S, A, Time, Duration>,
{
    tasks: Vec<Arc<Task<S>>>,
    /// A basic heuristic for each task, that will be used inside the
    /// Reverse Resumable A* algorithm to compute a more precise heuristic.
    heuristics: Vec<Arc<H>>,
    _phantom: PhantomData<(TS, A)>,
}
