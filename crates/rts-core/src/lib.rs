//! Deterministic, renderer-independent simulation core.

pub mod command;
pub mod simulation;
pub mod state;

pub use command::Command;
pub use simulation::step;
pub use state::{GameState, GameTime, Unit, UnitId, Vec2};
