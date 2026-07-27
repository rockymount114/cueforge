//! `cueforge-statistics`
//!
//! Shot and match statistics tracking.

use cueforge_rules::ShotResult;

/// Match statistics container.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct StatisticsTracker {
    pub total_shots: u32,
    pub successful_pots: u32,
    pub fouls_committed: u32,
    pub cue_ball_distance_traveled: f64,
}

impl StatisticsTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_shot(&mut self, result: &ShotResult, cue_distance: f64) {
        self.total_shots += 1;
        self.cue_ball_distance_traveled += cue_distance;
        if result.is_valid && !result.pocketed_balls.is_empty() {
            self.successful_pots += result.pocketed_balls.len() as u32;
        }
        if !result.is_valid {
            self.fouls_committed += 1;
        }
    }

    pub fn pot_accuracy(&self) -> f64 {
        if self.total_shots == 0 {
            0.0
        } else {
            (self.successful_pots as f64) / (self.total_shots as f64)
        }
    }
}
