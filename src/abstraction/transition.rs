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
pub trait TransitionSystem<S, A, C, DC> {
    fn actions_from(&self, state: Arc<S>) -> slice::Iter<A>;

    fn transition(&self, state: Arc<S>, action: &A) -> S;
    fn transition_cost(&self, state: Arc<S>, action: &A) -> DC;

    fn reverse_actions_from(&self, state: Arc<S>) -> slice::Iter<A>;

    fn reverse_transition(&self, state: Arc<S>, action: &A) -> S;
    fn reverse_transition_cost(&self, state: Arc<S>, action: &A) -> DC;

    fn can_wait_at(&self, state: Arc<S>) -> bool;

    fn conflict(&self, moves: (&Move<S, A, C, DC>, &Move<S, A, C, DC>)) -> bool;
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
    S: State + Eq,
{
    pub initial_state: Arc<S>,
    pub goal_state: Arc<S>,
    pub initial_cost: C,
}

impl<S, C> Task<S, C>
where
    S: State + Eq,
{
    pub fn new(initial_state: Arc<S>, goal_state: Arc<S>, initial_cost: C) -> Self {
        Self {
            initial_state,
            goal_state,
            initial_cost,
        }
    }

    pub fn is_goal_state(&self, state: &S) -> bool {
        state.is_equivalent(&self.goal_state)
    }
}

/// Definition of a move in a transition system.
pub struct Move<S, A, C, DC> {
    pub agent: usize,
    pub from: Arc<S>,
    pub to: Arc<S>,
    pub action: A,
    pub time: C,
    pub duration: DC,
}
