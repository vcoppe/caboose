use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap},
    hash::Hash,
    marker::PhantomData,
    ops::Add,
    sync::Arc,
};

use chrono::Duration;

use crate::{Interval, Move, State, Task, Time, TransitionSystem};

/// Description of a solution to a search problem
#[derive(Debug, Clone)]
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
    TS: TransitionSystem<S, A, C, DC>,
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
    TS: TransitionSystem<S, A, Time, Duration>,
    S: State + Hash + Eq,
    H: Heuristic<TS, S, A, Time, Duration>,
{
    task: Arc<Task<S, Time>>,
    pivots: Arc<Vec<Arc<S>>>,
    heuristic_to_pivots: Arc<Vec<Arc<H>>>,
    _phantom: PhantomData<(TS, S, A)>,
}

impl<TS, S, A, H> DifferentialHeuristic<TS, S, A, H>
where
    TS: TransitionSystem<S, A, Time, Duration>,
    S: State + Hash + Eq,
    H: Heuristic<TS, S, A, Time, Duration>,
{
    pub fn new(
        task: Arc<Task<S, Time>>,
        pivots: Arc<Vec<Arc<S>>>,
        heuristic_to_pivots: Arc<Vec<Arc<H>>>,
    ) -> Self {
        DifferentialHeuristic {
            task,
            pivots,
            heuristic_to_pivots,
            _phantom: PhantomData::default(),
        }
    }
}

impl<TS, S, A, H> Heuristic<TS, S, A, Time, Duration> for DifferentialHeuristic<TS, S, A, H>
where
    TS: TransitionSystem<S, A, Time, Duration>,
    S: State + Hash + Eq,
    H: Heuristic<TS, S, A, Time, Duration>,
{
    fn get_heuristic(&self, state: Arc<S>) -> Option<Duration> {
        let mut heuristic = Duration::zero();
        for (pivot, heuristic_to_pivot) in self.pivots.iter().zip(self.heuristic_to_pivots.iter()) {
            if pivot.is_equivalent(self.task.goal_state.as_ref()) {
                if let Some(h) = heuristic_to_pivot.get_heuristic(state.clone()) {
                    heuristic = heuristic.max(h);
                }
            } else if let (Some(h1), Some(h2)) = (
                heuristic_to_pivot.get_heuristic(state.clone()),
                heuristic_to_pivot.get_heuristic(self.task.goal_state.clone()),
            ) {
                heuristic = heuristic.max((h2 - h1).abs());
            }
        }
        Some(heuristic)
    }
}

/// Generic definition of a search node and the associated ordering functions
/// that allow performing best-first searches by ordering nodes by increasing
/// (cost + heuristic) values, with a tie-breaking favoring nodes with higher cost.
#[derive(Debug, Clone)]
pub struct SearchNode<S, C, DC>
where
    C: Copy + Eq + Ord + Add<DC, Output = C>,
    DC: Copy,
{
    pub state: Arc<S>,
    pub cost: C,
    pub heuristic: DC,
}

impl<S, C, DC> PartialEq for SearchNode<S, C, DC>
where
    C: Copy + Eq + Ord + Add<DC, Output = C>,
    DC: Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.cost + self.heuristic == other.cost + other.heuristic
    }
}

impl<S, C, DC> Eq for SearchNode<S, C, DC>
where
    C: Copy + Eq + Ord + Add<DC, Output = C>,
    DC: Copy,
{
}

impl<S, C, DC> PartialOrd for SearchNode<S, C, DC>
where
    C: Copy + Eq + Ord + Add<DC, Output = C>,
    DC: Copy,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.cost + self.heuristic == other.cost + other.heuristic {
            return other.cost.partial_cmp(&self.cost); // Estimation is more precise when the cost is larger
        } else {
            (self.cost + self.heuristic).partial_cmp(&(other.cost + other.heuristic))
        }
    }
}

impl<S, C, DC> Ord for SearchNode<S, C, DC>
where
    C: Copy + Eq + Ord + Add<DC, Output = C>,
    DC: Copy,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.cost + self.heuristic == other.cost + other.heuristic {
            return other.cost.cmp(&self.cost); // Estimation is more precise when the cost is larger
        } else {
            (self.cost + self.heuristic).cmp(&(other.cost + other.heuristic))
        }
    }
}

/// Definition of the different conflict types.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ConflictType {
    /// Solving this conflicts delays both agents
    Cardinal,
    /// Solving this conflicts delays one agent
    SemiCardinal,
    /// The conflict can be solved without delaying any agent
    NonCardinal,
}

/// Definition of a conflict between two moves.
pub struct Conflict<S, A, C, DC>
where
    C: Ord,
{
    pub moves: (Arc<Move<S, A, C, DC>>, Arc<Move<S, A, C, DC>>),
    pub type_: ConflictType,
}

// TODO: also take into account the overcost of solving the conflict
impl<S, A, C, DC> Conflict<S, A, C, DC>
where
    C: Ord,
{
    pub fn new(
        moves: (Arc<Move<S, A, C, DC>>, Arc<Move<S, A, C, DC>>),
        type_: ConflictType,
    ) -> Self {
        Self { moves, type_ }
    }
}

impl<S, A, C, DC> PartialEq for Conflict<S, A, C, DC>
where
    C: Ord + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.type_ == other.type_
            && self.moves.0.time.min(self.moves.1.time)
                == other.moves.0.time.min(other.moves.1.time)
    }
}

impl<S, A, C, DC> Eq for Conflict<S, A, C, DC> where C: Ord + Copy {}

impl<S, A, C, DC> PartialOrd for Conflict<S, A, C, DC>
where
    C: Ord + Copy,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.type_ == other.type_ {
            self.moves
                .0
                .time
                .min(self.moves.1.time)
                .partial_cmp(&other.moves.0.time.min(other.moves.1.time))
        } else {
            self.type_.partial_cmp(&other.type_)
        }
    }
}

impl<S, A, C, DC> Ord for Conflict<S, A, C, DC>
where
    C: Ord + Copy,
{
    fn cmp(&self, other: &Self) -> Ordering {
        if self.type_ == other.type_ {
            self.moves
                .0
                .time
                .min(self.moves.1.time)
                .cmp(&other.moves.0.time.min(other.moves.1.time))
        } else {
            self.type_.cmp(&other.type_)
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
