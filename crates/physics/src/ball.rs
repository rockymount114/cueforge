//! Ball state and motion representation.

use cueforge_common::{Kilograms, Meters, Vec3};

/// Unique identifier for a ball on the table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BallId(pub u32);

/// Motion state of a billiard ball.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionState {
    /// Ball is completely at rest.
    Stationary,
    /// Ball is sliding across the cloth surface (linear velocity != R * angular velocity vector).
    Sliding,
    /// Ball is in pure rolling motion without slipping.
    Rolling,
    /// Ball has fallen into a pocket and is no longer active on the cloth.
    Pocketed,
}

/// Representation of a pool/billiards ball.
#[derive(Debug, Clone, PartialEq)]
pub struct Ball {
    pub id: BallId,
    pub radius: Meters,
    pub mass: Kilograms,
    pub position: Vec3,
    pub velocity: Vec3,
    pub angular_velocity: Vec3,
    pub state: MotionState,
}

impl Ball {
    /// Create a standard regulation pool ball at given position.
    pub fn new(id: BallId, position: Vec3) -> Self {
        Self {
            id,
            // Regulation pool ball radius = 0.028575 m (2.25 in diameter)
            radius: Meters(0.028575),
            // Regulation pool ball mass = 0.170 kg (6.0 oz)
            mass: Kilograms(0.170),
            position,
            velocity: Vec3::ZERO,
            angular_velocity: Vec3::ZERO,
            state: MotionState::Stationary,
        }
    }

    /// Check if the ball is currently active on the table.
    pub fn is_active(&self) -> bool {
        self.state != MotionState::Pocketed
    }

    /// Check if the ball is moving (linear velocity or spin above threshold).
    pub fn is_moving(&self) -> bool {
        self.state == MotionState::Sliding || self.state == MotionState::Rolling
    }

    /// Compute velocity of the contact point between the ball and the table cloth (z=0).
    pub fn cloth_contact_velocity(&self) -> Vec3 {
        // Contact point relative to center of ball is r_c = (0, 0, -radius)
        let r_c = Vec3::new(0.0, 0.0, -self.radius.0);
        self.velocity + self.angular_velocity.cross(r_c)
    }
}
