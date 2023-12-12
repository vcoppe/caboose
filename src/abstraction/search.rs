use std::{
    cmp::Ordering,
    hash::Hash,
    marker::PhantomData,
    ops::{Add, Sub},
    rc::Rc,
    sync::Arc,
};

use fxhash::FxHashMap;
use tuple::A2;

use crate::{Move, State, Task, TransitionSystem};

/// Wrapper around an action that also contains the cost of the action.
#[derive(Debug, Clone, Copy)]
pub struct Action<A, DC> {
    pub action: Option<A>,
    pub cost: DC,
}

impl<A, DC> Action<A, DC> {
    pub fn new(action: A, cost: DC) -> Self {
        Self {
            action: Some(action),
            cost,
        }
    }

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
    pub states: Vec<S>,
    pub costs: Vec<C>,
    pub actions: Vec<Action<A, DC>>,
}

impl<S, A, C, DC> Default for Solution<S, A, C, DC>
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
    S: Hash + Eq + Clone,
    C: Eq + PartialOrd + Ord + Add<DC, Output = C> + Copy + Default + LimitValues,
{
    /// Returns the heuristic value for the given state,
    /// or None if the goal state is not reachable from that state.
    fn get_heuristic(&self, state: &S) -> Option<DC>;
}

/// Trait to build simple heuristics on the fly.
pub trait MinimalHeuristic<TS, S, A, C, DC>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: State + Hash + Eq + Clone,
    C: Eq + PartialOrd + Ord + Add<DC, Output = C> + Copy + Default + LimitValues,
{
    fn build(transition_system: Arc<TS>, task: Arc<Task<S, C>>) -> Self;
}

/// Differentiable heuristic built on top of heuristics dealing with
/// time and durations.
pub struct DifferentialHeuristic<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: State + Hash + Eq + Clone,
    C: Ord + Add<DC, Output = C> + Sub<C, Output = DC> + Copy + Default + LimitValues,
    H: Heuristic<TS, S, A, C, DC>,
{
    task: Arc<Task<S, C>>,
    heuristic_to_pivots: Arc<Vec<Arc<H>>>,
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
            _phantom: PhantomData::default(),
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

/// Generic definition of a search node and the associated ordering functions
/// that allow performing best-first searches by ordering nodes by increasing
/// (cost + heuristic) values, with a tie-breaking favoring nodes with higher cost.
#[derive(Debug, Clone)]
pub struct SearchNode<S, C, DC>
where
    C: Copy + Eq + Ord + Add<DC, Output = C>,
    DC: Copy,
{
    pub state: Rc<S>,
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
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<S, C, DC> Ord for SearchNode<S, C, DC>
where
    C: Copy + Eq + Ord + Add<DC, Output = C>,
    DC: Copy,
{
    fn cmp(&self, other: &Self) -> Ordering {
        (self.cost + self.heuristic)
            .cmp(&(other.cost + other.heuristic))
            .then_with(|| {
                self.cost.cmp(&other.cost).reverse() // Estimation is more precise when the cost is larger
            })
    }
}

/// Definition of the different conflict types.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ConflictType {
    /// Conflict involving a frozen agent
    Frozen,
    /// Solving this conflicts delays both agents
    Cardinal,
    /// Solving this conflicts delays one agent
    SemiCardinal,
    /// The conflict can be solved without delaying any agent
    NonCardinal,
}

/// Definition of a conflict between two moves.
#[derive(Debug)]
pub struct Conflict<S, A, C, DC>
where
    C: Ord + LimitValues,
    DC: Ord + Default,
{
    pub moves: A2<Move<S, A, C>>,
    pub type_: ConflictType,
    pub overcost: DC,
}

impl<S, A, C, DC> Conflict<S, A, C, DC>
where
    C: Ord + LimitValues,
    DC: Ord + Default,
{
    pub fn new(moves: A2<Move<S, A, C>>) -> Self {
        Self {
            moves,
            type_: ConflictType::NonCardinal,
            overcost: DC::default(),
        }
    }
}

impl<S, A, C, DC> PartialEq for Conflict<S, A, C, DC>
where
    C: Ord + Copy + LimitValues,
    DC: Ord + Default,
{
    fn eq(&self, other: &Self) -> bool {
        self.type_ == other.type_
            && self.overcost == other.overcost
            && self.moves.0.interval.start.min(self.moves.1.interval.start)
                == other
                    .moves
                    .0
                    .interval
                    .start
                    .min(other.moves.1.interval.start)
    }
}

impl<S, A, C, DC> Eq for Conflict<S, A, C, DC>
where
    C: Ord + Copy + LimitValues,
    DC: Ord + Default,
{
}

impl<S, A, C, DC> PartialOrd for Conflict<S, A, C, DC>
where
    C: Ord + Copy + LimitValues,
    DC: Ord + Default,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<S, A, C, DC> Ord for Conflict<S, A, C, DC>
where
    C: Ord + Copy + LimitValues,
    DC: Ord + Default,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.type_.cmp(&other.type_).then_with(|| {
            self.overcost.cmp(&other.overcost).reverse().then_with(|| {
                self.moves
                    .0
                    .interval
                    .start
                    .min(self.moves.1.interval.start)
                    .cmp(
                        &&other
                            .moves
                            .0
                            .interval
                            .start
                            .min(other.moves.1.interval.start),
                    )
            })
        })
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

    pub fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }
}

