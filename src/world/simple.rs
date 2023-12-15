use std::sync::Arc;

use ncollide2d::{
    na::{Point2, Vector2},
    query,
    shape::Ball,
};
use ordered_float::OrderedFloat;
use tuple::A2;

use crate::{
    Graph, GraphEdgeId, GraphNodeId, Heuristic, LimitValues, MinimalHeuristic, Move, State, Task,
    TransitionSystem,
};

pub type MyTime = OrderedFloat<f32>;
pub type SimpleNodeData = (f32, f32);
pub type SimpleEdgeData = f32;

/// A world simply described by a directed weighted graph
pub struct SimpleWorld {
    graph: Arc<Graph<SimpleNodeData, SimpleEdgeData>>,
}

impl SimpleWorld {
    const BALL: Ball<f32> = Ball { radius: 0.4 };

    pub fn new(graph: Arc<Graph<SimpleNodeData, SimpleEdgeData>>) -> Self {
        SimpleWorld { graph }
    }

    pub fn time_between(&self, from: GraphNodeId, to: GraphNodeId) -> MyTime {
        let from = self.graph.get_node(from);
        let to = self.graph.get_node(to);
        let dx = to.data.0 - from.data.0;
        let dy = to.data.1 - from.data.1;
        (dx * dx + dy * dy).sqrt().into()
    }

    pub fn time(&self, edge: GraphEdgeId) -> MyTime {
        let edge = self.graph.get_edge(edge);
        self.time_between(edge.from, edge.to)
    }

    pub fn get_center_and_vel(
        &self,
        m: &Move<SimpleState, GraphEdgeId, MyTime>,
        initial_time: &MyTime,
    ) -> (Point2<f32>, Vector2<f32>) {
        let interval = &m.interval;
        let from = self.graph.get_node(m.from.0).data;
        let to = self.graph.get_node(m.to.0).data;

        let d_x = to.0 - from.0;
        let d_y = to.1 - from.1;
        let d_t = interval.end.0 - interval.start.0;
        let vel_x = d_x / d_t;
        let vel_y = d_y / d_t;

        let pre_d_t = initial_time.0 - interval.start.0;

        let center_x = from.0 + vel_x * pre_d_t;
        let center_y = from.1 + vel_y * pre_d_t;

        (Point2::new(center_x, center_y), Vector2::new(vel_x, vel_y))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct SimpleState(pub GraphNodeId);

impl State for SimpleState {
    fn is_equivalent(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl TransitionSystem<SimpleState, GraphEdgeId, MyTime, MyTime> for SimpleWorld {
    fn actions_from(&self, state: &SimpleState) -> std::slice::Iter<GraphEdgeId> {
        self.graph.get_edges_out(state.0).iter()
    }

    fn transition(&self, _state: &SimpleState, action: &GraphEdgeId) -> SimpleState {
        SimpleState(self.graph.get_edge(*action).to)
    }

    fn transition_cost(&self, _state: &SimpleState, action: &GraphEdgeId) -> MyTime {
        self.time(*action)
    }

    fn reverse_actions_from(&self, state: &SimpleState) -> std::slice::Iter<GraphEdgeId> {
        self.graph.get_edges_in(state.0).iter()
    }

    fn reverse_transition(&self, _state: &SimpleState, action: &GraphEdgeId) -> SimpleState {
        SimpleState(self.graph.get_edge(*action).from)
    }

    fn reverse_transition_cost(&self, _state: &SimpleState, action: &GraphEdgeId) -> MyTime {
        self.time(*action)
    }

    fn can_wait_at(&self, _state: &SimpleState) -> bool {
        true
    }

    fn conflict(&self, moves: A2<&Move<SimpleState, GraphEdgeId, MyTime>>) -> bool {
        let initial_time = moves[0].interval.start.max(moves[1].interval.start);
        let max_time = moves[0].interval.end.min(moves[1].interval.end) - initial_time;

        let (center1, vel1) = self.get_center_and_vel(moves[0], &initial_time);
        let (center2, vel2) = self.get_center_and_vel(moves[1], &initial_time);

        let toi = query::time_of_impact_ball_ball(
            &center1,
            &vel1,
            &Self::BALL,
            &center2,
            &vel2,
            &Self::BALL,
            max_time.0,
            0.0,
        );

        toi.is_some()
    }
}

pub struct SimpleHeuristic {
    transition_system: Arc<SimpleWorld>,
    goal_state: SimpleState,
}

impl SimpleHeuristic {
    pub fn new(transition_system: Arc<SimpleWorld>, task: Arc<Task<SimpleState, MyTime>>) -> Self {
        SimpleHeuristic {
            transition_system,
            goal_state: task.goal_state.clone(),
        }
    }
}

impl Heuristic<SimpleWorld, SimpleState, GraphEdgeId, MyTime, MyTime> for SimpleHeuristic {
    fn get_heuristic(&self, state: &SimpleState) -> Option<MyTime> {
        Some(
            self.transition_system
                .time_between(state.0, self.goal_state.0),
        )
    }
}

impl MinimalHeuristic<SimpleWorld, SimpleState, GraphEdgeId, MyTime, MyTime> for SimpleHeuristic {
    fn build(transition_system: Arc<SimpleWorld>, task: Arc<Task<SimpleState, MyTime>>) -> Self {
        Self::new(transition_system, task)
    }
}

impl LimitValues for MyTime {
    fn min_value() -> Self {
        f32::MIN.into()
    }

    fn max_value() -> Self {
        f32::MAX.into()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ordered_float::OrderedFloat;
    use tuple::T2;

    use crate::{
        Graph, GraphEdgeId, GraphNodeId, Interval, Move, SimpleEdgeData, SimpleNodeData,
        SimpleState, SimpleWorld, TransitionSystem,
    };

    fn simple_graph(size: usize) -> Arc<Graph<SimpleNodeData, SimpleEdgeData>> {
        let mut graph = Graph::new();
        for x in 0..size {
            for y in 0..size {
                graph.add_node((x as f32, y as f32));
            }
        }
        for x in 0..size {
            for y in 0..size {
                let node_id = GraphNodeId(x + y * size);
                if x > 0 {
                    graph.add_edge(node_id, GraphNodeId(x - 1 + y * size), 1.0);
                }
                if y > 0 {
                    graph.add_edge(node_id, GraphNodeId(x + (y - 1) * size), 1.0);
                }
                if x < size - 1 {
                    graph.add_edge(node_id, GraphNodeId(x + 1 + y * size), 1.0);
                }
                if y < size - 1 {
                    graph.add_edge(node_id, GraphNodeId(x + (y + 1) * size), 1.0);
                }
            }
        }
        Arc::new(graph)
    }

    #[test]
    fn test_simple() {
        let size = 10;
        let graph = simple_graph(size);
        let transition_system = Arc::new(SimpleWorld::new(graph));

        let initial_time = OrderedFloat(0.0);

        let move1 = Move {
            agent: 0,
            action: Some(GraphEdgeId(0)),
            from: SimpleState(GraphNodeId(0)),
            to: SimpleState(GraphNodeId(1)),
            interval: Interval::new(initial_time, initial_time + 1.0),
        };

        let move2 = Move {
            agent: 1,
            action: Some(GraphEdgeId(0)),
            from: SimpleState(GraphNodeId(1)),
            to: SimpleState(GraphNodeId(0)),
            interval: Interval::new(initial_time, initial_time + 1.0),
        };

        assert!(transition_system.conflict(T2(&move1, &move2)));
    }
}
