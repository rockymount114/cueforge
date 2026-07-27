//! `cueforge-physics`
//!
//! Deterministic simulation core: World, fixed-timestep step function, and event model.
//!
//! See `docs/architecture/Overview.md` for how this crate fits into the
//! overall CueForge architecture.

pub mod ball;
pub mod events;
pub mod world;

pub use ball::{Ball, BallId, MotionState};
pub use events::Event;
pub use world::{World, WorldConfig};
