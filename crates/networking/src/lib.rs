//! `cueforge-networking`
//!
//! Lockstep deterministic network frame packet structure, input buffering, and desync verification.

use cueforge_physics::World;

/// Network input packet representing a shot command.
#[derive(Debug, Clone, PartialEq)]
pub struct NetworkShotPacket {
    pub tick: u64,
    pub player_id: u32,
    pub azimuth: f64,
    pub speed: f64,
    pub offset_x: f64,
    pub offset_y: f64,
    pub elevation: f64,
}

/// Network frame payload containing turn inputs and state checksum.
#[derive(Debug, Clone, PartialEq)]
pub struct LockstepFrame {
    pub tick: u64,
    pub shot_packet: NetworkShotPacket,
    pub world_checksum: u64,
}

/// Desync validation result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesyncStatus {
    Synchronized,
    DesyncDetected {
        tick: u64,
        local_checksum: u64,
        remote_checksum: u64,
    },
}

/// Lockstep network session controller buffering remote turns and verifying determinism.
#[derive(Debug, Clone, Default)]
pub struct LockstepSession {
    pub local_player_id: u32,
    pub current_tick: u64,
    pub buffered_frames: Vec<LockstepFrame>,
    pub last_desync_status: Option<DesyncStatus>,
}

impl LockstepSession {
    pub fn new(local_player_id: u32) -> Self {
        Self {
            local_player_id,
            current_tick: 0,
            buffered_frames: Vec::new(),
            last_desync_status: None,
        }
    }

    pub fn push_remote_frame(&mut self, frame: LockstepFrame) {
        self.buffered_frames.push(frame);
    }

    pub fn verify_checksum(
        &mut self,
        tick: u64,
        local_world: &World,
        remote_checksum: u64,
    ) -> DesyncStatus {
        let local_checksum = compute_world_checksum(local_world);
        let status = if local_checksum == remote_checksum {
            DesyncStatus::Synchronized
        } else {
            DesyncStatus::DesyncDetected {
                tick,
                local_checksum,
                remote_checksum,
            }
        };
        self.last_desync_status = Some(status);
        status
    }
}

/// Compute 64-bit state checksum of simulation world for desync verification.
pub fn compute_world_checksum(world: &World) -> u64 {
    let mut hash = 14695981039346656037u64; // FNV-1a basis
    for ball in &world.balls {
        if !ball.is_active() {
            continue;
        }
        let px = ball.position.x.to_bits();
        let py = ball.position.y.to_bits();
        let vx = ball.velocity.x.to_bits();
        let vy = ball.velocity.y.to_bits();

        for bits in [px, py, vx, vy] {
            hash ^= bits;
            hash = hash.wrapping_mul(1099511628211u64);
        }
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use cueforge_common::Vec3;
    use cueforge_physics::{Ball, BallId, WorldConfig};

    #[test]
    fn test_lockstep_checksum_and_desync_detection() {
        let mut world1 = World::new(WorldConfig::default());
        world1.add_ball(Ball::new(BallId(0), Vec3::new(0.0, 0.0, 0.0)));

        let mut world2 = world1.clone();

        let checksum1 = compute_world_checksum(&world1);
        let checksum2 = compute_world_checksum(&world2);

        assert_eq!(checksum1, checksum2);

        let mut session = LockstepSession::new(0);
        let status = session.verify_checksum(1, &world1, checksum2);
        assert_eq!(status, DesyncStatus::Synchronized);

        // Introduce divergence in world2
        world2.balls[0].position.x += 0.001;
        let checksum2_div = compute_world_checksum(&world2);

        let status_div = session.verify_checksum(2, &world1, checksum2_div);
        assert!(matches!(status_div, DesyncStatus::DesyncDetected { .. }));
    }
}
