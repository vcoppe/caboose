use std::sync::Arc;

use ordered_float::OrderedFloat;

use crate::{Graph, GraphEdgeId, GraphNodeId, Heuristic, Task, TransitionSystem};

pub struct SimpleWorld {
    graph: Arc<Graph>,
}

impl SimpleWorld {
    pub fn new(graph: Arc<Graph>) -> Self {
        SimpleWorld { graph }
    }
}

impl TransitionSystem<GraphNodeId, GraphEdgeId, OrderedFloat<f64>> for SimpleWorld {
    fn for_each_action(
        &self,
        state: Arc<GraphNodeId>,
        f: &mut dyn crate::ActionCallback<GraphEdgeId>,
    ) {
        self.graph
            .get_edges_out(*state)
            .iter()
            .for_each(|edge| f.apply(*edge));
    }

    fn transition(&self, _state: Arc<GraphNodeId>, action: &GraphEdgeId) -> GraphNodeId {
        self.graph.get_edge(*action).to
    }

    fn transition_cost(&self, _state: Arc<GraphNodeId>, action: &GraphEdgeId) -> OrderedFloat<f64> {
        OrderedFloat(self.graph.get_edge(*action).distance)
    }

    fn for_each_reverse_action(
        &self,
        state: Arc<GraphNodeId>,
        f: &mut dyn crate::ActionCallback<GraphEdgeId>,
    ) {
        self.graph
            .get_edges_in(*state)
            .iter()
            .for_each(|edge| f.apply(*edge));
    }

    fn reverse_transition(&self, _state: Arc<GraphNodeId>, action: &GraphEdgeId) -> GraphNodeId {
        self.graph.get_edge(*action).from
    }

    fn reverse_transition_cost(
        &self,
        _state: Arc<GraphNodeId>,
        action: &GraphEdgeId,
    ) -> OrderedFloat<f64> {
        OrderedFloat(self.graph.get_edge(*action).distance)
    }
}

#[derive(Clone)]
pub struct SimpleTask {
    initial_state: GraphNodeId,
    goal_state: GraphNodeId,
}

impl SimpleTask {
    pub fn new(initial_state: GraphNodeId, goal_state: GraphNodeId) -> Self {
        SimpleTask {
            initial_state,
            goal_state,
        }
    }
}

impl Task<GraphNodeId> for SimpleTask {
    fn initial_state(&self) -> GraphNodeId {
        self.initial_state
    }

    fn goal_state(&self) -> GraphNodeId {
        self.goal_state
    }
}

pub struct SimpleHeuristic {
    graph: Arc<Graph>,
    goal_state: GraphNodeId,
}

impl
    Heuristic<
        SimpleWorld,
        GraphNodeId,
        GraphEdgeId,
        OrderedFloat<f64>,
        OrderedFloat<f64>,
        SimpleTask,
    > for SimpleHeuristic
{
    fn new(transition_system: Arc<SimpleWorld>, task: Arc<SimpleTask>) -> Self
    where
        Self: Sized,
    {
        SimpleHeuristic {
            graph: transition_system.graph.clone(),
            goal_state: task.goal_state(),
        }
    }

    fn get_heuristic(&mut self, state: Arc<GraphNodeId>) -> Option<OrderedFloat<f64>> {
        let goal_node = self.graph.get_node(self.goal_state);
        let state_node = self.graph.get_node(*state);
        let dx = goal_node.position.0 - state_node.position.0;
        let dy = goal_node.position.1 - state_node.position.1;
        let distance = (dx * dx + dy * dy).sqrt();
        Some(OrderedFloat(distance))
    }
}
