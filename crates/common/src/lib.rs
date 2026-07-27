//! `cueforge-common`
//!
//! Shared types: units (Meters, Seconds, ...), seeded RNG, math helpers.
//!
//! See `docs/architecture/Overview.md` for how this crate fits into the
//! overall CueForge architecture.

pub mod math;
pub mod rng;
pub mod units;

pub use math::{Vec2, Vec3};
pub use rng::RngState;
pub use units::{Kilograms, Meters, MetersPerSecond, Newtons, Radians, RadiansPerSecond, Seconds};
