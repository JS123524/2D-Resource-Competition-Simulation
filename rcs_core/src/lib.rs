//! Core types and logic for a 2-D resource-competition simulation.
//!
//! This crate defines the main building blocks of the simulation:
//!
//! - [`Agent`]: mobile entities that consume resources and may die.
//! - [`Cell`]: resource storage and regeneration at each grid position.
//! - [`SimulationError`]: error type used by update and movement operations.
//! - [`Updatable`]: a common trait for types that advance one simulation step.
//! - [`World`]: the grid of cells and agents, plus the step logic.
//! - [`WorldConfig`]: configuration for constructing a randomized world.

pub mod agent;
pub mod cell;
pub mod errors;
pub mod traits;
pub mod world;

pub use agent::Agent;
pub use cell::Cell;
pub use errors::SimulationError;
pub use traits::Updatable;
pub use world::{World, WorldConfig};