/// The types of constraints that can be imposed on agents in a search algorithm.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ConstraintType {
    /// Constraint that prevents an agent from visiting the given state during a given interval.
    State,
    /// Constraint that prevents an agent from connecting the two given states during a given interval.
    Action,
}

/// Defines a constraint that can be imposed on a given agent in a search algorithm.
#[derive(Debug, Clone)]
pub struct Constraint<S, C>
where
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues,
{
    pub agent: usize,
    pub state: S,
    pub next: Option<S>,
    pub interval: Interval<C>,
    pub type_: ConstraintType,
}

impl<S, C> Constraint<S, C>
where
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues,
{
    pub fn new_state_constraint(agent: usize, state: S, interval: Interval<C>) -> Self {
        Self {
            agent,
            state,
            next: None,
            interval,
            type_: ConstraintType::State,
        }
    }
    pub fn new_action_constraint(agent: usize, state: S, next: S, interval: Interval<C>) -> Self {
        Self {
            agent,
            state,
            next: Some(next),
            interval,
            type_: ConstraintType::Action,
        }
    }
}

impl<S, C> PartialEq for Constraint<S, C>
where
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues,
{
    fn eq(&self, other: &Self) -> bool {
        self.interval == other.interval
    }
}

impl<S, C> Eq for Constraint<S, C> where C: PartialEq + Eq + PartialOrd + Ord + LimitValues {}

impl<S, C> PartialOrd for Constraint<S, C>
where
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<S, C> Ord for Constraint<S, C>
where
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.interval.cmp(&other.interval)
    }
}

/// Set of constraints that can be imposed on agents in a search algorithm.
#[derive(Debug)]
pub struct ConstraintSet<S, C>
where
    S: State + Eq + Hash + Clone,
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues + Copy,
{
    pub state_constraints: FxHashMap<S, Vec<Constraint<S, C>>>,
    pub action_constraints: FxHashMap<(S, S), Vec<Constraint<S, C>>>,
}

impl<S, C> Default for ConstraintSet<S, C>
where
    S: State + Eq + Hash + Clone,
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues + Copy,
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
    S: State + Eq + Hash + Clone,
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues + Copy,
{
    pub fn add(&mut self, constraint: &Arc<Constraint<S, C>>) {
        match constraint.type_ {
            ConstraintType::State => {
                self.state_constraints
                    .entry(constraint.state.clone())
                    .or_default()
                    .push(constraint.as_ref().clone());
            }
            ConstraintType::Action => {
                self.action_constraints
                    .entry((
                        constraint.state.clone(),
                        constraint.next.as_ref().unwrap().clone(),
                    ))
                    .or_default()
                    .push(constraint.as_ref().clone());
            }
        }
    }

    pub fn get_state_constraints(&self, state: &S) -> Option<&Vec<Constraint<S, C>>> {
        self.state_constraints.get(state)
    }

    pub fn get_action_constraints(&self, from: &S, to: &S) -> Option<&Vec<Constraint<S, C>>> {
        self.action_constraints.get(&(from.clone(), to.clone()))
    }

    pub fn unify(&mut self) {
        for constraints in self.state_constraints.values_mut() {
            constraints.sort_unstable();

            let mut unified_constraints = vec![];

            let mut i = 0;
            while i < constraints.len() {
                let mut constraint = constraints[i].clone();

                let mut j = i + 1;
                while j < constraints.len()
                    && constraint.interval.overlaps(&constraints[j].interval)
                {
                    constraint.interval.end =
                        constraint.interval.end.max(constraints[j].interval.end);
                    j += 1;
                }

                unified_constraints.push(constraint);
                i = j + 1;
            }

            *constraints = unified_constraints;
        }

        for constraints in self.action_constraints.values_mut() {
            constraints.sort_unstable();

            let mut unified_constraints = vec![];

            let mut i = 0;
            while i < constraints.len() {
                let mut constraint = constraints[i].clone();

                let mut j = i + 1;
                while j < constraints.len()
                    && constraint.interval.overlaps(&constraints[j].interval)
                {
                    constraint.interval.end =
                        constraint.interval.end.max(constraints[j].interval.end);
                    j += 1;
                }

                unified_constraints.push(constraint);
                i = j + 1;
            }

            *constraints = unified_constraints;
        }
    }
}

pub type LandmarkSet<S, C> = Vec<Arc<Constraint<S, C>>>;
