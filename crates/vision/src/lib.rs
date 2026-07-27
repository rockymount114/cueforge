//! `cueforge-vision`
//!
//! Interface types for computer-vision input integration.

use cueforge_common::Vec2;

/// Ball detection result from physical camera tracker.
#[derive(Debug, Clone, PartialEq)]
pub struct DetectedBall {
    pub ball_id: u32,
    pub table_position: Vec2,
    pub confidence: f64,
}
