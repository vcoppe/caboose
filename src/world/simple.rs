use std::sync::Arc;

use chrono::Duration;

use crate::{
    Graph, GraphEdgeId, GraphNodeId, Heuristic, ReverseResumableAStar, Task, Time, Timed,
    TransitionSystem,
};

/// A world simply described by a directed weighted graph
pub struct SimpleWorld {
    graph: Arc<Graph>,
}

impl SimpleWorld {
    pub fn new(graph: Arc<Graph>) -> Self {
        SimpleWorld { graph }
    }

    pub fn time(&self, edge: GraphEdgeId) -> Duration {
        let edge = self.graph.get_edge(edge);
        let from = self.graph.get_node(edge.from);
        let to = self.graph.get_node(edge.to);
        let dx = to.position.0 - from.position.0;
        let dy = to.position.1 - from.position.1;
        let distance = (dx * dx + dy * dy).sqrt();
        Duration::milliseconds((distance * 1000.0).round() as i64)
    }
}

// Definitions that model a basic transition system that ignores the time dimension

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(Clone)]
pub struct SimpleTask {
    initial_state: Arc<SimpleState>,
    goal_state: Arc<SimpleState>,
}

impl Task<SimpleState> for SimpleTask {
    fn new(initial_state: Arc<SimpleState>, goal_state: Arc<SimpleState>) -> Self {
        SimpleTask {
            initial_state,
            goal_state,
        }
    }

    fn initial_state(&self) -> Arc<SimpleState> {
        self.initial_state.clone()
    }

    fn goal_state(&self) -> Arc<SimpleState> {
        self.goal_state.clone()
    }
}

pub struct SimpleHeuristic {
    graph: Arc<Graph>,
    goal_state: Arc<SimpleState>,
}

impl Heuristic<SimpleWorld, SimpleState, GraphEdgeId, Time, Duration, SimpleTask>
    for SimpleHeuristic
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

    fn get_heuristic(&mut self, state: Arc<SimpleState>) -> Option<Duration> {
        let goal_node = self.graph.get_node(self.goal_state.0);
        let state_node = self.graph.get_node(state.0);
        let dx = goal_node.position.0 - state_node.position.0;
        let dy = goal_node.position.1 - state_node.position.1;
        let distance = (dx * dx + dy * dy).sqrt();
        Some(Duration::milliseconds((distance * 1000.0).round() as i64))
    }
}

// Definitions that model a basic transition system with a time dimension

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct SimpleTimedState {
    pub node: GraphNodeId,
    pub time: Time,
}

impl Timed for SimpleTimedState {
    fn get_time(&self) -> Time {
        self.time
    }

    fn set_time(&mut self, time: Time) {
        self.time = time;
    }
}

impl TransitionSystem<SimpleTimedState, GraphEdgeId, Duration> for SimpleWorld {
    fn actions_from(&self, state: Arc<SimpleTimedState>) -> std::slice::Iter<GraphEdgeId> {
        self.graph.get_edges_out(state.node).iter()
    }

    fn transition(&self, state: Arc<SimpleTimedState>, action: &GraphEdgeId) -> SimpleTimedState {
        let edge = self.graph.get_edge(*action);
        SimpleTimedState {
            node: edge.to,
            time: state.time + self.time(*action),
        }
    }

    fn transition_cost(&self, _state: Arc<SimpleTimedState>, action: &GraphEdgeId) -> Duration {
        Duration::milliseconds((self.graph.get_edge(*action).distance * 1000.0).round() as i64)
    }

    fn reverse_actions_from(&self, state: Arc<SimpleTimedState>) -> std::slice::Iter<GraphEdgeId> {
        self.graph.get_edges_in(state.node).iter()
    }

    fn reverse_transition(
        &self,
        state: Arc<SimpleTimedState>,
        action: &GraphEdgeId,
    ) -> SimpleTimedState {
        let edge = self.graph.get_edge(*action);
        SimpleTimedState {
            node: edge.from,
            time: state.time + self.time(*action),
        }
    }

    fn reverse_transition_cost(
        &self,
        _state: Arc<SimpleTimedState>,
        action: &GraphEdgeId,
    ) -> Duration {
        Duration::milliseconds((self.graph.get_edge(*action).distance * 1000.0).round() as i64)
    }
}

#[derive(Clone)]
pub struct SimpleTimedTask {
    initial_state: Arc<SimpleTimedState>,
    goal_state: Arc<SimpleTimedState>,
}

impl Task<SimpleTimedState> for SimpleTimedTask {
    fn new(initial_state: Arc<SimpleTimedState>, goal_state: Arc<SimpleTimedState>) -> Self {
        SimpleTimedTask {
            initial_state,
            goal_state,
        }
    }

    fn initial_state(&self) -> Arc<SimpleTimedState> {
        self.initial_state.clone()
    }

    fn goal_state(&self) -> Arc<SimpleTimedState> {
        self.goal_state.clone()
    }

    fn is_goal_state(&self, state: &SimpleTimedState) -> bool {
        state.node == self.goal_state().node
    }
}

pub struct SimpleTimedHeuristic {
    graph: Arc<Graph>,
    goal_state: Arc<SimpleTimedState>,
}

impl Heuristic<SimpleWorld, SimpleTimedState, GraphEdgeId, Time, Duration, SimpleTimedTask>
    for SimpleTimedHeuristic
{
    fn new(transition_system: Arc<SimpleWorld>, task: Arc<SimpleTimedTask>) -> Self
    where
        Self: Sized,
    {
        SimpleTimedHeuristic {
            graph: transition_system.graph.clone(),
            goal_state: task.goal_state(),
        }
    }

    fn get_heuristic(&mut self, state: Arc<SimpleTimedState>) -> Option<Duration> {
        let goal_node = self.graph.get_node(self.goal_state.node);
        let state_node = self.graph.get_node(state.node);
        let dx = goal_node.position.0 - state_node.position.0;
        let dy = goal_node.position.1 - state_node.position.1;
        let distance = (dx * dx + dy * dy).sqrt();
        Some(Duration::milliseconds((distance * 1000.0).round() as i64))
    }
}

pub struct TimedHeuristic(
    ReverseResumableAStar<
        SimpleWorld,
        SimpleState,
        GraphEdgeId,
        Time,
        Duration,
        SimpleTask,
        SimpleHeuristic,
    >,
);

impl Heuristic<SimpleWorld, SimpleTimedState, GraphEdgeId, Time, Duration, SimpleTimedTask>
    for TimedHeuristic
{
    fn new(transition_system: Arc<SimpleWorld>, task: Arc<SimpleTimedTask>) -> Self
    where
        Self: Sized,
    {
        let task = Arc::new(SimpleTask {
            initial_state: Arc::new(SimpleState(task.initial_state.node)),
            goal_state: Arc::new(SimpleState(task.goal_state.node)),
        });
        TimedHeuristic(ReverseResumableAStar::new(transition_system, task))
    }

    fn get_heuristic(&mut self, state: Arc<SimpleTimedState>) -> Option<Duration> {
        self.0.get_heuristic(Arc::new(SimpleState(state.node)))
    }
}
