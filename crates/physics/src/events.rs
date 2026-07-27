//! Event representation emitted by the simulation core during physical updates.

use crate::ball::{BallId, MotionState};
use cueforge_common::Seconds;

/// Events generated during simulation stepping.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// Collision between two balls.
    BallBallCollision {
        ball1: BallId,
        ball2: BallId,
        time: Seconds,
        relative_speed: f64,
    },
    /// Collision between a ball and a table rail cushion.
    BallRailCollision {
        ball: BallId,
        rail_index: usize,
        time: Seconds,
    },
    /// Ball captured by a pocket.
    PocketCapture {
        ball: BallId,
        pocket_index: usize,
        time: Seconds,
    },
    /// Ball changed motion state.
    StateTransition {
        ball: BallId,
        from: MotionState,
        to: MotionState,
        time: Seconds,
    },
}
