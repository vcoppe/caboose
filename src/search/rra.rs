use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Add;
use std::sync::Arc;
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
    queue: BinaryHeap<Reverse<SearchNode<S, C, DC>>>,
    distance: HashMap<Arc<S>, C>,
    closed: HashSet<Arc<S>>,
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
            heuristic: H::new(transition_system, task),
            queue: BinaryHeap::new(),
            distance: HashMap::new(),
            closed: HashSet::new(),
            _phantom: PhantomData::default(),
        };
        rra.init();
        rra
    }

    fn get_heuristic(&mut self, state: Arc<S>) -> Option<DC> {
        if self.find_path(state.clone()) {
            Some(self.distance[&state] - C::default())
        } else {
            None
        }
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
            state: Arc::new(self.task.goal_state()),
            cost: C::default(),
            heuristic: C::default() - C::default(),
        };

        self.distance
            .insert(goal_node.state.clone(), goal_node.cost);
        self.queue.push(Reverse(goal_node));
    }

    /// Computes the shortest path between the given state and the goal state,
    /// or returns directly if it has already been computed.
    fn find_path(&mut self, state: Arc<S>) -> bool {
        if self.closed.contains(&state) {
            // The distance has already been computed
            return true;
        }

        while let Some(Reverse(current)) = self.queue.pop() {
            if current.cost > self.distance[&current.state] {
                // A better path has already been found
                continue;
            }

            if current.state == state {
                // The optimal distance has been found
                return true;
            }

            // Expand the current state and enqueue its successors if a better path has been found
            self.transition_system
                .for_each_reverse_action(current.state.clone(), &mut |action| {
                    let successor_state = Arc::new(
                        self.transition_system
                            .reverse_transition(current.state.clone(), &action),
                    );
                    let successor_cost = current.cost
                        + self
                            .transition_system
                            .reverse_transition_cost(current.state.clone(), &action);

                    let improved = match self.distance.entry(successor_state.clone()) {
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
                        if let Some(heuristic) =
                            self.heuristic.get_heuristic(successor_state.clone())
                        {
                            self.queue.push(Reverse(SearchNode {
                                state: successor_state,
                                cost: successor_cost,
                                heuristic,
                            }));
                        }
                    }
                });

            self.closed.insert(current.state.clone()); // Mark the state as closed because it has been expanded
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ordered_float::OrderedFloat;

    use crate::{
        Graph, GraphEdgeId, GraphNodeId, Heuristic, ReverseResumableAStar, SimpleHeuristic,
        SimpleTask, SimpleWorld,
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
    fn test_heuristic() {
        let size = 10;
        let graph = simple_graph(size);
        let transition_system = Arc::new(SimpleWorld::new(graph));
        let task = Arc::new(SimpleTask::new(
            GraphNodeId(0),
            GraphNodeId(size * size - 1),
        ));
        let mut heuristic: ReverseResumableAStar<
            SimpleWorld,
            GraphNodeId,
            GraphEdgeId,
            OrderedFloat<f64>,
            OrderedFloat<f64>,
            SimpleTask,
            SimpleHeuristic,
        > = ReverseResumableAStar::new(transition_system, task);

        for x in 0..size {
            for y in 0..size {
                assert_eq!(
                    heuristic
                        .get_heuristic(Arc::new(GraphNodeId(x + y * size)))
                        .unwrap(),
                    OrderedFloat(((size - x - 1) + (size - y - 1)) as f64)
                );
            }
        }
    }
}
