use std::{hash::Hash, marker::PhantomData, ops::Add, sync::Arc};

use chrono::Duration;

use crate::{State, Task, Time, TransitionSystem};

/// Description of a solution to a search problem
#[derive(Debug)]
pub struct Solution<S, A, C>
where
    C: Default,
{
    pub states: Vec<S>,
    pub costs: Vec<C>,
    pub actions: Vec<A>,
}

impl<S, A, C> Default for Solution<S, A, C>
where
    C: Default,
{
    fn default() -> Self {
        Self {
            states: Default::default(),
            costs: Default::default(),
            actions: Default::default(),
        }
    }
}

/// Defines a heuristic function that can be used by a search algorithm,
/// for a given transition system and task.
pub trait Heuristic<TS, S, A, C, DC>
where
    TS: TransitionSystem<S, A, DC>,
    S: Hash + Eq,
    C: Eq + PartialOrd + Ord + Add<DC, Output = C> + Copy + Default,
{
    /// Returns the heuristic value for the given state,
    /// or None if the goal state is not reachable from that state.
    fn get_heuristic(&self, state: Arc<S>) -> Option<DC>;
}

/// Differentiable heuristic built on top of heuristics dealing with
/// time and durations.
pub struct DifferentialHeuristic<TS, S, A, H>
where
    TS: TransitionSystem<S, A, Duration>,
    S: State + Hash + Eq,
    H: Heuristic<TS, S, A, Time, Duration>,
{
    task: Arc<Task<S>>,
    pivot: Arc<S>,
    heuristic_to_pivot: Arc<H>,
    _phantom: PhantomData<(TS, S, A)>,
}

impl<TS, S, A, H> DifferentialHeuristic<TS, S, A, H>
where
    TS: TransitionSystem<S, A, Duration>,
    S: State + Hash + Eq,
    H: Heuristic<TS, S, A, Time, Duration>,
{
    pub fn new(task: Arc<Task<S>>, pivot: Arc<S>, heuristic_to_pivot: Arc<H>) -> Self {
        DifferentialHeuristic {
            task,
            pivot,
            heuristic_to_pivot,
            _phantom: PhantomData::default(),
        }
    }
}

impl<TS, S, A, H> Heuristic<TS, S, A, Time, Duration> for DifferentialHeuristic<TS, S, A, H>
where
    TS: TransitionSystem<S, A, Duration>,
    S: State + Hash + Eq,
    H: Heuristic<TS, S, A, Time, Duration>,
{
    fn get_heuristic(&self, state: Arc<S>) -> Option<Duration> {
        if self.pivot == self.task.goal_state() {
            self.heuristic_to_pivot.get_heuristic(state.clone())
        } else if let (Some(h1), Some(h2)) = (
            self.heuristic_to_pivot.get_heuristic(state.clone()),
            self.heuristic_to_pivot
                .get_heuristic(self.task.goal_state()),
        ) {
            Some((h2 - h1).abs())
        } else {
            None
        }
    }
}

/// Generic definition of a search node and the associated ordering functions
/// that allow performing best-first searches by ordering nodes by increasing
/// (cost + heuristic) values, with a tie-breaking favoring nodes with higher cost.
#[derive(Debug, Clone)]
pub struct SearchNode<S: Hash, C: Copy + Eq + Ord + Add<DC, Output = C>, DC: Copy> {
    pub state: Arc<S>,
    pub cost: C,
    pub heuristic: DC,
}

impl<S: Hash, C: Copy + Eq + Ord + Add<DC, Output = C>, DC: Copy> PartialEq
    for SearchNode<S, C, DC>
{
    fn eq(&self, other: &Self) -> bool {
        self.cost + self.heuristic == other.cost + other.heuristic
    }
}

impl<S: Hash, C: Copy + Eq + Ord + Add<DC, Output = C>, DC: Copy> Eq for SearchNode<S, C, DC> {}

impl<S: Hash, C: Copy + Eq + Ord + Add<DC, Output = C>, DC: Copy> PartialOrd
    for SearchNode<S, C, DC>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.cost + self.heuristic == other.cost + other.heuristic {
            return other.cost.partial_cmp(&self.cost); // Estimation is more precise when the cost is larger
        } else {
            (self.cost + self.heuristic).partial_cmp(&(other.cost + other.heuristic))
        }
    }
}

impl<S: Hash, C: Copy + Eq + Ord + Add<DC, Output = C>, DC: Copy> Ord for SearchNode<S, C, DC> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.cost + self.heuristic == other.cost + other.heuristic {
            return other.cost.cmp(&self.cost); // Estimation is more precise when the cost is larger
        } else {
            (self.cost + self.heuristic).cmp(&(other.cost + other.heuristic))
        }
    }
}
