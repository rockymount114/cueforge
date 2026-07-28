//! `cueforge-trainer`
//!
//! Interactive training drills, drill catalog, progress scoring, and skill rating.

use cueforge_ai::PositionPrediction;
use cueforge_common::{Vec2, Vec3};
use cueforge_physics::{Ball, BallId, World, WorldConfig};
use cueforge_rules::ShotResult;

/// Drill difficulty category.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrillCategory {
    PottingAccuracy,
    PositionControl,
    WagonWheel,
    Safety,
}

/// Evaluation result of a single drill attempt.
#[derive(Debug, Clone, PartialEq)]
pub struct DrillAttemptResult {
    pub target_potted: bool,
    pub positional_error_m: f64,
    pub score_points: u32,
    pub max_possible_points: u32,
}

/// Comprehensive training drill definition.
#[derive(Debug, Clone, PartialEq)]
pub struct DrillSetup {
    pub name: String,
    pub description: String,
    pub category: DrillCategory,
    pub target_position: Vec2,
    pub balls: Vec<Ball>,
}

impl DrillSetup {
    /// Create a standard straight-line pot drill.
    pub fn straight_line_drill() -> Self {
        let cue_ball = Ball::new(BallId(0), Vec3::new(0.0, -0.5, 0.0));
        let object_ball = Ball::new(BallId(1), Vec3::new(0.0, 0.5, 0.0));

        Self {
            name: "Straight Line Pot & Stop".into(),
            description:
                "Practice potting object ball straight into corner pocket and stopping cue ball."
                    .into(),
            category: DrillCategory::PottingAccuracy,
            target_position: Vec2::new(0.0, 0.5),
            balls: vec![cue_ball, object_ball],
        }
    }

    /// Create a Wagon Wheel positional drill.
    pub fn wagon_wheel_drill() -> Self {
        let cue_ball = Ball::new(BallId(0), Vec3::new(0.0, 0.0, 0.0));
        let ball1 = Ball::new(BallId(1), Vec3::new(-0.3, 0.4, 0.0));
        let ball2 = Ball::new(BallId(2), Vec3::new(0.0, 0.5, 0.0));
        let ball3 = Ball::new(BallId(3), Vec3::new(0.3, 0.4, 0.0));

        Self {
            name: "Wagon Wheel Position Control".into(),
            description:
                "Pot center ball and control cue ball angle to align for subsequent targets.".into(),
            category: DrillCategory::WagonWheel,
            target_position: Vec2::new(0.0, -0.2),
            balls: vec![cue_ball, ball1, ball2, ball3],
        }
    }

    /// Load drill setup into a fresh simulation world.
    pub fn build_world(&self) -> World {
        let mut world = World::new(WorldConfig::default());
        for ball in &self.balls {
            world.add_ball(ball.clone());
        }
        world
    }

    /// Evaluate player's attempt against target position and shot result.
    pub fn evaluate_attempt(
        &self,
        shot_result: &ShotResult,
        prediction: &PositionPrediction,
    ) -> DrillAttemptResult {
        let target_potted = shot_result.is_valid && !shot_result.pocketed_balls.is_empty();
        let error = (prediction.final_cue_position - self.target_position).length();

        let mut score = 0;
        if target_potted {
            score += 50;
            if error < 0.10 {
                score += 50;
            } else if error < 0.25 {
                score += 30;
            } else if error < 0.50 {
                score += 10;
            }
        }

        DrillAttemptResult {
            target_potted,
            positional_error_m: error,
            score_points: score,
            max_possible_points: 100,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drill_setup_and_evaluation() {
        let drill = DrillSetup::straight_line_drill();
        let world = drill.build_world();
        assert_eq!(world.balls.len(), 2);

        let shot_result = ShotResult {
            cue_ball_scratched: false,
            first_ball_struck: Some(BallId(1)),
            pocketed_balls: vec![BallId(1)],
            rail_hit_after_contact: true,
            fouls: vec![],
            is_valid: true,
        };

        let prediction = PositionPrediction {
            final_cue_position: Vec2::new(0.0, 0.48),
            total_ticks_simulated: 100,
            balls_pocketed: vec![1],
        };

        let result = drill.evaluate_attempt(&shot_result, &prediction);
        assert!(result.target_potted);
        assert_eq!(result.score_points, 100);
    }
}
