use std::{hash::Hash, ops::Add, sync::Arc};

use crate::{Task, TransitionSystem};

/// Generic definition of a search algorithm that finds the optimal sequence of actions
/// to complete a given task in a given transition system, using a given heuristic.
pub trait Solver<TS, S, A, C, DC, T, H>
where
    TS: TransitionSystem<S, A, DC>,
    S: Hash + Eq,
    C: Eq + PartialOrd + Ord + Add<DC, Output = C> + Copy + Default,
    T: Task<S>,
    H: Heuristic<TS, S, A, C, DC, T>,
{
    /// Returns the optimal sequence of actions to complete the given task in the given transition system,
    /// or None if there is no way of completing the task.
    fn solve(&mut self, task: Arc<T>) -> Option<Vec<A>>;
}

/// Defines a heuristic function that can be used by a search algorithm,
/// for a given transition system and task.
pub trait Heuristic<TS, S, A, C, DC, T>
where
    TS: TransitionSystem<S, A, DC>,
    S: Hash + Eq,
    C: Eq + PartialOrd + Ord + Add<DC, Output = C> + Copy + Default,
    T: Task<S>,
{
    /// Creates the heuristic for a given task
    fn new(transition_system: Arc<TS>, task: Arc<T>) -> Self
    where
        Self: Sized;

    /// Returns the heuristic value for the given state,
    /// or None if the goal state is not reachable from that state.
    fn get_heuristic(&mut self, state: Arc<S>) -> Option<DC>;
}

/// Generic definition of a search node and the associated ordering functions
/// that allow performing best-first searches by ordering nodes by increasing
/// (cost + heuristic) values, with a tie-breaking favoring nodes with higher cost.
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
