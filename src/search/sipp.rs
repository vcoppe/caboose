use std::{
    cmp::Reverse,
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        BinaryHeap, HashMap, HashSet,
    },
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
    sync::Arc,
    vec,
};

use chrono::Duration;

use crate::{Heuristic, Interval, SearchNode, Solution, Task, Time, Timed, TransitionSystem};

struct SippConfig<TS, S, A, T, H>
where
    TS: TransitionSystem<S, A, Duration>,
    S: Debug + Copy + Hash + Eq + Timed,
    A: Copy,
    T: Task<S>,
    H: Heuristic<TS, S, A, Time, Duration, T>,
{
    task: Arc<T>,
    heuristic: H,
    _phantom: PhantomData<(TS, S, A)>,
}

#[derive(PartialEq, Eq, Hash)]
struct SippState<S>
where
    S: Debug + Timed,
{
    safe_interval_id: usize,
    safe_interval: Interval,
    internal_state: Arc<S>,
}

/// Implementation of the Safe Interval Path Planning algorithm that computes
/// the optimal sequence of actions to complete a given task in a given transition system,
/// while avoiding conflicts with other agents in the same environment.
struct SafeIntervalPathPlanning<TS, S, A, T, H>
where
    TS: TransitionSystem<S, A, Duration>,
    S: Debug + Copy + Hash + Eq + Timed,
    A: Copy,
    T: Task<S>,
    H: Heuristic<TS, S, A, Time, Duration, T>,
{
    transition_system: Arc<TS>,
    queue: BinaryHeap<Reverse<SearchNode<SippState<S>, Time, Duration>>>,
    distance: HashMap<Arc<SippState<S>>, Time>,
    closed: HashSet<Arc<SippState<S>>>,
    parent: HashMap<Arc<SippState<S>>, (A, Arc<SippState<S>>)>,
    _phantom: PhantomData<(A, T, H)>,
}

