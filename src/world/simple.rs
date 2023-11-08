use std::sync::Arc;

use chrono::Duration;

use crate::{Graph, GraphEdgeId, GraphNodeId, Heuristic, Task, Time, TransitionSystem};

/// A world simply described by a directed weighted graph
pub struct SimpleWorld {
    graph: Arc<Graph>,
}

impl SimpleWorld {
    pub fn new(graph: Arc<Graph>) -> Self {
        SimpleWorld { graph }
    }

    pub fn time_between(&self, from: GraphNodeId, to: GraphNodeId) -> Duration {
        let from = self.graph.get_node(from);
        let to = self.graph.get_node(to);
        let dx = to.position.0 - from.position.0;
        let dy = to.position.1 - from.position.1;
        let distance = (dx * dx + dy * dy).sqrt();
        Duration::milliseconds((distance * 1000.0).round() as i64)
    }

    pub fn time(&self, edge: GraphEdgeId) -> Duration {
        let edge = self.graph.get_edge(edge);
        self.time_between(edge.from, edge.to)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct SimpleState(pub GraphNodeId);

impl TransitionSystem<SimpleState, GraphEdgeId, Duration> for SimpleWorld {
    fn actions_from(&self, state: Arc<SimpleState>) -> std::slice::Iter<GraphEdgeId> {
        self.graph.get_edges_out(state.0).iter()
    }

    fn transition(&self, _state: Arc<SimpleState>, action: &GraphEdgeId) -> SimpleState {
        SimpleState(self.graph.get_edge(*action).to)
    }

    fn transition_cost(&self, _state: Arc<SimpleState>, action: &GraphEdgeId) -> Duration {
        Duration::milliseconds((self.graph.get_edge(*action).distance * 1000.0).round() as i64)
    }

    fn reverse_actions_from(&self, state: Arc<SimpleState>) -> std::slice::Iter<GraphEdgeId> {
        self.graph.get_edges_in(state.0).iter()
    }

    fn reverse_transition(&self, _state: Arc<SimpleState>, action: &GraphEdgeId) -> SimpleState {
        SimpleState(self.graph.get_edge(*action).from)
    }

    fn reverse_transition_cost(&self, _state: Arc<SimpleState>, action: &GraphEdgeId) -> Duration {
        Duration::milliseconds((self.graph.get_edge(*action).distance * 1000.0).round() as i64)
    }
}

pub struct SimpleHeuristic {
    transition_system: Arc<SimpleWorld>,
    goal_state: Arc<SimpleState>,
}

impl SimpleHeuristic {
    pub fn new(transition_system: Arc<SimpleWorld>, task: Arc<Task<SimpleState>>) -> Self {
        SimpleHeuristic {
            transition_system,
            goal_state: task.goal_state(),
        }
    }
}

impl Heuristic<SimpleWorld, SimpleState, GraphEdgeId, Time, Duration> for SimpleHeuristic {
    fn get_heuristic(&self, state: Arc<SimpleState>) -> Option<Duration> {
        Some(
            self.transition_system
                .time_between(state.0, self.goal_state.0),
        )
    }
}
