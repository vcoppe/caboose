use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
    sync::Arc,
};

use chrono::Duration;

use crate::{
    ActionConstraint, Conflict, Heuristic, LSippConfig, ReverseResumableAStar,
    SafeIntervalPathPlanningWithLandmarks, SippState, Solution, State, StateConstraint, Task, Time,
    TransitionSystem,
};

/// Implementation of the Conflict-Based Search algorithm.
struct ConflictBasedSearch<TS, S, A, H>
where
    TS: TransitionSystem<S, A, Time, Duration>,
    S: Debug + State + Eq + Hash + Copy,
    A: Debug + Copy,
    H: Heuristic<TS, S, A, Time, Duration>,
{
    transition_system: Arc<TS>,
    queue: BinaryHeap<Reverse<CbsNode<S, A>>>,
    lsipp: SafeIntervalPathPlanningWithLandmarks<
        TS,
        S,
        A,
        ReverseResumableAStar<TS, S, A, Time, Duration, H>,
    >,
}

impl<TS, S, A, H> ConflictBasedSearch<TS, S, A, H>
where
    TS: TransitionSystem<S, A, Time, Duration>,
    S: Debug + State + Eq + Hash + Copy,
    A: Debug + Copy,
    H: Heuristic<TS, S, A, Time, Duration>,
{
    pub fn new(transition_system: Arc<TS>) -> Self {
        let lsipp = SafeIntervalPathPlanningWithLandmarks::new(transition_system.clone());
        Self {
            transition_system,
            queue: BinaryHeap::new(),
            lsipp,
        }
    }

    fn init(&mut self, config: &CbsConfig<TS, S, A, H>) {
        self.queue.clear();

        if let Some(root) = self.get_root(config) {
            self.queue.push(Reverse(root));
        }
    }

    fn get_root(&mut self, config: &CbsConfig<TS, S, A, H>) -> Option<CbsNode<S, A>> {
        let mut root = CbsNode::default();

        // Solve each task independently
        for task in config.tasks.iter() {
            let config = LSippConfig::new_with_pivots(
                task.clone(),
                Default::default(),
                Default::default(),
                Default::default(),
                config.pivots.clone(),
                config.heuristic_to_pivots.clone(),
            );
            if let Some(solution) = self.lsipp.solve(&config) {
                root.solutions.push(solution);
            } else {
                return None;
            }
        }

        Some(root)
    }

    pub fn solve(
        &mut self,
        config: &CbsConfig<TS, S, A, H>,
    ) -> Option<Vec<Solution<Arc<SippState<S>>, A, Time>>> {
        self.init(config);

        while let Some(Reverse(mut node)) = self.queue.pop() {
            // Compute conflicts between the solutions of the current node
            self.compute_conflicts(&mut node, config.tasks.len());

            if node.conflicts.is_empty() {
                // No conflicts, we have a solution
                return Some(
                    node.get_solutions(config.tasks.len())
                        .iter()
                        .map(|sol| (*sol).clone())
                        .collect(),
                );
            }

            // Find the conflict with the highest priority
            let conflict = node.conflicts.iter().min().unwrap();

            // TODO: create child nodes that solve the conflict differently
        }

        None
    }

    /// Computes the conflicts between the solutions of the given node.
    fn compute_conflicts(&self, node: &mut CbsNode<S, A>, n_agents: usize) {
        let mut conflicts = vec![];

        if let Some(parent) = &node.parent {
            let agent = node.agent.unwrap();

            // Get conflicts from the parent node that do not involve the given agent
            parent
                .conflicts
                .iter()
                .filter(|c| c.moves.0.agent != agent && c.moves.1.agent != agent)
                .for_each(|c| {
                    conflicts.push(c.clone());
                });

            // Compute conflicts between the given agent and all other agents
            let solutions = node.get_solutions(n_agents);
            for other in 0..solutions.len() {
                if other == agent {
                    continue;
                }

                if let Some(conflict) = self.get_conflicts((solutions[agent], solutions[other])) {
                    // TODO: Set conflict type
                    conflicts.push(Arc::new(conflict));
                }
            }
        } else {
            // Root node, compute conflicts between each pair of solutions
            for i in 0..node.solutions.len() {
                for j in i + 1..node.solutions.len() {
                    if let Some(conflict) =
                        self.get_conflicts((&node.solutions[i], &node.solutions[j]))
                    {
                        conflicts.push(Arc::new(conflict));
                    }
                }
            }
        }

        conflicts
            .drain(..)
            .for_each(|conflict| node.conflicts.push(conflict))
    }

    /// Returns the first conflict between the given solutions, if any.
    fn get_conflicts(
        &self,
        solutions: (
            &Solution<Arc<SippState<S>>, A, Time>,
            &Solution<Arc<SippState<S>>, A, Time>,
        ),
    ) -> Option<Conflict<S, A, Time, Duration>> {
        // Iterate through both solutions and find moves overlapping in time
        let mut i = 0;
        let mut j = 0;

        while i < solutions.0.states.len() || j < solutions.1.states.len() {
            // TODO
        }

        None
    }
}

