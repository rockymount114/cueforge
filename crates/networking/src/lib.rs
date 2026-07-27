//! `cueforge-networking`
//!
//! Lockstep deterministic network frame packet structure and checksum validation.

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
