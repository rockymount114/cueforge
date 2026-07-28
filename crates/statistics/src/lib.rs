//! `cueforge-statistics`
//!
//! Advanced shot, positional accuracy, safety, and match telemetry tracking.

use cueforge_common::Vec2;
use cueforge_rules::ShotResult;

/// 10x20 spatial grid for shot landing heatmap density tracking.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeatmapGrid {
    pub rows: usize,
    pub cols: usize,
    pub counts: Vec<Vec<u32>>,
}

impl Default for HeatmapGrid {
    fn default() -> Self {
        Self {
            rows: 10,
            cols: 20,
            counts: vec![vec![0; 20]; 10],
        }
    }
}

impl HeatmapGrid {
    pub fn record_position(&mut self, pos: Vec2, table_width: f64, table_length: f64) {
        let half_w = table_width / 2.0;
        let half_l = table_length / 2.0;

        let norm_x = ((pos.x + half_w) / table_width).clamp(0.0, 0.9999);
        let norm_y = ((pos.y + half_l) / table_length).clamp(0.0, 0.9999);

        let col = (norm_x * self.cols as f64) as usize;
        let row = (norm_y * self.rows as f64) as usize;

        self.counts[row][col] += 1;
    }
}

/// Comprehensive match and player statistics tracker.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct StatisticsTracker {
    pub total_shots: u32,
    pub successful_pots: u32,
    pub fouls_committed: u32,
    pub safeties_attempted: u32,
    pub safeties_successful: u32,
    pub cue_ball_distance_traveled: f64,
    pub positional_errors: Vec<f64>,
    pub pocket_distribution: [u32; 6],
    pub landing_heatmap: HeatmapGrid,
}

impl StatisticsTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_shot(
        &mut self,
        result: &ShotResult,
        cue_distance: f64,
        final_cue_pos: Vec2,
        table_width: f64,
        table_length: f64,
    ) {
        self.total_shots += 1;
        self.cue_ball_distance_traveled += cue_distance;

        if result.is_valid && !result.pocketed_balls.is_empty() {
            self.successful_pots += result.pocketed_balls.len() as u32;
        }

        if !result.is_valid {
            self.fouls_committed += 1;
        }

        self.landing_heatmap
            .record_position(final_cue_pos, table_width, table_length);
    }

    pub fn record_positional_accuracy(&mut self, target_pos: Vec2, actual_pos: Vec2) {
        let error = (target_pos - actual_pos).length();
        self.positional_errors.push(error);
    }

    pub fn record_safety_attempt(&mut self, is_successful: bool) {
        self.safeties_attempted += 1;
        if is_successful {
            self.safeties_successful += 1;
        }
    }

    pub fn pot_accuracy(&self) -> f64 {
        if self.total_shots == 0 {
            0.0
        } else {
            (self.successful_pots as f64) / (self.total_shots as f64)
        }
    }

    pub fn average_positional_error(&self) -> f64 {
        if self.positional_errors.is_empty() {
            0.0
        } else {
            self.positional_errors.iter().sum::<f64>() / (self.positional_errors.len() as f64)
        }
    }

    pub fn safety_success_rate(&self) -> f64 {
        if self.safeties_attempted == 0 {
            0.0
        } else {
            (self.safeties_successful as f64) / (self.safeties_attempted as f64)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statistics_heatmap_and_metrics() {
        let mut tracker = StatisticsTracker::new();
        let shot = ShotResult {
            cue_ball_scratched: false,
            first_ball_struck: None,
            pocketed_balls: vec![],
            rail_hit_after_contact: true,
            fouls: vec![],
            is_valid: true,
        };

        tracker.record_shot(&shot, 1.5, Vec2::new(0.0, 0.0), 1.27, 2.54);
        tracker.record_positional_accuracy(Vec2::new(0.0, 0.0), Vec2::new(0.1, 0.0));
        tracker.record_safety_attempt(true);

        assert_eq!(tracker.total_shots, 1);
        assert!((tracker.average_positional_error() - 0.1).abs() < 1e-5);
        assert_eq!(tracker.safety_success_rate(), 1.0);
        assert_eq!(tracker.landing_heatmap.counts[5][10], 1);
    }
}
