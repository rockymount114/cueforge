//! `cueforge-rules`
//!
//! Game variant rules: 8-ball, 9-ball (WPA spec), straight pool, foul evaluation, scoring state machine.

use cueforge_physics::{BallId, Event};

/// Pool game variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameVariant {
    EightBall,
    NineBall,
    StraightPool,
}

/// Ball group classification in 8-Ball.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BallGroup {
    Solids,    // Balls 1..=7
    Stripes,   // Balls 9..=15
    EightBall, // Ball 8
}

impl BallGroup {
    pub fn for_ball(id: BallId) -> Option<Self> {
        match id.0 {
            1..=7 => Some(BallGroup::Solids),
            8 => Some(BallGroup::EightBall),
            9..=15 => Some(BallGroup::Stripes),
            _ => None,
        }
    }
}

/// Foul types in cue sports based on WPA standards.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoulType {
    /// Cue ball scratch (pocketed).
    Scratch,
    /// No ball was hit by cue ball.
    NoContact,
    /// Cue ball hit illegal ball first (rotation violation).
    WrongBallFirst,
    /// No rail driven after initial contact and no ball pocketed.
    NoRailContact,
}

/// Outcome of evaluating physical events from a shot.
#[derive(Debug, Clone, PartialEq)]
pub struct ShotResult {
    pub cue_ball_scratched: bool,
    pub first_ball_struck: Option<BallId>,
    pub pocketed_balls: Vec<BallId>,
    pub rail_hit_after_contact: bool,
    pub fouls: Vec<FoulType>,
    pub is_valid: bool,
}

/// Outcome of a turn after evaluating rules state machine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TurnOutcome {
    KeepTurn,
    SwitchTurn { ball_in_hand: bool },
    PushOutOffered,
    GameOver { winner: usize },
}

/// State container for an 8-Ball match.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EightBallState {
    pub table_open: bool,
    pub player_groups: [Option<BallGroup>; 2],
    pub active_player: usize,
    pub winner: Option<usize>,
}

impl Default for EightBallState {
    fn default() -> Self {
        Self {
            table_open: true,
            player_groups: [None, None],
            active_player: 0,
            winner: None,
        }
    }
}

/// Comprehensive WPA 9-Ball state machine based on `test_result.md`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NineBallState {
    pub lowest_active_ball: u32,
    pub consecutive_fouls: [u8; 2],
    pub active_player: usize,
    pub winner: Option<usize>,
    pub is_break_shot: bool,
    pub push_out_available: bool,
    pub push_out_active: bool,
    pub wpa_spot_9ball_on_break: bool,
    pub respawn_9ball_needed: bool,
}

impl Default for NineBallState {
    fn default() -> Self {
        Self {
            lowest_active_ball: 1,
            consecutive_fouls: [0, 0],
            active_player: 0,
            winner: None,
            is_break_shot: true,
            push_out_available: false,
            push_out_active: false,
            wpa_spot_9ball_on_break: true,
            respawn_9ball_needed: false,
        }
    }
}

impl NineBallState {
    /// Announce a push-out shot immediately after a legal break.
    pub fn announce_push_out(&mut self) -> Result<(), &'static str> {
        if !self.push_out_available {
            return Err("Push out is only available immediately after a legal break shot.");
        }
        self.push_out_active = true;
        self.push_out_available = false;
        Ok(())
    }

    /// Opponent option to pass turn back to push-out shooter.
    pub fn pass_push_out_shot_back(&mut self) {
        self.active_player = 1 - self.active_player;
    }
}

/// State container for a Straight Pool (14.1 Continuous) match.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StraightPoolState {
    pub scores: [i32; 2],
    pub target_score: i32,
    pub consecutive_fouls: [u8; 2],
    pub active_player: usize,
    pub winner: Option<usize>,
    pub active_rack_count: u32,
}

impl Default for StraightPoolState {
    fn default() -> Self {
        Self {
            scores: [0, 0],
            target_score: 15,
            consecutive_fouls: [0, 0],
            active_player: 0,
            winner: None,
            active_rack_count: 1,
        }
    }
}

/// Unified Rule Engine evaluating events against active game state.
#[derive(Debug, Clone)]
pub struct RuleEngine {
    pub variant: GameVariant,
    pub cue_ball_id: BallId,
    pub eight_ball_state: EightBallState,
    pub nine_ball_state: NineBallState,
    pub straight_pool_state: StraightPoolState,
}

