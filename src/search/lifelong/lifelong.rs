use std::{
    fmt::Debug,
    hash::Hash,
    ops::{Add, Div, Sub},
    sync::Arc,
};

use fxhash::FxHashMap;

use crate::{
    CbsConfig, ConflictBasedSearch, Heuristic, HeuristicBuilder, Interval, LimitValues,
    ReverseResumableAStar, SippState, Solution, State, Task, TransitionSystem,
};

/// A lifelong planner that supports requests for new tasks while other tasks are being executed.
/// It uses Conflict-Based Search under the hood.
pub struct Lifelong<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC> + Send + Sync,
    S: Debug + State + Eq + Hash + Clone + Send + Sync,
    A: Debug + Copy + Send + Sync,
    C: Debug
        + Hash
        + Eq
        + PartialOrd
        + Ord
        + Add<DC, Output = C>
        + Sub<DC, Output = C>
        + Sub<C, Output = DC>
        + Copy
        + Default
        + LimitValues
        + Send
        + Sync,
    DC: Debug
        + Hash
        + Ord
        + Add<DC, Output = DC>
        + Sub<DC, Output = DC>
        + Div<f64, Output = DC>
        + Copy
        + Default
        + Send
        + Sync,
    H: Heuristic<TS, S, A, C, DC> + HeuristicBuilder<TS, S, A, C, DC> + Send + Sync,
{
    transition_system: Arc<TS>,
    solver: ConflictBasedSearch<TS, S, A, C, DC, H>,
    tasks: Vec<Arc<Task<S, C>>>,
    solutions: Vec<Solution<Arc<SippState<S, C, DC>>, A, C, DC>>,
    heuristic_to_pivots: Vec<Arc<ReverseResumableAStar<TS, S, A, C, DC, H>>>,
    collision_precision: DC,
}

impl<TS, S, A, C, DC, H> Lifelong<TS, S, A, C, DC, H>
where
    TS: TransitionSystem<S, A, C, DC> + Send + Sync,
    S: Debug + State + Eq + Hash + Clone + Send + Sync,
    A: Debug + Copy + Send + Sync,
    C: Debug
        + Hash
        + Eq
        + PartialOrd
        + Ord
        + Add<DC, Output = C>
        + Sub<DC, Output = C>
        + Sub<C, Output = DC>
        + Copy
        + Default
        + LimitValues
        + Send
        + Sync,
    DC: Debug
        + Hash
        + Ord
        + Add<DC, Output = DC>
        + Sub<DC, Output = DC>
        + Div<f64, Output = DC>
        + Copy
        + Default
        + Send
        + Sync,
    H: Heuristic<TS, S, A, C, DC> + HeuristicBuilder<TS, S, A, C, DC> + Send + Sync,
{
    /// Creates a new lifelong planner for the given transition system and initial states.
    ///
    /// # Arguments
    ///
    /// * `transition_system` - The transition system in which the agents navigate.
    /// * `initial_states` - The initial states of the agents.
    /// * `initial_cost` - The initial cost of the agents.
    /// * `collision_precision` - The precision used to detect collisions.
    pub fn new(
        transition_system: Arc<TS>,
        initial_states: Vec<S>,
        initial_cost: C,
        collision_precision: DC,
    ) -> Self {
        let solver = ConflictBasedSearch::new(transition_system.clone());
        let mut tasks = vec![];
        let mut solutions = vec![];
        let mut heuristic_to_pivots = vec![];
        for initial_state in initial_states {
            let task = Arc::new(Task::new(
                initial_state.clone(),
                initial_state.clone(),
                initial_cost,
            ));
            heuristic_to_pivots.push(Arc::new(ReverseResumableAStar::new(
                transition_system.clone(),
                task.clone(),
                H::build(transition_system.clone(), Arc::new(task.reverse())),
            )));
            // Each agent stands still at its initial position.
            solutions.push(Solution {
                cost: C::default(),
                steps: vec![(
                    Arc::new(SippState {
                        safe_interval: Interval::default(),
                        internal_state: initial_state,
                    }),
                    initial_cost,
                )],
                actions: vec![],
            });
            tasks.push(task);
        }

        Self {
            transition_system,
            solver,
            tasks,
            solutions,
            heuristic_to_pivots,
            collision_precision,
        }
    }

    /// Plan optimal paths to complete the given tasks,
    /// while avoiding other currently executing tasks.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration of the planning request, specifying the new tasks to solve.
    pub fn solve(
        &mut self,
        config: &LifelongConfig<S, C>,
    ) -> Option<&Vec<Solution<Arc<SippState<S, C, DC>>, A, C, DC>>> {
        for (agent, task) in &config.tasks {
            self.tasks[*agent] = task.clone();
            self.heuristic_to_pivots[*agent] = Arc::new(ReverseResumableAStar::new(
                self.transition_system.clone(),
                task.clone(),
                H::build(self.transition_system.clone(), Arc::new(task.reverse())),
            ));
        }

        let mut cbs_config = CbsConfig::new_with_pivots(
            self.tasks.clone(),
            Arc::new(self.tasks.iter().map(|t| t.goal_state.clone()).collect()),
            Arc::new(self.heuristic_to_pivots.clone()),
            self.collision_precision,
            config.n_threads,
            None,
        );

        for agent in 0..self.tasks.len() {
            if !config.tasks.contains_key(&agent) {
                cbs_config.add_frozen(agent, self.solutions[agent].clone());
            }
        }

        if let Some(solutions) = self.solver.solve(&cbs_config) {
            self.solutions = solutions;
            Some(&self.solutions)
        } else {
            None
        }
    }
}

