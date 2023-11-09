use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap},
    hash::Hash,
    marker::PhantomData,
    ops::Add,
    sync::Arc,
};

use chrono::Duration;

use crate::{Interval, State, Task, Time, TransitionSystem};

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

/// Constraint that prevents an agent from visiting the given state
/// during a given interval.
#[derive(Debug)]
pub struct StateConstraint<S>
where
    S: State,
{
    pub state: Arc<S>,
    pub interval: Interval,
}

impl<S> StateConstraint<S>
where
    S: State,
{
    pub fn new(state: Arc<S>, interval: Interval) -> Self {
        Self { state, interval }
    }
}

impl<S> PartialEq for StateConstraint<S>
where
    S: State,
{
    fn eq(&self, other: &Self) -> bool {
        self.interval == other.interval
    }
}

impl<S> Eq for StateConstraint<S> where S: State {}

impl<S> PartialOrd for StateConstraint<S>
where
    S: State,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.interval.partial_cmp(&other.interval)
    }
}

impl<S> Ord for StateConstraint<S>
where
    S: State,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.interval.cmp(&other.interval)
    }
}

/// Constraint that prevents an agent from connecting the two given states
/// during a given interval.
pub struct ActionConstraint<S>
where
    S: State,
{
    pub from: Arc<S>,
    pub to: Arc<S>,
    pub interval: Interval,
}

impl<S> ActionConstraint<S>
where
    S: State,
{
    pub fn new(from: Arc<S>, to: Arc<S>, interval: Interval) -> Self {
        Self { from, to, interval }
    }
}

impl<S> PartialEq for ActionConstraint<S>
where
    S: State,
{
    fn eq(&self, other: &Self) -> bool {
        self.interval == other.interval
    }
}

impl<S> Eq for ActionConstraint<S> where S: State {}

impl<S> PartialOrd for ActionConstraint<S>
where
    S: State,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.interval.partial_cmp(&other.interval)
    }
}

impl<S> Ord for ActionConstraint<S>
where
    S: State,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.interval.cmp(&other.interval)
    }
}

/// Set of constraints that can be used by a search algorithm.
pub struct ConstraintSet<S>
where
    S: State + Eq + Hash,
{
    state_constraints: HashMap<Arc<S>, BTreeSet<StateConstraint<S>>>,
    action_constraints: HashMap<(Arc<S>, Arc<S>), BTreeSet<ActionConstraint<S>>>,
}

impl<S> Default for ConstraintSet<S>
where
    S: State + Eq + Hash,
{
    fn default() -> Self {
        Self {
            state_constraints: Default::default(),
            action_constraints: Default::default(),
        }
    }
}

impl<S> ConstraintSet<S>
where
    S: State + Eq + Hash,
{
    pub fn add_state_constraint(&mut self, constraint: StateConstraint<S>) {
        self.state_constraints
            .entry(constraint.state.clone())
            .or_default()
            .insert(constraint);
    }

    pub fn add_action_constraint(&mut self, constraint: ActionConstraint<S>) {
        self.action_constraints
            .entry((constraint.from.clone(), constraint.to.clone()))
            .or_default()
            .insert(constraint);
    }

    pub fn get_state_constraints(&self, state: &Arc<S>) -> Option<&BTreeSet<StateConstraint<S>>> {
        self.state_constraints.get(state)
    }

    pub fn get_action_constraints(
        &self,
        from: &Arc<S>,
        to: &Arc<S>,
    ) -> Option<&BTreeSet<ActionConstraint<S>>> {
        self.action_constraints.get(&(from.clone(), to.clone()))
    }
}
