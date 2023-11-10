use std::{
    ops::{Add, Sub},
    sync::Arc,
};

use chrono::{DateTime, Duration, Local, Utc};
use tuple::A2;

use crate::{
    Constraint, Graph, GraphEdgeId, GraphNodeId, Heuristic, LimitValues, Move, State, Task,
    TransitionSystem,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct MyTime(pub DateTime<Local>);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct MyDuration(pub Duration);

/// A world simply described by a directed weighted graph
pub struct SimpleWorld {
    graph: Arc<Graph>,
}

impl SimpleWorld {
    pub fn new(graph: Arc<Graph>) -> Self {
        SimpleWorld { graph }
    }

    pub fn time_between(&self, from: GraphNodeId, to: GraphNodeId) -> MyDuration {
        let from = self.graph.get_node(from);
        let to = self.graph.get_node(to);
        let dx = to.position.0 - from.position.0;
        let dy = to.position.1 - from.position.1;
        let distance = (dx * dx + dy * dy).sqrt();
        MyDuration(Duration::milliseconds((distance * 1000.0).round() as i64))
    }

    pub fn time(&self, edge: GraphEdgeId) -> MyDuration {
        let edge = self.graph.get_edge(edge);
        self.time_between(edge.from, edge.to)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct SimpleState(pub GraphNodeId);

impl State for SimpleState {
    fn is_equivalent(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl TransitionSystem<SimpleState, GraphEdgeId, MyTime, MyDuration> for SimpleWorld {
    fn actions_from(&self, state: Arc<SimpleState>) -> std::slice::Iter<GraphEdgeId> {
        self.graph.get_edges_out(state.0).iter()
    }

    fn transition(&self, _state: Arc<SimpleState>, action: &GraphEdgeId) -> SimpleState {
        SimpleState(self.graph.get_edge(*action).to)
    }

    fn transition_cost(&self, _state: Arc<SimpleState>, action: &GraphEdgeId) -> MyDuration {
        self.time(*action)
    }

    fn reverse_actions_from(&self, state: Arc<SimpleState>) -> std::slice::Iter<GraphEdgeId> {
        self.graph.get_edges_in(state.0).iter()
    }

    fn reverse_transition(&self, _state: Arc<SimpleState>, action: &GraphEdgeId) -> SimpleState {
        SimpleState(self.graph.get_edge(*action).from)
    }

    fn reverse_transition_cost(
        &self,
        _state: Arc<SimpleState>,
        action: &GraphEdgeId,
    ) -> MyDuration {
        self.time(*action)
    }

    fn can_wait_at(&self, _state: Arc<SimpleState>) -> bool {
        true
    }

    fn conflict(&self, _moves: A2<&Move<SimpleState, GraphEdgeId, MyTime, MyDuration>>) -> bool {
        todo!("Implement conflict detection for SimpleWorld")
    }

    fn get_constraint(
        &self,
        _moves: A2<&Move<SimpleState, GraphEdgeId, MyTime, MyDuration>>,
    ) -> Constraint<SimpleState, MyTime> {
        todo!("Implement constraint generation for SimpleWorld")
    }
}

pub struct SimpleHeuristic {
    transition_system: Arc<SimpleWorld>,
    goal_state: Arc<SimpleState>,
}

impl SimpleHeuristic {
    pub fn new(transition_system: Arc<SimpleWorld>, task: Arc<Task<SimpleState, MyTime>>) -> Self {
        SimpleHeuristic {
            transition_system,
            goal_state: task.goal_state.clone(),
        }
    }
}

impl Heuristic<SimpleWorld, SimpleState, GraphEdgeId, MyTime, MyDuration> for SimpleHeuristic {
    fn get_heuristic(&self, state: Arc<SimpleState>) -> Option<MyDuration> {
        Some(
            self.transition_system
                .time_between(state.0, self.goal_state.0),
        )
    }
}

impl LimitValues for MyTime {
    fn min_value() -> Self {
        MyTime(DateTime::<Utc>::MIN_UTC.into())
    }

    fn max_value() -> Self {
        MyTime((DateTime::<Utc>::MAX_UTC - Duration::days(1)).into())
    }
}

impl Default for MyTime {
    fn default() -> Self {
        MyTime(DateTime::<Utc>::MIN_UTC.into())
    }
}

impl Default for MyDuration {
    fn default() -> Self {
        MyDuration(Duration::zero())
    }
}

impl Add<MyDuration> for MyTime {
    type Output = Self;

    fn add(self, rhs: MyDuration) -> Self::Output {
        MyTime(self.0 + rhs.0)
    }
}

impl Sub<MyDuration> for MyTime {
    type Output = Self;

    fn sub(self, rhs: MyDuration) -> Self::Output {
        MyTime(self.0 - rhs.0)
    }
}

impl Sub<MyTime> for MyTime {
    type Output = MyDuration;

    fn sub(self, rhs: MyTime) -> Self::Output {
        MyDuration(self.0 - rhs.0)
    }
}

impl Sub<MyDuration> for MyDuration {
    type Output = Self;

    fn sub(self, rhs: MyDuration) -> Self::Output {
        MyDuration(self.0 - rhs.0)
    }
}
