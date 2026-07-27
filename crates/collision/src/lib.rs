//! `cueforge-collision`
//!
//! Ball-ball and ball-rail collision detection & impulse resolution.

use cueforge_common::Vec3;
use cueforge_physics::Ball;

/// Result of a continuous collision query between two balls.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CollisionContact {
    pub time_of_impact: f64,
    pub normal: Vec3,
    pub point: Vec3,
}

/// Compute exact time of impact (TOI) within tick interval [0, dt] between two moving balls.
pub fn ball_ball_toi(ball1: &Ball, ball2: &Ball, dt: f64) -> Option<CollisionContact> {
    let r1 = ball1.radius.0;
    let r2 = ball2.radius.0;
    let min_dist = r1 + r2;

    let p0 = ball2.position - ball1.position;
    let v = ball2.velocity - ball1.velocity;

    let a = v.length_squared();
    let b = 2.0 * p0.dot(v);
    let c = p0.length_squared() - min_dist * min_dist;

    if c <= 0.0 {
        let dist = p0.length();
        let normal = if dist > 1e-9 {
            p0 / dist
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        return Some(CollisionContact {
            time_of_impact: 0.0,
            normal,
            point: ball1.position + normal * r1,
        });
    }

    if a <= 1e-12 {
        return None;
    }

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }

    let t = (-b - discriminant.sqrt()) / (2.0 * a);
    if (0.0..=dt).contains(&t) {
        let p_at_t = p0 + v * t;
        let normal = p_at_t.normalize();
        Some(CollisionContact {
            time_of_impact: t,
            normal,
            point: (ball1.position + ball1.velocity * t) + normal * r1,
        })
    } else {
        None
    }
}

/// Resolve impulse between two colliding balls.
pub fn resolve_ball_ball_impulse(
    ball1: &mut Ball,
    ball2: &mut Ball,
    normal: Vec3,
    restitution: f64,
    friction_coef: f64,
) {
    let m1 = ball1.mass.0;
    let m2 = ball2.mass.0;

    let rel_vel = ball2.velocity - ball1.velocity;
    let v_normal = rel_vel.dot(normal);

    if v_normal >= 0.0 {
        return;
    }

    // Normal impulse scalar
    let j_normal = -(1.0 + restitution) * v_normal / (1.0 / m1 + 1.0 / m2);
    let impulse_normal = normal * j_normal;

    ball1.velocity -= impulse_normal / m1;
    ball2.velocity += impulse_normal / m2;

    // Tangential (throw) friction impulse
    let tangent_vel = rel_vel - normal * v_normal;
    let tangent_speed = tangent_vel.length();

    if tangent_speed > 1e-6 {
        let tangent_dir = tangent_vel / tangent_speed;
        let max_friction = friction_coef * j_normal;
        let j_tangent = max_friction.min(tangent_speed / (1.0 / m1 + 1.0 / m2));

        let impulse_tangent = tangent_dir * j_tangent;
        ball1.velocity -= impulse_tangent / m1;
        ball2.velocity += impulse_tangent / m2;
    }
}
