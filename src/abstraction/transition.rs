use std::{ops::Sub, slice};

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
    C: Ord + LimitValues + Sub<C, Output = DC> + Copy,
{
    /// Returns the actions that can be applied from the given state.
    fn actions_from(&self, state: &S) -> slice::Iter<A>;

    /// Returns the state resulting from applying the given action to the given state.
    fn transition(&self, state: &S, action: &A) -> S;

    /// Returns the cost of applying the given action to the given state (i.e. the duration of the action).
    fn transition_cost(&self, state: &S, action: &A) -> DC;

    /// Returns the actions that can be applied to reach the given state.
    fn reverse_actions_from(&self, state: &S) -> slice::Iter<A>;

    /// Returns the state resulting from applying the given reverse action to the given state.
    fn reverse_transition(&self, state: &S, action: &A) -> S;

    /// Returns the cost of applying the given reverse action to the given state (i.e. the duration of the action).
    fn reverse_transition_cost(&self, state: &S, action: &A) -> DC;

    /// Returns true if agents can wait at the given state.
    fn can_wait_at(&self, state: &S) -> bool;

    /// Returns true if the two moves lead to a collision.
    fn conflict(&self, moves: A2<&Move<S, A, C, DC>>) -> bool;
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
    /// The initial state of the agent.
    pub initial_state: S,
    /// The goal state that the agent must reach.
    pub goal_state: S,
    /// The initial cost of the agent.
    pub initial_cost: C,
}

impl<S, C> Task<S, C>
where
    S: State + Eq + Clone,
    C: Copy,
{
    /// Creates a new task.
    ///
    /// # Arguments
    ///
    /// * `initial_state` - The initial state of the agent.
    /// * `goal_state` - The goal state that the agent must reach.
    /// * `initial_cost` - The initial cost of the agent.
    pub fn new(initial_state: S, goal_state: S, initial_cost: C) -> Self {
        Self {
            initial_state,
            goal_state,
            initial_cost,
        }
    }

    /// Returns true if the given state is a goal state for the task.
    pub fn is_goal_state(&self, state: &S) -> bool {
        state.is_equivalent(&self.goal_state)
    }

    /// Returns the reverse task.
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
pub struct Move<S, A, C, DC>
where
    C: Ord + LimitValues + Sub<C, Output = DC> + Copy,
{
    /// The agent that performs the move.
    pub agent: usize,
    /// The state the agent moves from.
    pub from: S,
    /// The state the agent moves to.
    pub to: S,
    /// The action the agent performs, or None if the agent waits.
    pub action: Option<A>,
    /// The interval during which the agent performs the action.
    pub interval: Interval<C, DC>,
}

impl<S, A, C, DC> Move<S, A, C, DC>
where
    C: Ord + LimitValues + Sub<C, Output = DC> + Copy,
{
    /// Creates a new move.
    ///
    /// # Arguments
    ///
    /// * `agent` - The agent that performs the move.
    /// * `from` - The state the agent moves from.
    /// * `to` - The state the agent moves to.
    /// * `action` - The action the agent performs, or None if the agent waits.
    /// * `interval` - The interval during which the agent performs the action.
    pub fn new(agent: usize, from: S, to: S, action: Option<A>, interval: Interval<C, DC>) -> Self {
        Self {
            agent,
            from,
            to,
            action,
            interval,
        }
    }
}
