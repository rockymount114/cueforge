//! `cueforge-spin`
//!
//! Coupled spin/friction integration (english, throw, squirt, swerve, massé).

use cueforge_common::Vec3;
use cueforge_physics::Ball;

/// Compute swerve acceleration on a spinning, sliding ball.
pub fn compute_swerve_acceleration(ball: &Ball, mu_s: f64, gravity: f64) -> Vec3 {
    let vc = ball.cloth_contact_velocity();
    let vc_planar = Vec3::new(vc.x, vc.y, 0.0);
    let speed = vc_planar.length();

    if speed > 1e-4 {
        let dir = vc_planar / speed;
        -dir * (mu_s * gravity)
    } else {
        Vec3::ZERO
    }
}

/// Apply continuous spin update over a time step dt.
pub fn update_spin_and_velocity(ball: &mut Ball, dt: f64, mu_s: f64, mu_r: f64, gravity: f64) {
    let r = ball.radius.0;
    let vc = ball.cloth_contact_velocity();
    let vc_planar = Vec3::new(vc.x, vc.y, 0.0);
    let speed_c = vc_planar.length();

    if speed_c > 1e-4 {
        let dir_c = vc_planar / speed_c;
        let accel = -dir_c * (mu_s * gravity);
        let z_hat = Vec3::new(0.0, 0.0, 1.0);
        let alpha = z_hat.cross(dir_c) * (2.5 * mu_s * gravity / r);

        ball.position += ball.velocity * dt + accel * (0.5 * dt * dt);
        ball.velocity += accel * dt;
        ball.angular_velocity += alpha * dt;
    } else {
        let speed_v = Vec3::new(ball.velocity.x, ball.velocity.y, 0.0).length();
        if speed_v > 1e-4 {
            let dir_v = Vec3::new(ball.velocity.x, ball.velocity.y, 0.0) / speed_v;
            let accel = -dir_v * (mu_r * gravity);

            ball.position += ball.velocity * dt + accel * (0.5 * dt * dt);
            ball.velocity += accel * dt;
            ball.angular_velocity.x = -ball.velocity.y / r;
            ball.angular_velocity.y = ball.velocity.x / r;
        } else {
            ball.velocity = Vec3::ZERO;
        }
    }
}
