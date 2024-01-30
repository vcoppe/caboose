use std::{
    cmp::Ordering,
    hash::Hash,
    ops::{Add, Sub},
    sync::Arc,
};

use fxhash::FxHashMap;
use tuple::A2;

use crate::{Interval, LimitValues, Move, State};

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
    C: Ord + LimitValues + Sub<C, Output = DC> + Copy,
    DC: Ord + Default,
{
    pub moves: A2<Move<S, A, C, DC>>,
    pub type_: ConflictType,
    pub overcost: DC,
}

impl<S, A, C, DC> Conflict<S, A, C, DC>
where
    C: Ord + LimitValues + Sub<C, Output = DC> + Copy,
    DC: Ord + Default,
{
    pub fn new(moves: A2<Move<S, A, C, DC>>) -> Self {
        Self {
            moves,
            type_: ConflictType::NonCardinal,
            overcost: DC::default(),
        }
    }
}

impl<S, A, C, DC> PartialEq for Conflict<S, A, C, DC>
where
    C: Ord + Copy + LimitValues + Sub<C, Output = DC>,
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
    C: Ord + Copy + LimitValues + Sub<C, Output = DC>,
    DC: Ord + Default,
{
}

impl<S, A, C, DC> PartialOrd for Conflict<S, A, C, DC>
where
    C: Ord + Copy + LimitValues + Sub<C, Output = DC>,
    DC: Ord + Default,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<S, A, C, DC> Ord for Conflict<S, A, C, DC>
where
    C: Ord + Copy + LimitValues + Sub<C, Output = DC>,
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
                        &other
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
pub struct Constraint<S, C, DC>
where
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues + Sub<C, Output = DC> + Copy,
    DC: PartialEq + Eq + PartialOrd + Ord,
{
    pub agent: usize,
    pub state: S,
    pub next: Option<S>,
    pub interval: Interval<C, DC>,
    pub type_: ConstraintType,
}

impl<S, C, DC> Constraint<S, C, DC>
where
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues + Sub<C, Output = DC> + Copy,
    DC: PartialEq + Eq + PartialOrd + Ord,
{
    pub fn new_state_constraint(agent: usize, state: S, interval: Interval<C, DC>) -> Self {
        Self {
            agent,
            state,
            next: None,
            interval,
            type_: ConstraintType::State,
        }
    }
    pub fn new_action_constraint(
        agent: usize,
        state: S,
        next: S,
        interval: Interval<C, DC>,
    ) -> Self {
        Self {
            agent,
            state,
            next: Some(next),
            interval,
            type_: ConstraintType::Action,
        }
    }
}

impl<S, C, DC> PartialEq for Constraint<S, C, DC>
where
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues + Sub<C, Output = DC> + Copy,
    DC: PartialEq + Eq + PartialOrd + Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.interval == other.interval
    }
}

impl<S, C, DC> Eq for Constraint<S, C, DC>
where
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues + Sub<C, Output = DC> + Copy,
    DC: PartialEq + Eq + PartialOrd + Ord,
{
}

impl<S, C, DC> PartialOrd for Constraint<S, C, DC>
where
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues + Sub<C, Output = DC> + Copy,
    DC: PartialEq + Eq + PartialOrd + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<S, C, DC> Ord for Constraint<S, C, DC>
where
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues + Sub<C, Output = DC> + Copy,
    DC: PartialEq + Eq + PartialOrd + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.interval.cmp(&other.interval)
    }
}

/// Set of constraints that can be imposed on agents in a search algorithm.
#[derive(Debug)]
pub struct ConstraintSet<S, C, DC>
where
    S: State + Eq + Hash + Clone,
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues + Copy + Sub<C, Output = DC>,
    DC: PartialEq + Eq + PartialOrd + Ord + Copy,
{
    pub state_constraints: FxHashMap<S, Vec<Constraint<S, C, DC>>>,
    pub action_constraints: FxHashMap<(S, S), Vec<Constraint<S, C, DC>>>,
}

impl<S, C, DC> Default for ConstraintSet<S, C, DC>
where
    S: State + Eq + Hash + Clone,
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues + Copy + Sub<C, Output = DC>,
    DC: PartialEq + Eq + PartialOrd + Ord + Copy,
{
    fn default() -> Self {
        Self {
            state_constraints: Default::default(),
            action_constraints: Default::default(),
        }
    }
}

impl<S, C, DC> ConstraintSet<S, C, DC>
where
    S: State + Eq + Hash + Clone,
    C: PartialEq + Eq + PartialOrd + Ord + LimitValues + Copy + Sub<C, Output = DC>,
    DC: PartialEq + Eq + PartialOrd + Ord + Copy,
{
    pub fn add(&mut self, constraint: &Arc<Constraint<S, C, DC>>) {
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

    pub fn get_state_constraints(&self, state: &S) -> Option<&Vec<Constraint<S, C, DC>>> {
        self.state_constraints.get(state)
    }

    pub fn get_action_constraints(&self, from: &S, to: &S) -> Option<&Vec<Constraint<S, C, DC>>> {
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

pub type LandmarkSet<S, C, DC> = Vec<Arc<Constraint<S, C, DC>>>;
