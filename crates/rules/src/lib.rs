//! `cueforge-rules`
//!
//! Game variant rules: 8-ball, 9-ball, straight pool, foul evaluation, scoring state.

use cueforge_physics::{BallId, Event};

/// Pool game variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameVariant {
    EightBall,
    NineBall,
    StraightPool,
}

/// Foul types in cue sports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoulType {
    /// Cue ball scratch (pocketed).
    Scratch,
    /// No ball was hit by cue ball.
    NoContact,
    /// Cue ball hit illegal ball first.
    WrongBallFirst,
    /// No rail driven after contact.
    NoRailContact,
}

/// Outcome of evaluating a shot's physical events.
#[derive(Debug, Clone, PartialEq)]
pub struct ShotResult {
    pub cue_ball_scratched: bool,
    pub first_ball_struck: Option<BallId>,
    pub pocketed_balls: Vec<BallId>,
    pub rail_hit_after_contact: bool,
    pub fouls: Vec<FoulType>,
    pub is_valid: bool,
}

/// Rule engine evaluating events against active game state.
#[derive(Debug, Clone)]
pub struct RuleEngine {
    pub variant: GameVariant,
    pub cue_ball_id: BallId,
    pub active_player: usize,
    pub player_scores: [u32; 2],
}

impl RuleEngine {
    pub fn new(variant: GameVariant, cue_ball_id: BallId) -> Self {
        Self {
            variant,
            cue_ball_id,
            active_player: 0,
            player_scores: [0, 0],
        }
    }

    /// Evaluate shot events stream produced by `World::step`.
    pub fn evaluate_shot(&self, events: &[Event]) -> ShotResult {
        let mut cue_ball_scratched = false;
        let mut first_ball_struck = None;
        let mut pocketed_balls = Vec::new();
        let mut rail_hit_after_contact = false;
        let mut fouls = Vec::new();

        for event in events {
            match event {
                Event::BallBallCollision { ball1, ball2, .. } => {
                    if first_ball_struck.is_none() {
                        if *ball1 == self.cue_ball_id {
                            first_ball_struck = Some(*ball2);
                        } else if *ball2 == self.cue_ball_id {
                            first_ball_struck = Some(*ball1);
                        }
                    }
                }
                Event::BallRailCollision { .. } => {
                    if first_ball_struck.is_some() {
                        rail_hit_after_contact = true;
                    }
                }
                Event::PocketCapture { ball, .. } => {
                    if *ball == self.cue_ball_id {
                        cue_ball_scratched = true;
                    } else {
                        pocketed_balls.push(*ball);
                    }
                }
                _ => {}
            }
        }

        if cue_ball_scratched {
            fouls.push(FoulType::Scratch);
        }

        if first_ball_struck.is_none() {
            fouls.push(FoulType::NoContact);
        }

        let is_valid = fouls.is_empty();

        ShotResult {
            cue_ball_scratched,
            first_ball_struck,
            pocketed_balls,
            rail_hit_after_contact,
            fouls,
            is_valid,
        }
    }
}
