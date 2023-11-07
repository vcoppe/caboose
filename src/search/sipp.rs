use std::{
    cmp::Reverse,
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        BinaryHeap, HashMap, HashSet,
    },
    hash::Hash,
    marker::PhantomData,
    sync::Arc,
    vec,
};

use chrono::Duration;

use crate::{Heuristic, Interval, SearchNode, Solver, Task, Time, Timed, TransitionSystem};

#[derive(PartialEq, Eq, Hash)]
struct SippState<S>
where
    S: Timed,
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
    S: Copy + Hash + Eq + Timed,
    A: Copy,
    T: Task<S>,
    H: Heuristic<TS, S, A, Time, Duration, T>,
{
    transition_system: Arc<TS>,
    heuristic: Option<H>,
    queue: BinaryHeap<Reverse<SearchNode<SippState<S>, Time, Duration>>>,
    distance: HashMap<Arc<SippState<S>>, Time>,
    closed: HashSet<Arc<SippState<S>>>,
    parent: HashMap<Arc<SippState<S>>, (A, Arc<SippState<S>>)>,
    _phantom: PhantomData<(A, T)>,
}

impl<TS, S, A, T, H> Solver<TS, S, A, Time, Duration, T, H>
    for SafeIntervalPathPlanning<TS, S, A, T, H>
where
    TS: TransitionSystem<S, A, Duration>,
    S: Copy + Hash + Eq + Timed,
    A: Copy,
    T: Task<S>,
    H: Heuristic<TS, S, A, Time, Duration, T>,
{
    fn solve(&mut self, task: Arc<T>) -> Option<Vec<A>> {
        self._solve(task)
    }
}

impl<TS, S, A, T, H> SafeIntervalPathPlanning<TS, S, A, T, H>
where
    TS: TransitionSystem<S, A, Duration>,
    S: Copy + Hash + Eq + Timed,
    A: Copy,
    T: Task<S>,
    H: Heuristic<TS, S, A, Time, Duration, T>,
{
    fn new(transition_system: Arc<TS>) -> Self {
        SafeIntervalPathPlanning {
            transition_system,
            heuristic: None,
            queue: BinaryHeap::new(),
            distance: HashMap::new(),
            closed: HashSet::new(),
            parent: HashMap::new(),
            _phantom: PhantomData::default(),
        }
    }

    /// Initializes the search algorithm by enqueueing the initial state.
    /// Returns true if the initial state belongs to a safe interval, and false otherwise.
    fn init(&mut self, task: Arc<T>) -> bool {
        self.queue.clear();
        self.distance.clear();
        self.closed.clear();
        self.parent.clear();

        let initial_state = task.initial_state();
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
            internal_state: Arc::new(initial_state),
        });

        let initial_node = SearchNode {
            state: initial_state,
            cost: Time::default(),
            heuristic: Duration::seconds(0),
        };

        self.distance
            .insert(initial_node.state.clone(), initial_node.cost);
        self.queue.push(Reverse(initial_node));

        self.heuristic = Some(H::new(self.transition_system.clone(), task));

        true
    }

    fn _solve(&mut self, task: Arc<T>) -> Option<Vec<A>> {
        if !self.init(task.clone()) {
            return None;
        }

        self.find_path(task.as_ref()).map(|g| self.get_path(g))
    }

    fn find_path(&mut self, task: &T) -> Option<Arc<SippState<S>>> {
        while let Some(Reverse(current)) = self.queue.pop() {
            if current.cost > self.distance[current.state.as_ref()] {
                // A better path has already been found
                continue;
            }

            if task.is_goal_state(current.state.internal_state.clone()) {
                // The optimal distance has been found
                return Some(current.state);
            }

            // Expand the current state and enqueue its successors
            for successor in self.get_successors(current.state.clone()) {
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
        current: Arc<SippState<S>>,
    ) -> Vec<SearchNode<SippState<S>, Time, Duration>> {
        let mut successors = vec![];

        self.transition_system
            .for_each_action(current.internal_state.clone(), &mut |action| {
                let generic_successor_state = Arc::new(
                    self.transition_system
                        .transition(current.internal_state.clone(), &action),
                );
                let transition_cost = self
                    .transition_system
                    .transition_cost(current.internal_state.clone(), &action);

                let mut heuristic = Duration::seconds(0);
                if let Some(heuristic_function) = self.heuristic.as_mut() {
                    let heuristic_value =
                        heuristic_function.get_heuristic(generic_successor_state.clone());
                    if heuristic_value.is_none() {
                        return; // Goal state is not reachable from this state
                    }
                    heuristic = heuristic_value.unwrap();
                }

                let collision_intervals = self.get_collision_intervals(action);

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
                            .insert(successor.state.clone(), (action, current.clone()));
                        successors.push(successor);
                    }
                }
            });

        successors
    }

    fn get_path(&self, goal: Arc<SippState<S>>) -> Vec<A> {
        let mut actions = vec![];
        let mut current = goal;

        while let Some((action, parent)) = self.parent.get(&current) {
            actions.push(*action);
            current = parent.clone();
        }

        actions.reverse();

        actions
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
