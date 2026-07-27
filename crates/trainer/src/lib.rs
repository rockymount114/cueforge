//! `cueforge-trainer`
//!
//! Drill generation and progress tracking.

use cueforge_common::Vec3;
use cueforge_physics::{Ball, BallId, World, WorldConfig};

/// Training drill setup.
#[derive(Debug, Clone, PartialEq)]
pub struct DrillSetup {
    pub name: String,
    pub description: String,
    pub balls: Vec<Ball>,
}

impl DrillSetup {
    /// Create a standard straight-line pot drill.
    pub fn straight_line_drill() -> Self {
        let cue_ball = Ball::new(BallId(0), Vec3::new(0.0, -0.5, 0.0));
        let object_ball = Ball::new(BallId(1), Vec3::new(0.0, 0.5, 0.0));

        Self {
            name: "Straight Line Speed Control".into(),
            description: "Practice potting object ball into foot corner pocket with precise cue ball positioning.".into(),
            balls: vec![cue_ball, object_ball],
        }
    }

    /// Load drill into a fresh simulation world.
    pub fn build_world(&self) -> World {
        let mut world = World::new(WorldConfig::default());
        for ball in &self.balls {
            world.add_ball(ball.clone());
        }
        world
    }
}
