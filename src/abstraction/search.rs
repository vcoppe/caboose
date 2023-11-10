use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap},
    hash::Hash,
    marker::PhantomData,
    ops::{Add, Sub},
    sync::Arc,
};

use crate::{Move, State, Task, TransitionSystem};

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
pub struct DifferentialHeuristic<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: State + Hash + Eq,
    C: Ord + Add<DC, Output = C> + Sub<C, Output = DC> + Copy + Default,
    H: Heuristic<TS, S, A, C, DC>,
{
    task: Arc<Task<S, C>>,
    pivots: Arc<Vec<Arc<S>>>,
    heuristic_to_pivots: Arc<Vec<Arc<H>>>,
    _phantom: PhantomData<(TS, S, A, DC)>,
}

impl<TS, S, A, C, DC, H> DifferentialHeuristic<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: State + Hash + Eq,
    C: Ord + Add<DC, Output = C> + Sub<C, Output = DC> + Copy + Default,
    DC: Ord + Sub<DC, Output = DC> + Copy,
    H: Heuristic<TS, S, A, C, DC>,
{
    pub fn new(
        task: Arc<Task<S, C>>,
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

impl<TS, S, A, C, DC, H> Heuristic<TS, S, A, C, DC> for DifferentialHeuristic<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: State + Hash + Eq,
    C: Ord + Add<DC, Output = C> + Sub<C, Output = DC> + Copy + Default,
    DC: Ord + Sub<DC, Output = DC> + Copy,
    H: Heuristic<TS, S, A, C, DC>,
{
    fn get_heuristic(&self, state: Arc<S>) -> Option<DC> {
        let mut heuristic = C::default() - C::default();
        for (pivot, heuristic_to_pivot) in self.pivots.iter().zip(self.heuristic_to_pivots.iter()) {
            if pivot.is_equivalent(self.task.goal_state.as_ref()) {
                if let Some(h) = heuristic_to_pivot.get_heuristic(state.clone()) {
                    heuristic = heuristic.max(h);
                }
            } else if let (Some(h1), Some(h2)) = (
                heuristic_to_pivot.get_heuristic(state.clone()),
                heuristic_to_pivot.get_heuristic(self.task.goal_state.clone()),
            ) {
                heuristic = heuristic.max(h2 - h1).max(h1 - h2)
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

/// Trait to specify the minimum and maximum values of a type.
pub trait LimitValues {
    fn min_value() -> Self;
    fn max_value() -> Self;
}

/// Defines a time interval (start <= end).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Interval<C>
where
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues,
{
    pub start: C,
    pub end: C,
}

impl<C> Default for Interval<C>
where
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues,
{
    fn default() -> Self {
        Self::new(C::min_value(), C::max_value())
    }
}

impl<C> Interval<C>
where
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues,
{
    pub fn new(start: C, end: C) -> Self {
        Self { start, end }
    }
}

/// Constraint that prevents an agent from visiting the given state
/// during a given interval.
#[derive(Debug)]
pub struct StateConstraint<S, C>
where
    S: State,
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues,
{
    pub state: Arc<S>,
    pub interval: Interval<C>,
}

impl<S, C> StateConstraint<S, C>
where
    S: State,
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues,
{
    pub fn new(state: Arc<S>, interval: Interval<C>) -> Self {
        Self { state, interval }
    }
}

impl<S, C> PartialEq for StateConstraint<S, C>
where
    S: State,
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues,
{
    fn eq(&self, other: &Self) -> bool {
        self.interval == other.interval
    }
}

impl<S, C> Eq for StateConstraint<S, C>
where
    S: State,
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues,
{
}

impl<S, C> PartialOrd for StateConstraint<S, C>
where
    S: State,
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.interval.partial_cmp(&other.interval)
    }
}

impl<S, C> Ord for StateConstraint<S, C>
where
    S: State,
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.interval.cmp(&other.interval)
    }
}

/// Constraint that prevents an agent from connecting the two given states
/// during a given interval.
pub struct ActionConstraint<S, C>
where
    S: State,
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues,
{
    pub from: Arc<S>,
    pub to: Arc<S>,
    pub interval: Interval<C>,
}

impl<S, C> ActionConstraint<S, C>
where
    S: State,
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues,
{
    pub fn new(from: Arc<S>, to: Arc<S>, interval: Interval<C>) -> Self {
        Self { from, to, interval }
    }
}

impl<S, C> PartialEq for ActionConstraint<S, C>
where
    S: State,
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues,
{
    fn eq(&self, other: &Self) -> bool {
        self.interval == other.interval
    }
}

impl<S, C> Eq for ActionConstraint<S, C>
where
    S: State,
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues,
{
}

impl<S, C> PartialOrd for ActionConstraint<S, C>
where
    S: State,
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.interval.partial_cmp(&other.interval)
    }
}

impl<S, C> Ord for ActionConstraint<S, C>
where
    S: State,
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.interval.cmp(&other.interval)
    }
}

/// Set of constraints that can be used by a search algorithm.
pub struct ConstraintSet<S, C>
where
    S: State + Eq + Hash,
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues,
{
    state_constraints: HashMap<Arc<S>, BTreeSet<StateConstraint<S, C>>>,
    action_constraints: HashMap<(Arc<S>, Arc<S>), BTreeSet<ActionConstraint<S, C>>>,
}

impl<S, C> Default for ConstraintSet<S, C>
where
    S: State + Eq + Hash,
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues,
{
    fn default() -> Self {
        Self {
            state_constraints: Default::default(),
            action_constraints: Default::default(),
        }
    }
}

impl<S, C> ConstraintSet<S, C>
where
    S: State + Eq + Hash,
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues,
{
    pub fn add_state_constraint(&mut self, constraint: StateConstraint<S, C>) {
        self.state_constraints
            .entry(constraint.state.clone())
            .or_default()
            .insert(constraint);
    }

    pub fn add_action_constraint(&mut self, constraint: ActionConstraint<S, C>) {
        self.action_constraints
            .entry((constraint.from.clone(), constraint.to.clone()))
            .or_default()
            .insert(constraint);
    }

    pub fn get_state_constraints(
        &self,
        state: &Arc<S>,
    ) -> Option<&BTreeSet<StateConstraint<S, C>>> {
        self.state_constraints.get(state)
    }

    pub fn get_action_constraints(
        &self,
        from: &Arc<S>,
        to: &Arc<S>,
    ) -> Option<&BTreeSet<ActionConstraint<S, C>>> {
        self.action_constraints.get(&(from.clone(), to.clone()))
    }
}