/// The input configuration for a new planning request.
pub struct LifelongConfig<S, C>
where
    S: State + Eq + Clone,
    C: Copy,
{
    /// The tasks to solve.
    pub tasks: FxHashMap<usize, Arc<Task<S, C>>>, // TODO: only require new destination and use current position as initial state?
    /// The number of threads to use.
    pub n_threads: usize,
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use fxhash::FxHashMap;
    use ordered_float::OrderedFloat;

    use crate::{
        simple_graph, GraphEdgeId, GraphNodeId, Lifelong, LifelongConfig, SimpleHeuristic,
        SimpleState, SimpleWorld, Task,
    };

    #[test]
    fn test_simple() {
        let size = 10;
        let graph = simple_graph(size);
        let transition_system = Arc::new(SimpleWorld::new(graph, 0.4));

        let initial_states = vec![
            SimpleState(GraphNodeId(0)),
            SimpleState(GraphNodeId(1)),
            SimpleState(GraphNodeId(2)),
        ];

        let mut planner: Lifelong<
            SimpleWorld,
            SimpleState,
            GraphEdgeId,
            OrderedFloat<f64>,
            OrderedFloat<f64>,
            SimpleHeuristic,
        > = Lifelong::new(
            transition_system,
            initial_states,
            OrderedFloat(0.0),
            OrderedFloat(1e-6),
        );

        let mut config = LifelongConfig {
            tasks: FxHashMap::default(),
            n_threads: 1,
        };

        config.tasks.insert(
            0,
            Arc::new(Task::new(
                SimpleState(GraphNodeId(0)),
                SimpleState(GraphNodeId(9)),
                OrderedFloat(0.0),
            )),
        );
        config.tasks.insert(
            2,
            Arc::new(Task::new(
                SimpleState(GraphNodeId(2)),
                SimpleState(GraphNodeId(8)),
                OrderedFloat(0.0),
            )),
        );

        let solutions = planner.solve(&config).unwrap();

        assert_eq!(solutions[0].cost + solutions[2].cost, OrderedFloat(17.0));
        assert!(solutions[1].actions.is_empty());
    }
}