impl RuleEngine {
    pub fn new(variant: GameVariant, cue_ball_id: BallId) -> Self {
        Self {
            variant,
            cue_ball_id,
            eight_ball_state: EightBallState::default(),
            nine_ball_state: NineBallState::default(),
            straight_pool_state: StraightPoolState::default(),
        }
    }

    /// Evaluate raw physical events from a shot simulation.
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
        } else if pocketed_balls.is_empty() && !rail_hit_after_contact {
            // WPA Rule Section 8 / 19: After contact, if no ball is pocketed, a rail must be hit.
            fouls.push(FoulType::NoRailContact);
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

    /// Process turn progression for 8-Ball.
    pub fn process_eight_ball(&mut self, shot: &ShotResult) -> TurnOutcome {
        if let Some(w) = self.eight_ball_state.winner {
            return TurnOutcome::GameOver { winner: w };
        }

        let active = self.eight_ball_state.active_player;
        let opponent = 1 - active;

        // Check if 8-ball was pocketed
        if shot.pocketed_balls.contains(&BallId(8)) {
            if shot.cue_ball_scratched || !shot.fouls.is_empty() {
                // Loss if pocketed 8-ball on a foul/scratch
                self.eight_ball_state.winner = Some(opponent);
                return TurnOutcome::GameOver { winner: opponent };
            }
            self.eight_ball_state.winner = Some(active);
            return TurnOutcome::GameOver { winner: active };
        }

        if !shot.is_valid {
            self.eight_ball_state.active_player = opponent;
            return TurnOutcome::SwitchTurn { ball_in_hand: true };
        }

        // Group assignment if table is open
        if self.eight_ball_state.table_open && !shot.pocketed_balls.is_empty() {
            for ball in &shot.pocketed_balls {
                if let Some(group) = BallGroup::for_ball(*ball) {
                    if group != BallGroup::EightBall {
                        let opp_group = match group {
                            BallGroup::Solids => BallGroup::Stripes,
                            BallGroup::Stripes => BallGroup::Solids,
                            _ => unreachable!(),
                        };
                        self.eight_ball_state.player_groups[active] = Some(group);
                        self.eight_ball_state.player_groups[opponent] = Some(opp_group);
                        self.eight_ball_state.table_open = false;
                        break;
                    }
                }
            }
        }

        if !shot.pocketed_balls.is_empty() {
            TurnOutcome::KeepTurn
        } else {
            self.eight_ball_state.active_player = opponent;
            TurnOutcome::SwitchTurn {
                ball_in_hand: false,
            }
        }
    }

    /// Process turn progression for 9-Ball per WPA Official Rules (test_result.md).
    pub fn process_nine_ball(&mut self, shot: &ShotResult) -> TurnOutcome {
        if let Some(w) = self.nine_ball_state.winner {
            return TurnOutcome::GameOver { winner: w };
        }

        let active = self.nine_ball_state.active_player;
        let opponent = 1 - active;
        self.nine_ball_state.respawn_9ball_needed = false;

        // Push Out Shot Evaluation
        if self.nine_ball_state.push_out_active {
            self.nine_ball_state.push_out_active = false;
            self.nine_ball_state.push_out_available = false;
            self.nine_ball_state.active_player = opponent;
            return TurnOutcome::PushOutOffered;
        }

        let mut fouls = shot.fouls.clone();

        // Rotation rule check: Must hit lowest active ball first (Section 8 / Section 19)
        if let Some(first) = shot.first_ball_struck {
            if first.0 != self.nine_ball_state.lowest_active_ball
                && !fouls.contains(&FoulType::WrongBallFirst)
            {
                fouls.push(FoulType::WrongBallFirst);
            }
        } else if !fouls.contains(&FoulType::NoContact) {
            fouls.push(FoulType::NoContact);
        }

        let is_legal = fouls.is_empty();

        // Break Shot handling (Section 6 & Section 16)
        if self.nine_ball_state.is_break_shot {
            self.nine_ball_state.is_break_shot = false;

            if is_legal {
                self.nine_ball_state.push_out_available = true;
                if shot.pocketed_balls.contains(&BallId(9)) {
                    if self.nine_ball_state.wpa_spot_9ball_on_break {
                        // WPA Rule 16: Spot 9-ball on foot spot, shooter continues
                        self.nine_ball_state.respawn_9ball_needed = true;
                        return TurnOutcome::KeepTurn;
                    } else {
                        // League Rule: Immediate Win
                        self.nine_ball_state.winner = Some(active);
                        return TurnOutcome::GameOver { winner: active };
                    }
                }
            } else {
                self.nine_ball_state.push_out_available = false;
                if shot.pocketed_balls.contains(&BallId(9)) {
                    self.nine_ball_state.respawn_9ball_needed = true;
                }
                self.nine_ball_state.consecutive_fouls[active] += 1;
                self.nine_ball_state.active_player = opponent;
                return TurnOutcome::SwitchTurn { ball_in_hand: true };
            }
        } else {
            self.nine_ball_state.push_out_available = false;
        }

        // Foul Handling & 3-Foul Rule (Section 25)
        if !is_legal {
            self.nine_ball_state.consecutive_fouls[active] += 1;
            if shot.pocketed_balls.contains(&BallId(9)) {
                // Section 19: Spot 9-ball if pocketed on a foul
                self.nine_ball_state.respawn_9ball_needed = true;
            }

            if self.nine_ball_state.consecutive_fouls[active] >= 3 {
                // 3 consecutive fouls -> loss of rack/game
                self.nine_ball_state.winner = Some(opponent);
                return TurnOutcome::GameOver { winner: opponent };
            }

            self.nine_ball_state.active_player = opponent;
            return TurnOutcome::SwitchTurn { ball_in_hand: true };
        }

        // Reset foul counter on legal shot
        self.nine_ball_state.consecutive_fouls[active] = 0;

        // Legal 9-Ball Pocketed -> Immediate Win (Section 15)
        if shot.pocketed_balls.contains(&BallId(9)) {
            self.nine_ball_state.winner = Some(active);
            return TurnOutcome::GameOver { winner: active };
        }

        // Advance lowest active ball if potted
        let mut pocketed_lowest = false;
        for ball in &shot.pocketed_balls {
            if ball.0 == self.nine_ball_state.lowest_active_ball {
                pocketed_lowest = true;
            }
        }
        if pocketed_lowest {
            self.nine_ball_state.lowest_active_ball += 1;
        }

        if !shot.pocketed_balls.is_empty() {
            TurnOutcome::KeepTurn
        } else {
            self.nine_ball_state.active_player = opponent;
            TurnOutcome::SwitchTurn {
                ball_in_hand: false,
            }
        }
    }

    /// Process turn progression for Straight Pool (14.1 Continuous).
    pub fn process_straight_pool(&mut self, shot: &ShotResult) -> TurnOutcome {
        if let Some(w) = self.straight_pool_state.winner {
            return TurnOutcome::GameOver { winner: w };
        }

        let active = self.straight_pool_state.active_player;
        let opponent = 1 - active;

        if !shot.is_valid {
            self.straight_pool_state.scores[active] -= 1;
            self.straight_pool_state.consecutive_fouls[active] += 1;

            if self.straight_pool_state.consecutive_fouls[active] >= 3 {
                self.straight_pool_state.scores[active] -= 15;
                self.straight_pool_state.consecutive_fouls[active] = 0;
            }

            self.straight_pool_state.active_player = opponent;
            return TurnOutcome::SwitchTurn { ball_in_hand: true };
        }

        self.straight_pool_state.consecutive_fouls[active] = 0;
        let points = shot.pocketed_balls.len() as i32;
        self.straight_pool_state.scores[active] += points;

        if self.straight_pool_state.scores[active] >= self.straight_pool_state.target_score {
            self.straight_pool_state.winner = Some(active);
            return TurnOutcome::GameOver { winner: active };
        }

        if points > 0 {
            TurnOutcome::KeepTurn
        } else {
            self.straight_pool_state.active_player = opponent;
            TurnOutcome::SwitchTurn {
                ball_in_hand: false,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cueforge_physics::BallId;

    #[test]
    fn test_eight_ball_group_assignment() {
        let mut engine = RuleEngine::new(GameVariant::EightBall, BallId(0));
        let shot = ShotResult {
            cue_ball_scratched: false,
            first_ball_struck: Some(BallId(3)),
            pocketed_balls: vec![BallId(3)],
            rail_hit_after_contact: true,
            fouls: vec![],
            is_valid: true,
        };

        let outcome = engine.process_eight_ball(&shot);
        assert_eq!(outcome, TurnOutcome::KeepTurn);
        assert_eq!(
            engine.eight_ball_state.player_groups[0],
            Some(BallGroup::Solids)
        );
        assert_eq!(
            engine.eight_ball_state.player_groups[1],
            Some(BallGroup::Stripes)
        );
        assert!(!engine.eight_ball_state.table_open);
    }

    #[test]
    fn test_nine_ball_rotation_and_no_rail_foul() {
        let mut engine = RuleEngine::new(GameVariant::NineBall, BallId(0));
        engine.nine_ball_state.is_break_shot = false;
        engine.nine_ball_state.lowest_active_ball = 1;

        // Wrong ball hit first (hit 3 instead of 1)
        let wrong_ball_shot = ShotResult {
            cue_ball_scratched: false,
            first_ball_struck: Some(BallId(3)),
            pocketed_balls: vec![],
            rail_hit_after_contact: true,
            fouls: vec![],
            is_valid: true,
        };

        let outcome = engine.process_nine_ball(&wrong_ball_shot);
        assert_eq!(outcome, TurnOutcome::SwitchTurn { ball_in_hand: true });
        assert_eq!(engine.nine_ball_state.consecutive_fouls[0], 1);
    }

    #[test]
    fn test_nine_ball_push_out() {
        let mut engine = RuleEngine::new(GameVariant::NineBall, BallId(0));
        let break_shot = ShotResult {
            cue_ball_scratched: false,
            first_ball_struck: Some(BallId(1)),
            pocketed_balls: vec![BallId(2)],
            rail_hit_after_contact: true,
            fouls: vec![],
            is_valid: true,
        };

        engine.process_nine_ball(&break_shot);
        assert!(engine.nine_ball_state.push_out_available);

        assert!(engine.nine_ball_state.announce_push_out().is_ok());
        assert!(engine.nine_ball_state.push_out_active);

        let push_out_shot = ShotResult {
            cue_ball_scratched: false,
            first_ball_struck: None,
            pocketed_balls: vec![],
            rail_hit_after_contact: false,
            fouls: vec![FoulType::NoContact],
            is_valid: false,
        };

        let outcome = engine.process_nine_ball(&push_out_shot);
        assert_eq!(outcome, TurnOutcome::PushOutOffered);
    }

    #[test]
    fn test_nine_ball_wpa_spot_on_break() {
        let mut engine = RuleEngine::new(GameVariant::NineBall, BallId(0));
        let break_9ball_shot = ShotResult {
            cue_ball_scratched: false,
            first_ball_struck: Some(BallId(1)),
            pocketed_balls: vec![BallId(9)],
            rail_hit_after_contact: true,
            fouls: vec![],
            is_valid: true,
        };

        let outcome = engine.process_nine_ball(&break_9ball_shot);
        assert_eq!(outcome, TurnOutcome::KeepTurn);
        assert!(engine.nine_ball_state.respawn_9ball_needed);
        assert_eq!(engine.nine_ball_state.winner, None);
    }

    #[test]
    fn test_nine_ball_three_foul_rule() {
        let mut engine = RuleEngine::new(GameVariant::NineBall, BallId(0));
        let foul_shot = ShotResult {
            cue_ball_scratched: true,
            first_ball_struck: None,
            pocketed_balls: vec![],
            rail_hit_after_contact: false,
            fouls: vec![FoulType::Scratch, FoulType::NoContact],
            is_valid: false,
        };

        engine.process_nine_ball(&foul_shot);
        engine.nine_ball_state.active_player = 0; // Force same player for 3 fouls test
        engine.process_nine_ball(&foul_shot);
        engine.nine_ball_state.active_player = 0;
        let outcome = engine.process_nine_ball(&foul_shot);

        assert_eq!(outcome, TurnOutcome::GameOver { winner: 1 });
    }

    #[test]
    fn test_straight_pool_scoring() {
        let mut engine = RuleEngine::new(GameVariant::StraightPool, BallId(0));
        let shot = ShotResult {
            cue_ball_scratched: false,
            first_ball_struck: Some(BallId(1)),
            pocketed_balls: vec![BallId(1), BallId(2)],
            rail_hit_after_contact: true,
            fouls: vec![],
            is_valid: true,
        };

        let outcome = engine.process_straight_pool(&shot);
        assert_eq!(outcome, TurnOutcome::KeepTurn);
        assert_eq!(engine.straight_pool_state.scores[0], 2);
    }
}
