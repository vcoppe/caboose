use std::{
    hash::Hash,
    marker::PhantomData,
    ops::{Add, Sub},
    sync::Arc,
};

use crate::{State, Task, TransitionSystem};

/// Defines a time interval (start <= end).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Interval<C, DC>
where
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues + Sub<C, Output = DC> + Copy,
{
    /// The start of the interval.
    pub start: C,
    /// The end of the interval.
    pub end: C,
}

impl<C, DC> Default for Interval<C, DC>
where
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues + Sub<C, Output = DC> + Copy,
{
    fn default() -> Self {
        Self::new(C::min_value(), C::max_value())
    }
}

impl<C, DC> Interval<C, DC>
where
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues + Sub<C, Output = DC> + Copy,
{
    /// Creates a new interval with the given start and end.
    pub fn new(start: C, end: C) -> Self {
        Self { start, end }
    }

    /// Returns true if the interval overlaps with the given interval.
    pub fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    /// Returns true if the interval contains the given interval.
    pub fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    /// Returns the length of the interval.
    pub fn length(&self) -> DC {
        self.end - self.start
    }
}

/// Wrapper around an action that also contains the cost of the action.
#[derive(Debug, Clone, Copy)]
pub struct Action<A, DC> {
    /// The action, of None if the agent waits.
    pub action: Option<A>,
    /// The cost (or duration) of the action.
    pub cost: DC,
}

impl<A, DC> Action<A, DC> {
    /// Creates a new action with the given action and cost.
    ///
    /// # Arguments
    ///
    /// * `action` - The action.
    /// * `cost` - The cost (or duration) of the action.
    pub fn new(action: A, cost: DC) -> Self {
        Self {
            action: Some(action),
            cost,
        }
    }

    /// Creates a new wait action with the given cost.
    ///
    /// # Arguments
    ///
    /// * `cost` - The cost (or duration) of the wait action.
    pub fn wait(cost: DC) -> Self {
        Self { action: None, cost }
    }
}

/// Description of a solution to a search problem
#[derive(Debug, Clone)]
pub struct Solution<S, A, C, DC>
where
    C: Default,
{
    /// The cost of the solution.
    pub cost: C,
    /// The sequence of states that the agent visits, and the associated cost (timestamp).
    pub steps: Vec<(S, C)>,
    /// The sequence of actions that the agent performs, and the associated cost (duration).
    pub actions: Vec<Action<A, DC>>,
}

impl<S, A, C, DC> Default for Solution<S, A, C, DC>
where
    C: Default,
{
    fn default() -> Self {
        Self {
            cost: C::default(),
            steps: Default::default(),
            actions: Default::default(),
        }
    }
}

/// Defines a heuristic function that can be used by a search algorithm,
/// for a given transition system and task.
pub trait Heuristic<TS, S, A, C, DC>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: Hash + Eq + Clone,
    C: Eq
        + PartialOrd
        + Ord
        + Add<DC, Output = C>
        + Sub<C, Output = DC>
        + Copy
        + Default
        + LimitValues,
{
    /// Returns the heuristic value for the given state,
    /// or None if the goal state is not reachable from that state.
    fn get_heuristic(&self, state: &S) -> Option<DC>;
}

/// Trait to build heuristics on the fly.
pub trait HeuristicBuilder<TS, S, A, C, DC>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: State + Hash + Eq + Clone,
    C: Eq
        + PartialOrd
        + Ord
        + Add<DC, Output = C>
        + Sub<C, Output = DC>
        + Copy
        + Default
        + LimitValues,
{
    /// Builds a new heuristic for the given transition system and task.
    ///
    /// # Arguments
    ///
    /// * `transition_system` - The transition system in which the agent navigates.
    /// * `task` - The task to solve.
    fn build(transition_system: Arc<TS>, task: Arc<Task<S, C>>) -> Self;
}

/// Differential heuristic built on top of heuristics dealing with
/// time and durations.
pub struct DifferentialHeuristic<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: State + Hash + Eq + Clone,
    C: Ord + Add<DC, Output = C> + Sub<C, Output = DC> + Copy + Default + LimitValues,
    H: Heuristic<TS, S, A, C, DC>,
{
    /// The task to solve.
    task: Arc<Task<S, C>>,
    /// The heuristics injected in the differential heuristic.
    heuristic_to_pivots: Arc<Vec<Arc<H>>>,
    /// The index of the heuristic associated to the goal state, if any.
    task_heuristic: Option<usize>,
    _phantom: PhantomData<(TS, S, A, DC)>,
}

impl<TS, S, A, C, DC, H> DifferentialHeuristic<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: State + Hash + Eq + Clone,
    C: Ord + Add<DC, Output = C> + Sub<C, Output = DC> + Copy + Default + LimitValues,
    DC: Ord + Sub<DC, Output = DC> + Copy,
    H: Heuristic<TS, S, A, C, DC>,
{
    /// Creates a new differential heuristic for the given task, that exploits
    /// a series of existing heuristics for the given pivots.
    ///
    /// # Arguments
    ///
    /// * `task` - The task to solve.
    /// * `pivots` - The pivots for which heuristics have been initialized.
    /// * `heuristic_to_pivots` - The heuristics associated to the pivots.
    pub fn new(
        task: Arc<Task<S, C>>,
        pivots: Arc<Vec<S>>,
        heuristic_to_pivots: Arc<Vec<Arc<H>>>,
    ) -> Self {
        DifferentialHeuristic {
            task_heuristic: pivots
                .iter()
                .position(|pivot| pivot.is_equivalent(&task.goal_state)),
            task,
            heuristic_to_pivots,
            _phantom: PhantomData,
        }
    }
}

impl<TS, S, A, C, DC, H> Heuristic<TS, S, A, C, DC> for DifferentialHeuristic<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: State + Hash + Eq + Clone,
    C: Ord + Add<DC, Output = C> + Sub<C, Output = DC> + Copy + Default + LimitValues,
    DC: Ord + Sub<DC, Output = DC> + Copy,
    H: Heuristic<TS, S, A, C, DC>,
{
    fn get_heuristic(&self, state: &S) -> Option<DC> {
        if let Some(task_heuristic) = self.task_heuristic {
            self.heuristic_to_pivots[task_heuristic].get_heuristic(state)
        } else {
            let mut heuristic = C::default() - C::default();
            for heuristic_to_pivot in self.heuristic_to_pivots.iter() {
                if let (Some(h1), Some(h2)) = (
                    heuristic_to_pivot.get_heuristic(state),
                    heuristic_to_pivot.get_heuristic(&self.task.goal_state),
                ) {
                    heuristic = heuristic.max(h2 - h1).max(h1 - h2)
                }
            }
            Some(heuristic)
        }
    }
}

/// Trait to specify the minimum and maximum values of a type.
pub trait LimitValues {
    fn min_value() -> Self;
    fn max_value() -> Self;
}
