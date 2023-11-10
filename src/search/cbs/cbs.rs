use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
    ops::{Add, Sub},
    sync::Arc,
};

use crate::{
    Conflict, Constraint, Heuristic, LSippConfig, LimitValues, ReverseResumableAStar,
    SafeIntervalPathPlanningWithLandmarks, SippState, Solution, State, Task, TransitionSystem,
};

/// Implementation of the Conflict-Based Search algorithm.
struct ConflictBasedSearch<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: Debug + State + Eq + Hash + Copy,
    A: Debug + Copy,
    C: Hash
        + Eq
        + PartialOrd
        + Ord
        + Add<DC, Output = C>
        + Sub<DC, Output = C>
        + Sub<C, Output = DC>
        + Copy
        + Default
        + LimitValues,
    DC: Ord + Sub<DC, Output = DC> + Copy + Default,
    H: Heuristic<TS, S, A, C, DC>,
{
    transition_system: Arc<TS>,
    queue: BinaryHeap<Reverse<CbsNode<S, A, C, DC>>>,
    lsipp: SafeIntervalPathPlanningWithLandmarks<
        TS,
        S,
        A,
        C,
        DC,
        ReverseResumableAStar<TS, S, A, C, DC, H>,
    >,
}

impl<TS, S, A, C, DC, H> ConflictBasedSearch<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: Debug + State + Eq + Hash + Copy,
    A: Debug + Copy,
    C: Hash
        + Eq
        + PartialOrd
        + Ord
        + Add<DC, Output = C>
        + Sub<DC, Output = C>
        + Sub<C, Output = DC>
        + Copy
        + Default
        + LimitValues,
    DC: Ord + Sub<DC, Output = DC> + Copy + Default,
    H: Heuristic<TS, S, A, C, DC>,
{
    pub fn new(transition_system: Arc<TS>) -> Self {
        let lsipp = SafeIntervalPathPlanningWithLandmarks::new(transition_system.clone());
        Self {
            transition_system,
            queue: BinaryHeap::new(),
            lsipp,
        }
    }

    fn init(&mut self, config: &CbsConfig<TS, S, A, C, DC, H>) {
        self.queue.clear();

        if let Some(root) = self.get_root(config) {
            self.enqueue(config, root);
        }
    }

    fn enqueue(&mut self, config: &CbsConfig<TS, S, A, C, DC, H>, mut node: CbsNode<S, A, C, DC>) {
        self.compute_conflicts(&mut node, config.tasks.len());
        self.queue.push(Reverse(node));
    }

    fn get_root(&mut self, config: &CbsConfig<TS, S, A, C, DC, H>) -> Option<CbsNode<S, A, C, DC>> {
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
        config: &CbsConfig<TS, S, A, C, DC, H>,
    ) -> Option<Vec<Solution<Arc<SippState<S, C>>, A, C>>> {
        self.init(config);

        while let Some(Reverse(node)) = self.queue.pop() {
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

            // plan path with first negative constraint
            // plan path with second negative constraint
        }

        None
    }

    /// Computes the conflicts between the solutions of the given node.
    fn compute_conflicts(&self, node: &mut CbsNode<S, A, C, DC>, n_agents: usize) {
        let mut conflicts = vec![];

        if let Some(parent) = &node.parent {
            let agent = node.constraint.as_ref().unwrap().agent;

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
            &Solution<Arc<SippState<S, C>>, A, C>,
            &Solution<Arc<SippState<S, C>>, A, C>,
        ),
    ) -> Option<Conflict<S, A, C, DC>> {
        // Iterate through both solutions and find moves overlapping in C
        let mut i = 0;
        let mut j = 0;

        while i < solutions.0.states.len() || j < solutions.1.states.len() {
            // TODO
        }

        // TODO compute conflict type

        None
    }
}

/// Input configuration for the Conflict-Based Search algorithm.
struct CbsConfig<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC>,
    S: State + Eq + Hash,
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
    tasks: Vec<Arc<Task<S, C>>>,
    /// A set of pivot states.
    pivots: Arc<Vec<Arc<S>>>,
    /// A set of heuristics to those pivot states.
    heuristic_to_pivots: Arc<Vec<Arc<ReverseResumableAStar<TS, S, A, C, DC, H>>>>,
    _phantom: PhantomData<(TS, A)>,
}

/// A node in the Conflict-Based Search tree.
struct CbsNode<S, A, C, DC>
where
    S: Debug + State + Eq,
    C: Ord + Default + LimitValues,
{
    total_cost: DC,
    parent: Option<Arc<Self>>,
    solutions: Vec<Solution<Arc<SippState<S, C>>, A, C>>,
    conflicts: Vec<Arc<Conflict<S, A, C, DC>>>,
    constraint: Option<Constraint<S, C>>,
    landmark: Option<Constraint<S, C>>,
}

impl<S, A, C, DC> Default for CbsNode<S, A, C, DC>
where
    S: Debug + State + Eq,
    C: Ord + Default + LimitValues,
    DC: Default,
{
    fn default() -> Self {
        Self {
            total_cost: DC::default(),
            parent: None,
            solutions: vec![],
            conflicts: vec![],
            constraint: None,
            landmark: None,
        }
    }
}

impl<S, A, C, DC> CbsNode<S, A, C, DC>
where
    S: Debug + State + Eq,
    C: Ord + Default + LimitValues,
{
    pub fn get_solutions(&self, n_agents: usize) -> Vec<&Solution<Arc<SippState<S, C>>, A, C>> {
        let mut found = 0;
        let mut solutions = vec![None; n_agents];

        let mut current = self;
        loop {
            if let Some(constraint) = &current.constraint {
                let agent = constraint.agent;
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
impl<S, A, C, DC> PartialEq for CbsNode<S, A, C, DC>
where
    S: Debug + State + Eq,
    C: Ord + Default + LimitValues,
    DC: PartialEq + Eq + PartialOrd + Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.total_cost == other.total_cost
    }
}

impl<S, A, C, DC> Eq for CbsNode<S, A, C, DC>
where
    S: Debug + State + Eq,
    C: Ord + Default + LimitValues,
    DC: PartialEq + Eq + PartialOrd + Ord,
{
}

impl<S, A, C, DC> PartialOrd for CbsNode<S, A, C, DC>
where
    S: Debug + State + Eq,
    C: Ord + Default + LimitValues,
    DC: PartialEq + Eq + PartialOrd + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.total_cost.partial_cmp(&other.total_cost)
    }
}

impl<S, A, C, DC> Ord for CbsNode<S, A, C, DC>
where
    S: Debug + State + Eq,
    C: Ord + Default + LimitValues,
    DC: PartialEq + Eq + PartialOrd + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_cost.cmp(&other.total_cost)
    }
}
