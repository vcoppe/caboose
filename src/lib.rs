//! CaBooSe (Conflict-Based Search) is a crate implementing the
//! *Continuous Conflict-Based Search* algorithm (CCBS) for the
//! *Multi-Agent Path Finding* problem (MAPF).
//! It uses the *Safe Interval Path Planning* algorithm (SIPP)
//! algorithm under the hood to plan individual paths while avoiding
//! already processed collisions. Furthermore, SIPP itself uses the
//! *Reverse Resumable A\** algorithm (RRA*) to compute heuristics
//! for each task to solve.
//!
//! ## Usage
//!
//! The main entry point of the crate is the [`ConflictBasedSearch`] struct, which
//! requires a [`TransitionSystem`] to be created. It can then be used to plan paths
//! for a set of agents, by calling the [`ConflictBasedSearch::solve`] function on a
//! [`CbsConfig`] that gathers all the information needed to solve the problem.
//!
//! ### Example
//!
//! ```rust
//! use ordered_float::OrderedFloat;
//! use std::sync::Arc;
//! use caboose::{
//!    simple_graph, SimpleWorld, SimpleState, SimpleHeuristic, GraphNodeId, GraphEdgeId,
//!    ConflictBasedSearch, CbsConfig, Task,
//! };
//!
//! let size = 10;
//! let graph = simple_graph(size);
//! let transition_system = Arc::new(SimpleWorld::new(graph, 0.4));
//! let tasks = vec![
//!     Arc::new(Task::new(
//!         SimpleState(GraphNodeId(0)),
//!         SimpleState(GraphNodeId(9)),
//!         OrderedFloat(0.0),
//!     )),
//!     Arc::new(Task::new(
//!         SimpleState(GraphNodeId(9)),
//!         SimpleState(GraphNodeId(0)),
//!         OrderedFloat(0.0),
//!     )),
//! ];
//! let config: CbsConfig<
//!     SimpleWorld,
//!     SimpleState,
//!     GraphEdgeId,
//!     OrderedFloat<f64>,
//!     OrderedFloat<f64>,
//!     SimpleHeuristic,
//! > = CbsConfig::new(
//!     transition_system.clone(),
//!     tasks,
//!     OrderedFloat(1e-6),
//!     1,
//!     None,
//! );
//! let mut solver = ConflictBasedSearch::new(transition_system.clone());
//! let solutions = solver.solve(&config).unwrap();
//! ```
//!
//! ## Lifelong
//!
//! A second entry point of the crate is the [`Lifelong`] struct, which
//! can process a stream of planning requests. Each of those requests can ask
//! for new paths for a subset of agents, and those must avoid collisions with
//! the last planned paths for the other agents.
//!
//! ## Genericity
//!
//! Caboose allows to specify new environments by implementing the [`TransitionSystem`] trait.
//! It reasons over:
//! - a set of states, implementing the [`State`] trait, that describe the state of an agent;
//! - a set of actions that describe the possible moves of an agent, depending on its state;
//!
//! and requires implementing a few functions that link them together, in particular:
//! - [`TransitionSystem::actions_from`] and [`TransitionSystem::transition`] to describe the
//!  result of applying an action to a state;
//! - [`TransitionSystem::transition_cost`] to describe the cost of applying an action to
//! a state (i.e. the duration of the action);
//! - [`TransitionSystem::conflict`] that checks whether two actions performed by two different
//! agents lead to a collision.
//!
//! You will probably also need to create a simple heuristic for your environment by implementing the [`Heuristic`] trait.
//! This heuristic will be used by the search algorithm to guide the search, but should be simplistic
//! as it will only be used to guide the search for a more advanced heuristic given by [`ReverseResumableAStar`].

mod abstraction;
mod search;
mod util;
mod world;

pub use abstraction::*;
pub use search::*;
pub use util::*;
pub use world::*;
