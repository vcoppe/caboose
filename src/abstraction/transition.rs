use std::{slice, sync::Arc};

/// Definition of a state in a transition system.
pub trait State {
    /// Returns true if the state is mostly equivalent to the other state
    /// when checking whether it is a goal state.
    fn is_equivalent(&self, other: &Self) -> bool;
}

/// Definition of a transition system that contains a set of states and actions,
/// and transition functions that describe the result of any action applied to any state.
/// The reverse transitions must also be described to allow using the reverse search as a heuristic.
pub trait TransitionSystem<State, Action, Cost> {
    fn actions_from(&self, state: Arc<State>) -> slice::Iter<Action>;

    fn transition(&self, state: Arc<State>, action: &Action) -> State;
    fn transition_cost(&self, state: Arc<State>, action: &Action) -> Cost;

    fn reverse_actions_from(&self, state: Arc<State>) -> slice::Iter<Action>;

    fn reverse_transition(&self, state: Arc<State>, action: &Action) -> State;
    fn reverse_transition_cost(&self, state: Arc<State>, action: &Action) -> Cost;

    fn can_wait_at(&self, state: Arc<State>) -> bool;
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
pub struct Task<S: State + Eq> {
    initial_state: Arc<S>,
    goal_state: Arc<S>,
}

impl<S: State + Eq> Task<S> {
    pub fn new(initial_state: Arc<S>, goal_state: Arc<S>) -> Self {
        Self {
            initial_state,
            goal_state,
        }
    }

    pub fn initial_state(&self) -> Arc<S> {
        self.initial_state.clone()
    }

    pub fn goal_state(&self) -> Arc<S> {
        self.goal_state.clone()
    }

    pub fn is_goal_state(&self, state: &S) -> bool {
        state.is_equivalent(&self.goal_state)
    }
}
