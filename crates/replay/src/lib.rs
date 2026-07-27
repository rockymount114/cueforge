//! `cueforge-replay`
//!
//! Deterministic replay recording & playback.

use cueforge_cue::CueStrike;
use cueforge_physics::{Ball, Event};

/// A recorded shot session for playback.
#[derive(Debug, Clone, PartialEq)]
pub struct ShotReplay {
    pub initial_balls: Vec<Ball>,
    pub strike: CueStrike,
    pub recorded_events: Vec<Event>,
}

impl ShotReplay {
    pub fn new(initial_balls: Vec<Ball>, strike: CueStrike, recorded_events: Vec<Event>) -> Self {
        Self {
            initial_balls,
            strike,
            recorded_events,
        }
    }
}
