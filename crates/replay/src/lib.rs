//! `cueforge-replay`
//!
//! Deterministic replay recording, frame playback controller, and timeline seeking.

use cueforge_cue::CueStrike;
use cueforge_physics::{Ball, Event};

/// Frame snapshot recorded at a specific simulation tick.
#[derive(Debug, Clone, PartialEq)]
pub struct ReplayFrame {
    pub tick: u64,
    pub balls: Vec<Ball>,
    pub events: Vec<Event>,
}

/// A recorded shot session containing initial state, input strike, and per-tick frame keyframes.
#[derive(Debug, Clone, PartialEq)]
pub struct ShotReplay {
    pub initial_balls: Vec<Ball>,
    pub strike: CueStrike,
    pub recorded_events: Vec<Event>,
    pub frames: Vec<ReplayFrame>,
}

impl ShotReplay {
    pub fn new(initial_balls: Vec<Ball>, strike: CueStrike, recorded_events: Vec<Event>) -> Self {
        Self {
            initial_balls,
            strike,
            recorded_events,
            frames: Vec::new(),
        }
    }

    pub fn push_frame(&mut self, tick: u64, balls: Vec<Ball>, events: Vec<Event>) {
        self.frames.push(ReplayFrame {
            tick,
            balls,
            events,
        });
    }

    pub fn total_frames(&self) -> usize {
        self.frames.len()
    }
}

/// Playback control state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackState {
    Playing,
    Paused,
    Stopped,
}

/// Controller for stepping, seeking, and playing back recorded replays.
#[derive(Debug, Clone)]
pub struct ReplayController {
    pub replay: ShotReplay,
    pub current_frame_index: usize,
    pub playback_state: PlaybackState,
    pub playback_speed: f64,
}

impl ReplayController {
    pub fn new(replay: ShotReplay) -> Self {
        Self {
            replay,
            current_frame_index: 0,
            playback_state: PlaybackState::Stopped,
            playback_speed: 1.0,
        }
    }

    pub fn play(&mut self) {
        self.playback_state = PlaybackState::Playing;
    }

    pub fn pause(&mut self) {
        self.playback_state = PlaybackState::Paused;
    }

    pub fn stop(&mut self) {
        self.playback_state = PlaybackState::Stopped;
        self.current_frame_index = 0;
    }

    pub fn seek_to_frame(&mut self, index: usize) -> Option<&ReplayFrame> {
        if index < self.replay.total_frames() {
            self.current_frame_index = index;
            Some(&self.replay.frames[self.current_frame_index])
        } else if !self.replay.frames.is_empty() {
            self.current_frame_index = self.replay.total_frames() - 1;
            Some(&self.replay.frames[self.current_frame_index])
        } else {
            None
        }
    }

    pub fn step_forward(&mut self) -> Option<&ReplayFrame> {
        if self.current_frame_index + 1 < self.replay.total_frames() {
            self.current_frame_index += 1;
            Some(&self.replay.frames[self.current_frame_index])
        } else {
            self.playback_state = PlaybackState::Stopped;
            self.replay.frames.last()
        }
    }

    pub fn step_backward(&mut self) -> Option<&ReplayFrame> {
        if self.current_frame_index > 0 {
            self.current_frame_index -= 1;
            Some(&self.replay.frames[self.current_frame_index])
        } else {
            self.replay.frames.first()
        }
    }

    pub fn current_frame(&self) -> Option<&ReplayFrame> {
        self.replay.frames.get(self.current_frame_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cueforge_common::Vec3;
    use cueforge_physics::BallId;

    fn sample_ball() -> Ball {
        Ball::new(BallId(0), Vec3::ZERO)
    }

    fn sample_strike() -> CueStrike {
        CueStrike::center_shot(2.0, 0.0)
    }

    #[test]
    fn test_replay_controller_seeking_and_stepping() {
        let mut replay = ShotReplay::new(vec![sample_ball()], sample_strike(), vec![]);
        for tick in 0..10 {
            replay.push_frame(tick, vec![sample_ball()], vec![]);
        }

        let mut controller = ReplayController::new(replay);
        assert_eq!(controller.current_frame_index, 0);

        controller.play();
        assert_eq!(controller.playback_state, PlaybackState::Playing);

        controller.step_forward();
        assert_eq!(controller.current_frame_index, 1);

        controller.seek_to_frame(5);
        assert_eq!(controller.current_frame_index, 5);

        controller.step_backward();
        assert_eq!(controller.current_frame_index, 4);
    }
}
