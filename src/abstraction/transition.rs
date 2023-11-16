use std::slice;

use tuple::A2;

use crate::{Interval, LimitValues};

/// Definition of a state in a transition system.
pub trait State {
    /// Returns true if the state is mostly equivalent to the other state
    /// when checking whether it is a goal state.
    fn is_equivalent(&self, other: &Self) -> bool;
}

/// Definition of a transition system that contains a set of states and actions,
/// and transition functions that describe the result of any action applied to any state.
/// The reverse transitions must also be described to allow using the reverse search as a heuristic.
pub trait TransitionSystem<S, A, C, DC>
where
    C: Ord + LimitValues,
{
    fn actions_from(&self, state: &S) -> slice::Iter<A>;

    fn transition(&self, state: &S, action: &A) -> S;
    fn transition_cost(&self, state: &S, action: &A) -> DC;

    fn reverse_actions_from(&self, state: &S) -> slice::Iter<A>;

    fn reverse_transition(&self, state: &S, action: &A) -> S;
    fn reverse_transition_cost(&self, state: &S, action: &A) -> DC;

    fn can_wait_at(&self, state: &S) -> bool;

    /// Returns true if the two moves lead to a collision.
    fn conflict(&self, moves: A2<&Move<S, A, C>>) -> bool;
}

/// Definition of a callback that can be used to apply actions to a transition system.
pub trait ActionCallback<Action> {
    fn apply(&mut self, action: Action);
}

impl<Action, X: FnMut(Action)> ActionCallback<Action> for X {
    fn apply(&mut self, action: Action) {
        self(action)
    }
}

/// Definition of a task in a given transition system that can then
/// be fed to a search algorithm.
pub struct Task<S, C>
where
    S: State + Eq + Clone,
    C: Copy,
{
    pub initial_state: S,
    pub goal_state: S,
    pub initial_cost: C,
}

impl<S, C> Task<S, C>
where
    S: State + Eq + Clone,
    C: Copy,
{
    pub fn new(initial_state: S, goal_state: S, initial_cost: C) -> Self {
        Self {
            initial_state,
            goal_state,
            initial_cost,
        }
    }

    pub fn is_goal_state(&self, state: &S) -> bool {
        state.is_equivalent(&self.goal_state)
    }

    pub fn reverse(&self) -> Self {
        Self {
            initial_state: self.goal_state.clone(),
            goal_state: self.initial_state.clone(),
            initial_cost: self.initial_cost,
        }
    }
}

/// Definition of a move in a transition system.
#[derive(Debug, Clone)]
pub struct Move<S, A, C>
where
    C: Ord + LimitValues,
{
    pub agent: usize,
    pub from: S,
    pub to: S,
    pub action: Option<A>,
    pub interval: Interval<C>,
}

impl<S, A, C> Move<S, A, C>
where
    C: Ord + LimitValues,
{
    pub fn new(agent: usize, from: S, to: S, action: Option<A>, interval: Interval<C>) -> Self {
        Self {
            agent,
            from,
            to,
            action,
            interval,
        }
    }
}
