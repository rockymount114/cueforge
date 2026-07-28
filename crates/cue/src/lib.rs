//! `cueforge-cue`
//!
//! Cue stick mechanics and cue strike input application.

use cueforge_common::{Kilograms, Meters, MetersPerSecond, Radians, Vec3};
use cueforge_physics::{Ball, MotionState};

/// Physical properties of the cue stick based on `cue-specification.md`.
#[derive(Debug, Clone, PartialEq)]
pub struct CueStick {
    /// Overall cue length (58 in = 1.473 m).
    pub length: Meters,
    /// Cue mass (standard 19 oz ~ 0.5388 kg).
    pub mass: Kilograms,
    /// Cue tip radius (12.5 mm tip diameter / 2 = 0.00625 m).
    pub tip_radius: Meters,
    /// Maximum practical tip offset without miscue (10 mm = 0.010 m).
    pub max_tip_offset: Meters,
    /// Maximum break stroke speed (10.0 m/s).
    pub break_speed: MetersPerSecond,
}

impl Default for CueStick {
    fn default() -> Self {
        Self {
            length: Meters(1.473),
            mass: Kilograms(0.5388),
            tip_radius: Meters(0.00625),
            max_tip_offset: Meters(0.010),
            break_speed: MetersPerSecond(10.0),
        }
    }
}

/// Parameters defining a single stroke of the cue stick against the cue ball.
#[derive(Debug, Clone, PartialEq)]
pub struct CueStrike {
    /// Cue stick forward stroke speed (m/s).
    pub speed: MetersPerSecond,
    /// Aiming direction angle in radians (azimuth in X-Y table plane).
    pub azimuth: Radians,
    /// Cue elevation angle above table plane in radians (theta).
    pub elevation: Radians,
    /// Tip offset horizontal ratio [-1.0, 1.0] (left/right english).
    pub offset_x: f64,
    /// Tip offset vertical ratio [-1.0, 1.0] (topspin / draw).
    pub offset_y: f64,
}

impl CueStrike {
    /// Simple straight center-ball strike with given speed and aim angle.
    pub fn center_shot(speed_m_s: f64, azimuth_rad: f64) -> Self {
        Self {
            speed: MetersPerSecond(speed_m_s),
            azimuth: Radians(azimuth_rad),
            elevation: Radians(0.0),
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }
}

/// Apply a cue strike to a cue ball, transferring linear and angular momentum.
pub fn strike_cue_ball(cue: &CueStick, strike: &CueStrike, cue_ball: &mut Ball) {
    let r_ball = cue_ball.radius.0;
    let m_ball = cue_ball.mass.0;
    let m_cue = cue.mass.0;

    // Contact tip offset relative to ball radius (clamped to physical ball radius safety 0.85 R or max_tip_offset)
    let max_offset_ratio = (cue.max_tip_offset.0 / r_ball).min(0.85);
    let a = strike.offset_x.clamp(-max_offset_ratio, max_offset_ratio) * r_ball;
    let b = strike.offset_y.clamp(-max_offset_ratio, max_offset_ratio) * r_ball;

    let phi = strike.azimuth.0;
    let theta = strike.elevation.0;

    // Cue stick direction unit vector
    let cos_t = theta.cos();
    let sin_t = theta.sin();
    let cos_p = phi.cos();
    let sin_p = phi.sin();

    // Cue strike impulse magnitude: J ~ 2 * m * M / (m + M) * V_stroke
    let v_stroke = strike.speed.0;
    let j_impulse = (2.0 * m_cue * m_ball / (m_cue + m_ball)) * v_stroke;

    // Aim unit vector in X-Y plane
    let aim_dir = Vec3::new(cos_p * cos_t, sin_p * cos_t, -sin_t);

    // Compute squirt (deflection angle alpha) due to off-center tip hit
    let squirt_angle = -0.5 * (a / r_ball) * (m_cue / (m_cue + m_ball));
    let cos_s = squirt_angle.cos();
    let sin_s = squirt_angle.sin();

    let actual_dir = Vec3::new(
        aim_dir.x * cos_s - aim_dir.y * sin_s,
        aim_dir.x * sin_s + aim_dir.y * cos_s,
        aim_dir.z,
    );

    // Initial linear velocity V_0 = J / M * actual_dir
    let v0 = actual_dir * (j_impulse / m_ball);

    // Angular momentum impulse L = r_contact x J_vec
    // Contact point displacement r_c relative to center
    let r_contact = Vec3::new(
        -a * sin_p + b * sin_t * cos_p,
        a * cos_p + b * sin_t * sin_p,
        b * cos_t,
    );

    let impulse_vec = actual_dir * j_impulse;
    let torque_impulse = r_contact.cross(impulse_vec);

    // Moment of inertia for solid sphere I = 2/5 * M * R^2
    let i_inertia = 0.4 * m_ball * r_ball * r_ball;
    let w0 = torque_impulse / i_inertia;

    cue_ball.velocity = Vec3::new(v0.x, v0.y, 0.0);
    cue_ball.angular_velocity = w0;
    cue_ball.state = MotionState::Sliding;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cue_stick_defaults() {
        let cue = CueStick::default();
        assert_eq!(cue.length.0, 1.473);
        assert_eq!(cue.mass.0, 0.5388);
        assert_eq!(cue.tip_radius.0, 0.00625);
        assert_eq!(cue.max_tip_offset.0, 0.010);
        assert_eq!(cue.break_speed.0, 10.0);
    }
}