impl<TS, S, A, T, H> SafeIntervalPathPlanning<TS, S, A, T, H>
where
    TS: TransitionSystem<S, A, Duration>,
    S: Debug + Copy + Hash + Eq + Timed,
    A: Copy,
    T: Task<S>,
    H: Heuristic<TS, S, A, Time, Duration, T>,
{
    fn new(transition_system: Arc<TS>) -> Self {
        SafeIntervalPathPlanning {
            transition_system,
            queue: BinaryHeap::new(),
            distance: HashMap::new(),
            closed: HashSet::new(),
            parent: HashMap::new(),
            _phantom: PhantomData::default(),
        }
    }

    /// Applies the algorithm to the given configuration.
    pub fn solve(&mut self, config: &mut SippConfig<TS, S, A, T, H>) -> Option<Solution<A, Time>> {
        if !self.init(config) {
            return None;
        }

        self.find_path(config).map(|g| self.get_solution(g))
    }

    /// Initializes the search algorithm by enqueueing the initial state.
    /// Returns true if the initial state belongs to a safe interval, and false otherwise.
    fn init(&mut self, config: &SippConfig<TS, S, A, T, H>) -> bool {
        self.queue.clear();
        self.distance.clear();
        self.closed.clear();
        self.parent.clear();

        let initial_state = config.task.initial_state();
        let initial_time = initial_state.get_time();

        let safe_intervals = self.get_safe_intervals(&initial_state);
        let safe_interval = safe_intervals
            .iter()
            .find(|interval| interval.start <= initial_time && interval.end >= initial_time);

        if safe_interval.is_none() {
            return false;
        }

        let initial_state = Arc::new(SippState {
            safe_interval_id: 0,
            safe_interval: *safe_interval.unwrap(),
            internal_state: initial_state,
        });

        let initial_node = SearchNode {
            state: initial_state,
            cost: initial_time,
            heuristic: Duration::seconds(0),
        };

        self.distance
            .insert(initial_node.state.clone(), initial_node.cost);
        self.queue.push(Reverse(initial_node));

        true
    }

    fn find_path(&mut self, config: &mut SippConfig<TS, S, A, T, H>) -> Option<Arc<SippState<S>>> {
        while let Some(Reverse(current)) = self.queue.pop() {
            if current.cost > self.distance[current.state.as_ref()] {
                // A better path has already been found
                continue;
            }

            if config
                .task
                .is_goal_state(current.state.internal_state.as_ref())
            {
                // The optimal distance has been found
                return Some(current.state);
            }

            // Expand the current state and enqueue its successors
            for successor in self.get_successors(config, current.state.clone()) {
                self.distance
                    .insert(successor.state.clone(), successor.cost);
                self.queue.push(Reverse(successor));
            }

            self.closed.insert(current.state.clone()); // Mark the state as closed because it has been expanded
        }

        None
    }

    fn get_successors(
        &mut self,
        config: &mut SippConfig<TS, S, A, T, H>,
        current: Arc<SippState<S>>,
    ) -> Vec<SearchNode<SippState<S>, Time, Duration>> {
        let mut successors = vec![];

        for action in self
            .transition_system
            .actions_from(current.internal_state.clone())
        {
            let generic_successor_state = Arc::new(
                self.transition_system
                    .transition(current.internal_state.clone(), &action),
            );
            let transition_cost = self
                .transition_system
                .transition_cost(current.internal_state.clone(), &action);

            let heuristic = config
                .heuristic
                .get_heuristic(generic_successor_state.clone());
            if heuristic.is_none() {
                continue; // Goal state is not reachable from this state
            }
            let heuristic = heuristic.unwrap();

            let collision_intervals = self.get_collision_intervals(*action);

            // Try to reach any of the safe intervals of the destination state
            // and add the corresponding successors to the queue if a better path has been found
            for (safe_interval_id, safe_interval) in self
                .get_safe_intervals(generic_successor_state.as_ref())
                .iter()
                .enumerate()
            {
                let mut successor_cost = current.internal_state.get_time() + transition_cost;

                if successor_cost > safe_interval.end {
                    // Cannot reach this safe interval in time
                    continue;
                }

                if successor_cost < safe_interval.start {
                    // Would arrive too early
                    successor_cost = safe_interval.start; // Try to depart later to arrive at the right time
                    if successor_cost - transition_cost > current.safe_interval.end {
                        // Cannot depart that late from the current safe interval
                        continue;
                    }
                }

                if let Some(collision_interval) = collision_intervals
                    .iter()
                    .find(|interval| interval.end >= successor_cost - transition_cost)
                {
                    if successor_cost - transition_cost >= collision_interval.start {
                        // Collision detected
                        successor_cost = collision_interval.end + transition_cost; // Try to depart later

                        if successor_cost - transition_cost > current.safe_interval.end
                            || successor_cost > safe_interval.end
                        {
                            continue;
                        }
                    }
                }

                // Create successor state with the real arrival time, so that future transitions are correct
                let mut successor_state = *generic_successor_state;
                successor_state.set_time(successor_cost);

                let successor_state = Arc::new(SippState {
                    safe_interval_id,
                    safe_interval: *safe_interval,
                    internal_state: Arc::new(successor_state),
                });

                let successor = SearchNode {
                    state: successor_state,
                    cost: successor_cost,
                    heuristic,
                };

                let improved = match self.distance.entry(successor.state.clone()) {
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
                    self.parent
                        .insert(successor.state.clone(), (*action, current.clone()));
                    successors.push(successor);
                }
            }
        }

        successors
    }

    fn get_solution(&self, goal: Arc<SippState<S>>) -> Solution<A, Time> {
        let mut solution = Solution::default();
        solution.cost = goal.internal_state.get_time();
        let mut current = goal;

        while let Some((action, parent)) = self.parent.get(&current) {
            solution.actions.push(*action);
            current = parent.clone();
        }

        solution.actions.reverse();

        solution
    }

    fn get_safe_intervals(&self, _state: &S) -> Vec<Interval> {
        vec![Interval {
            start: Time::MIN_UTC.into(),
            end: Time::MAX_UTC.into(),
        }]
    }

    fn get_collision_intervals(&self, _action: A) -> Vec<Interval> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::Duration;

    use crate::{
        search::sipp::SippConfig, Graph, GraphEdgeId, GraphNodeId, Heuristic, SimpleTimedState,
        SimpleTimedTask, SimpleWorld, Task, Time, TimedHeuristic,
    };

    use super::SafeIntervalPathPlanning;

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
        let initial_time = Time::MIN_UTC.into();
        let mut solver: SafeIntervalPathPlanning<
            SimpleWorld,
            SimpleTimedState,
            GraphEdgeId,
            SimpleTimedTask,
            TimedHeuristic,
        > = SafeIntervalPathPlanning::new(transition_system.clone());

        for x in 0..size {
            for y in 0..size {
                let task = Arc::new(SimpleTimedTask::new(
                    Arc::new(SimpleTimedState {
                        node: GraphNodeId(x + size * y),
                        time: initial_time,
                    }),
                    Arc::new(SimpleTimedState {
                        node: GraphNodeId(size * size - 1),
                        time: initial_time,
                    }),
                ));
                let mut config = SippConfig {
                    task: task.clone(),
                    heuristic: TimedHeuristic::new(transition_system.clone(), task),
                    _phantom: Default::default(),
                };

                assert_eq!(
                    solver.solve(&mut config).unwrap().cost,
                    initial_time
                        + Duration::milliseconds((((size - x - 1) + (size - y - 1)) * 1000) as i64)
                );
                break;
            }
        }
    }
}
