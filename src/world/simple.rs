use std::{
    ops::{Add, Div, Sub},
    sync::Arc,
};

use chrono::{DateTime, Duration, Local, Utc};
use ncollide2d::{
    na::{Point2, Vector2},
    query,
    shape::Ball,
};
use tuple::A2;

use crate::{
    Graph, GraphEdgeId, GraphNodeId, Heuristic, LimitValues, Move, State, Task, TransitionSystem,
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

    fn conflict(&self, moves: A2<&Move<SimpleState, GraphEdgeId, MyTime>>) -> bool {
        let initial_time = moves[0].interval.start.max(moves[1].interval.start);
        let max_time = moves[0].interval.end.min(moves[1].interval.end) - initial_time;
        let mut centers = vec![];
        let mut vels = vec![];
        let mut balls = vec![];

        for i in 0..=1 {
            let interval = &moves[i].interval;
            let from = self.graph.get_node(moves[i].from.0).position;
            let to = self.graph.get_node(moves[i].to.0).position;

            let d_x = to.0 - from.0;
            let d_y = to.1 - from.1;
            let d_t = 0.001 * (interval.end.0 - interval.start.0).num_milliseconds() as f64;
            let vel_x = d_x / d_t;
            let vel_y = d_y / d_t;

            let pre_d_t = 0.001 * (initial_time.0 - interval.start.0).num_milliseconds() as f64;

            let center_x = from.0 + vel_x * pre_d_t;
            let center_y = from.1 + vel_y * pre_d_t;

            let ball = Ball::new(0.4);

            centers.push(Point2::new(center_x, center_y));
            vels.push(Vector2::new(vel_x, vel_y));
            balls.push(ball);
        }

        let toi = query::time_of_impact_ball_ball(
            &centers[0],
            &vels[0],
            &balls[0],
            &centers[1],
            &vels[1],
            &balls[1],
            0.001 * max_time.0.num_milliseconds() as f64,
            0.0,
        );

        toi.is_some()
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

impl Div<i32> for MyDuration {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        MyDuration(self.0 / rhs)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::{Duration, Local, TimeZone};
    use tuple::T2;

    use crate::{
        Graph, GraphEdgeId, GraphNodeId, Interval, Move, MyDuration, MyTime, SimpleState,
        SimpleWorld, TransitionSystem,
    };

    fn simple_graph(size: usize) -> Arc<Graph> {
        let mut graph = Graph::new();
        for x in 0..size {
            for y in 0..size {
                graph.add_node((x as f64, y as f64), 1.0);
            }
        }
        for x in 0..size {
            for y in 0..size {
                let node_id = GraphNodeId(x + y * size);
                if x > 0 {
                    graph.add_edge(node_id, GraphNodeId(x - 1 + y * size), 1.0, 1.0);
                }
                if y > 0 {
                    graph.add_edge(node_id, GraphNodeId(x + (y - 1) * size), 1.0, 1.0);
                }
                if x < size - 1 {
                    graph.add_edge(node_id, GraphNodeId(x + 1 + y * size), 1.0, 1.0);
                }
                if y < size - 1 {
                    graph.add_edge(node_id, GraphNodeId(x + (y + 1) * size), 1.0, 1.0);
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

        let initial_time = MyTime(Local.with_ymd_and_hms(2000, 01, 01, 10, 0, 0).unwrap());

        let move1 = Move {
            agent: 0,
            action: Some(GraphEdgeId(0)),
            from: Arc::new(SimpleState(GraphNodeId(0))),
            to: Arc::new(SimpleState(GraphNodeId(1))),
            interval: Interval::new(
                initial_time,
                initial_time + MyDuration(Duration::seconds(1)),
            ),
        };

        let move2 = Move {
            agent: 1,
            action: Some(GraphEdgeId(0)),
            from: Arc::new(SimpleState(GraphNodeId(1))),
            to: Arc::new(SimpleState(GraphNodeId(0))),
            interval: Interval::new(
                initial_time,
                initial_time + MyDuration(Duration::seconds(1)),
            ),
        };

        assert!(transition_system.conflict(T2(&move1, &move2)));
    }
}
