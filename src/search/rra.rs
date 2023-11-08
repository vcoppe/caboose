use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Add;
use std::sync::{Arc, Mutex, RwLock};
use std::{
    cmp::Reverse,
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        BinaryHeap, HashMap, HashSet,
    },
    ops::Sub,
};

use crate::SearchNode;
use crate::{abstraction::TransitionSystem, Heuristic, Task};

/// Implementation of the Reverse Resumable A* algorithm
/// that computes the shortest path between:
/// - any state of a given transition system, and
/// - the goal state of a given task in this transition system.
/// The shortest paths are computed on demand by the heuristic requests.
/// This algorithm should not be used for Timed states, as it aims to compute
/// time-independent shortest paths between states.
pub struct ReverseResumableAStar<TS, S, A, C, DC, T, H>
where
    TS: TransitionSystem<S, A, DC>,
    S: Hash + Eq,
    C: Eq + PartialOrd + Ord + Add<DC, Output = C> + Sub<C, Output = DC> + Copy + Default,
    DC: Copy,
    T: Task<S> + Clone,
    H: Heuristic<TS, S, A, C, DC, T>,
{
    transition_system: Arc<TS>,
    task: Arc<T>,
    heuristic: H,
    queue: Mutex<BinaryHeap<Reverse<SearchNode<S, C, DC>>>>,
    distance: RwLock<HashMap<Arc<S>, C>>,
    closed: RwLock<HashSet<Arc<S>>>,
    initial_cost: C,
    _phantom: PhantomData<A>,
}

impl<TS, S, A, C, DC, T, H> Heuristic<TS, S, A, C, DC, T>
    for ReverseResumableAStar<TS, S, A, C, DC, T, H>
where
    TS: TransitionSystem<S, A, DC>,
    S: Hash + Eq,
    C: Eq + PartialOrd + Ord + Add<DC, Output = C> + Sub<C, Output = DC> + Copy + Default,
    DC: Copy,
    T: Task<S> + Clone,
    H: Heuristic<TS, S, A, C, DC, T>,
{
    fn new(transition_system: Arc<TS>, task: Arc<T>) -> Self
    where
        Self: Sized,
    {
        let mut rra = ReverseResumableAStar {
            transition_system: transition_system.clone(),
            task: task.clone(),
            heuristic: H::new(
                transition_system,
                // Reverse the task for the heuristic
                Arc::new(T::new(task.goal_state(), task.initial_state())),
            ),
            queue: Mutex::new(BinaryHeap::new()),
            distance: RwLock::new(HashMap::new()),
            closed: RwLock::new(HashSet::new()),
            initial_cost: C::default(),
            _phantom: PhantomData::default(),
        };
        rra.init();
        rra
    }

    fn get_heuristic(&self, state: Arc<S>) -> Option<DC> {
        self.find_path(state.clone())
    }
}

impl<TS, S, A, C, DC, T, H> ReverseResumableAStar<TS, S, A, C, DC, T, H>
where
    TS: TransitionSystem<S, A, DC>,
    S: Hash + Eq,
    C: Eq + PartialOrd + Ord + Add<DC, Output = C> + Sub<C, Output = DC> + Copy + Default,
    DC: Copy,
    T: Task<S> + Clone,
    H: Heuristic<TS, S, A, C, DC, T>,
{
    /// Initializes the reverse search algorithm by enqueueing the goal state.
    fn init(&mut self) {
        let goal_node = SearchNode {
            state: self.task.goal_state(),
            cost: C::default(),
            heuristic: C::default() - C::default(),
        };

        self.distance
            .write()
            .unwrap()
            .insert(goal_node.state.clone(), goal_node.cost);
        self.queue.get_mut().unwrap().push(Reverse(goal_node));
    }

    /// Computes the shortest path between the given state and the goal state,
    /// or returns directly if it has already been computed.
    fn find_path(&self, state: Arc<S>) -> Option<DC> {
        if self.closed.read().unwrap().contains(&state) {
            // The distance has already been computed
            return Some(self.distance.read().unwrap()[&state] - self.initial_cost);
        }

        let mut queue = self.queue.lock().unwrap(); // Lock the queue to avoid concurrent executions of the algorithm
        if self.closed.read().unwrap().contains(&state) {
            // Check if the distance has been computed while waiting for the lock
            return Some(self.distance.read().unwrap()[&state] - self.initial_cost);
        }

        while let Some(Reverse(current)) = queue.pop() {
            if current.cost > self.distance.read().unwrap()[&current.state] {
                // A better path has already been found
                continue;
            }

            if current.state == state {
                // The optimal distance has been found
                return Some(current.cost - self.initial_cost);
            }

            // Expand the current state and enqueue its successors if a better path has been found
            for action in self
                .transition_system
                .reverse_actions_from(current.state.clone())
            {
                let successor_state = Arc::new(
                    self.transition_system
                        .reverse_transition(current.state.clone(), &action),
                );
                let successor_cost = current.cost
                    + self
                        .transition_system
                        .reverse_transition_cost(current.state.clone(), &action);

                let improved = match self
                    .distance
                    .write()
                    .unwrap()
                    .entry(successor_state.clone())
                {
                    Occupied(mut e) => {
                        if successor_cost < *e.get() {
                            *e.get_mut() = successor_cost;
                            true
                        } else {
                            false
                        }
                    }
                    Vacant(e) => {
                        e.insert(successor_cost);
                        true
                    }
                };

                if improved {
                    if let Some(heuristic) = self.heuristic.get_heuristic(successor_state.clone()) {
                        queue.push(Reverse(SearchNode {
                            state: successor_state,
                            cost: successor_cost,
                            heuristic,
                        }));
                    }
                }
            }

            self.closed.write().unwrap().insert(current.state.clone()); // Mark the state as closed because it has been expanded
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::Duration;

    use crate::{
        Graph, GraphEdgeId, GraphNodeId, Heuristic, ReverseResumableAStar, SimpleHeuristic,
        SimpleState, SimpleTask, SimpleWorld, Task, Time,
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
        let task = Arc::new(SimpleTask::new(
            Arc::new(SimpleState(GraphNodeId(0))),
            Arc::new(SimpleState(GraphNodeId(size * size - 1))),
        ));
        let heuristic: ReverseResumableAStar<
            SimpleWorld,
            SimpleState,
            GraphEdgeId,
            Time,
            Duration,
            SimpleTask,
            SimpleHeuristic,
        > = ReverseResumableAStar::new(transition_system, task);

        for x in 0..size {
            for y in 0..size {
                assert_eq!(
                    heuristic
                        .get_heuristic(Arc::new(SimpleState(GraphNodeId(x + y * size))))
                        .unwrap(),
                    Duration::milliseconds((((size - x - 1) + (size - y - 1)) * 1000) as i64)
                );
            }
        }
    }
}
