use std::{
    cmp::Reverse,
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        BinaryHeap,
    },
    hash::Hash,
    marker::PhantomData,
    ops::Add,
    ops::Sub,
    sync::Arc,
};

use fxhash::{FxHashMap, FxHashSet};
use parking_lot::{Mutex, RwLock};

use crate::{abstraction::TransitionSystem, Heuristic, Task};
use crate::{LimitValues, SearchNode, State};

/// Implementation of the Reverse Resumable A* algorithm
/// that computes the shortest path between:
/// - any state of a given transition system, and
/// - the goal state of a given task in this transition system.
/// The shortest paths are computed on demand by the heuristic requests.
pub struct ReverseResumableAStar<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: State + Hash + Eq,
    C: Eq
        + PartialOrd
        + Ord
        + Add<DC, Output = C>
        + Sub<C, Output = DC>
        + Copy
        + Default
        + LimitValues,
    DC: Copy,
    H: Heuristic<TS, S, A, C, DC>,
{
    transition_system: Arc<TS>,
    task: Arc<Task<S, C>>,
    /// The heuristic must be an estimate of the distance to the start state
    heuristic: Arc<H>,
    queue: Mutex<BinaryHeap<Reverse<SearchNode<S, C, DC>>>>,
    distance: RwLock<FxHashMap<Arc<S>, C>>,
    closed: RwLock<FxHashSet<Arc<S>>>,
    _phantom: PhantomData<A>,
}

impl<TS, S, A, C, DC, H> Heuristic<TS, S, A, C, DC> for ReverseResumableAStar<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: State + Hash + Eq,
    C: Eq
        + PartialOrd
        + Ord
        + Add<DC, Output = C>
        + Sub<C, Output = DC>
        + Copy
        + Default
        + LimitValues,
    DC: Copy,
    H: Heuristic<TS, S, A, C, DC>,
{
    fn get_heuristic(&self, state: Arc<S>) -> Option<DC> {
        self.find_path(state.clone())
    }
}

impl<TS, S, A, C, DC, H> ReverseResumableAStar<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: State + Hash + Eq,
    C: Eq
        + PartialOrd
        + Ord
        + Add<DC, Output = C>
        + Sub<C, Output = DC>
        + Copy
        + Default
        + LimitValues,
    DC: Copy,
    H: Heuristic<TS, S, A, C, DC>,
{
    pub fn new(transition_system: Arc<TS>, task: Arc<Task<S, C>>, heuristic: Arc<H>) -> Self
    where
        Self: Sized,
    {
        let mut rra = ReverseResumableAStar {
            transition_system: transition_system.clone(),
            task: task.clone(),
            heuristic,
            queue: Mutex::new(BinaryHeap::new()),
            distance: RwLock::new(FxHashMap::default()),
            closed: RwLock::new(FxHashSet::default()),
            _phantom: PhantomData::default(),
        };
        rra.init();
        rra
    }

    /// Initializes the reverse search algorithm by enqueueing the goal state.
    fn init(&mut self) {
        let goal_node = SearchNode {
            state: self.task.goal_state.clone(),
            cost: self.task.initial_cost,
            heuristic: C::default() - C::default(),
        };

        self.distance
            .write()
            .insert(goal_node.state.clone(), goal_node.cost);
        self.queue.get_mut().push(Reverse(goal_node));
    }

    /// Computes the shortest path between the given state and the goal state,
    /// or returns directly if it has already been computed.
    fn find_path(&self, state: Arc<S>) -> Option<DC> {
        if self.closed.read().contains(&state) {
            // The distance has already been computed
            return Some(self.distance.read()[&state] - self.task.initial_cost);
        }

        let mut queue = self.queue.lock(); // Lock the queue to avoid concurrent executions of the algorithm
        if self.closed.read().contains(&state) {
            // Check if the distance has been computed while waiting for the lock
            return Some(self.distance.read()[&state] - self.task.initial_cost);
        }

        while let Some(Reverse(current)) = queue.pop() {
            if current.cost > self.distance.read()[&current.state] {
                // A better path has already been found
                continue;
            }

            if current.state == state {
                // The optimal distance has been found
                return Some(current.cost - self.task.initial_cost);
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

                let improved = match self.distance.write().entry(successor_state.clone()) {
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

            self.closed.write().insert(current.state.clone()); // Mark the state as closed because it has been expanded
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ordered_float::OrderedFloat;

    use crate::{
        Graph, GraphNodeId, Heuristic, ReverseResumableAStar, SimpleHeuristic, SimpleState,
        SimpleWorld, Task,
    };

    fn simple_graph(size: usize) -> Arc<Graph> {
        let mut graph = Graph::new();
        for x in 0..size {
            for y in 0..size {
                graph.add_node((x as f32, y as f32), 1.0);
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
        let task = Arc::new(Task::new(
            Arc::new(SimpleState(GraphNodeId(0))),
            Arc::new(SimpleState(GraphNodeId(size * size - 1))),
            OrderedFloat(0.0),
        ));
        let heuristic = ReverseResumableAStar::new(
            transition_system.clone(),
            task.clone(),
            Arc::new(SimpleHeuristic::new(transition_system, task)),
        );

        for x in 0..size {
            for y in 0..size {
                assert_eq!(
                    heuristic
                        .get_heuristic(Arc::new(SimpleState(GraphNodeId(x + y * size))))
                        .unwrap(),
                    OrderedFloat(((size - x - 1) + (size - y - 1)) as f32)
                );
            }
        }
    }
}
