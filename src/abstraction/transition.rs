use std::sync::Arc;

use crate::Time;

/// Definition of a transition system that contains a set of states and actions,
/// and transition functions that describe the result of any action applied to any state.
/// The reverse transitions must also be described to allow using the reverse search as a heuristic.
pub trait TransitionSystem<State, Action, Cost> {
    fn for_each_action(&self, state: Arc<State>, f: &mut dyn ActionCallback<Action>);

    fn transition(&self, state: Arc<State>, action: &Action) -> State;
    fn transition_cost(&self, state: Arc<State>, action: &Action) -> Cost;

    fn for_each_reverse_action(&self, state: Arc<State>, f: &mut dyn ActionCallback<Action>);

    fn reverse_transition(&self, state: Arc<State>, action: &Action) -> State;
    fn reverse_transition_cost(&self, state: Arc<State>, action: &Action) -> Cost;
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
pub trait Task<State: Eq> {
    fn initial_state(&self) -> State;
    fn goal_state(&self) -> State;
    fn is_goal_state(&self, state: Arc<State>) -> bool {
        *state == self.goal_state()
    }
}

/// Trait for states that have a time dimension.
pub trait Timed {
    fn get_time(&self) -> Time;

    fn set_time(&mut self, time: Time);
}