/// Input configuration for the Conflict-Based Search algorithm.
struct CbsConfig<TS, S, A, H>
where
    TS: TransitionSystem<S, A, Time, Duration>,
    S: State + Eq + Hash,
    H: Heuristic<TS, S, A, Time, Duration>,
{
    tasks: Vec<Arc<Task<S, Time>>>,
    /// A set of pivot states.
    pivots: Arc<Vec<Arc<S>>>,
    /// A set of heuristics to those pivot states.
    heuristic_to_pivots: Arc<Vec<Arc<ReverseResumableAStar<TS, S, A, Time, Duration, H>>>>,
    _phantom: PhantomData<(TS, A)>,
}

/// A node in the Conflict-Based Search tree.
struct CbsNode<S, A>
where
    S: Debug + State + Eq,
{
    total_cost: Duration,
    parent: Option<Arc<Self>>,
    solutions: Vec<Solution<Arc<SippState<S>>, A, Time>>,
    conflicts: Vec<Arc<Conflict<S, A, Time, Duration>>>,
    agent: Option<usize>,
    state_constraint: Option<StateConstraint<S>>,
    action_constraint: Option<ActionConstraint<S>>,
}

impl<S, A> Default for CbsNode<S, A>
where
    S: Debug + State + Eq,
{
    fn default() -> Self {
        Self {
            total_cost: Duration::zero(),
            parent: None,
            solutions: vec![],
            conflicts: vec![],
            agent: None,
            state_constraint: None,
            action_constraint: None,
        }
    }
}

impl<S, A> CbsNode<S, A>
where
    S: Debug + State + Eq,
{
    pub fn get_solutions(&self, n_agents: usize) -> Vec<&Solution<Arc<SippState<S>>, A, Time>> {
        let mut found = 0;
        let mut solutions = vec![None; n_agents];

        let mut current = self;
        loop {
            if let Some(agent) = current.agent {
                if solutions[agent].is_none() {
                    solutions[agent] = Some(&current.solutions[0]);
                    found += 1;
                }
            } else {
                for agent in 0..n_agents {
                    if solutions[agent].is_none() {
                        solutions[agent] = Some(&current.solutions[agent]);
                        found += 1;
                    }
                }
            }

            if found == n_agents {
                break;
            }

            if let Some(parent) = &current.parent {
                current = parent;
            } else {
                break;
            }
        }

        solutions.into_iter().map(|s| s.unwrap()).collect()
    }
}

// TODO: add high-level heuristic to ordering
impl<S, A> PartialEq for CbsNode<S, A>
where
    S: Debug + State + Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.total_cost == other.total_cost
    }
}

impl<S, A> Eq for CbsNode<S, A> where S: Debug + State + Eq {}

impl<S, A> PartialOrd for CbsNode<S, A>
where
    S: Debug + State + Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.total_cost.partial_cmp(&other.total_cost)
    }
}

impl<S, A> Ord for CbsNode<S, A>
where
    S: Debug + State + Eq,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_cost.cmp(&other.total_cost)
    }
}
