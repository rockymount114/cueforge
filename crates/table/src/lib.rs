//! `cueforge-table`
//!
//! Table geometry, cloth/cushion parameters, pocket geometry, and rail contact resolution.

use cueforge_common::{Meters, Vec2};
use cueforge_physics::Ball;

/// Pocket location and radius on the table bed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pocket {
    pub index: usize,
    pub position: Vec2,
    pub radius: Meters,
}

/// Rail segment boundary.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RailSegment {
    pub index: usize,
    pub start: Vec2,
    pub end: Vec2,
    pub normal: Vec2,
}

/// Complete specifications of a pool/billiards table.
#[derive(Debug, Clone, PartialEq)]
pub struct TableSpec {
    /// Width along X axis (short dimension), default 1.27m (50 in) for 9ft table.
    pub width: Meters,
    /// Length along Y axis (long dimension), default 2.54m (100 in) for 9ft table.
    pub length: Meters,
    /// Restitution coefficient of cushion rail bounces.
    pub rail_restitution: f64,
    /// Cloth sliding friction coefficient mu_s.
    pub cloth_sliding_friction: f64,
    /// Cloth rolling friction coefficient mu_r.
    pub cloth_rolling_friction: f64,
    /// 6 Pocket locations.
    pub pockets: Vec<Pocket>,
    /// 4 Rail boundaries.
    pub rails: Vec<RailSegment>,
}

impl Default for TableSpec {
    fn default() -> Self {
        Self::new_9ft_pool()
    }
}

impl TableSpec {
    /// Create standard 9-foot regulation pool table.
    pub fn new_9ft_pool() -> Self {
        let half_w = 0.635; // 1.27m total width / 2
        let half_l = 1.270; // 2.54m total length / 2
        let corner_r = Meters(0.060);
        let side_r = Meters(0.065);

        let pockets = vec![
            Pocket {
                index: 0,
                position: Vec2::new(-half_w, -half_l),
                radius: corner_r,
            },
            Pocket {
                index: 1,
                position: Vec2::new(half_w, -half_l),
                radius: corner_r,
            },
            Pocket {
                index: 2,
                position: Vec2::new(-half_w, 0.0),
                radius: side_r,
            },
            Pocket {
                index: 3,
                position: Vec2::new(half_w, 0.0),
                radius: side_r,
            },
            Pocket {
                index: 4,
                position: Vec2::new(-half_w, half_l),
                radius: corner_r,
            },
            Pocket {
                index: 5,
                position: Vec2::new(half_w, half_l),
                radius: corner_r,
            },
        ];

        let rails = vec![
            // Left rail (-x)
            RailSegment {
                index: 0,
                start: Vec2::new(-half_w, -half_l),
                end: Vec2::new(-half_w, half_l),
                normal: Vec2::new(1.0, 0.0),
            },
            // Right rail (+x)
            RailSegment {
                index: 1,
                start: Vec2::new(half_w, -half_l),
                end: Vec2::new(half_w, half_l),
                normal: Vec2::new(-1.0, 0.0),
            },
            // Head rail (-y)
            RailSegment {
                index: 2,
                start: Vec2::new(-half_w, -half_l),
                end: Vec2::new(half_w, -half_l),
                normal: Vec2::new(0.0, 1.0),
            },
            // Foot rail (+y)
            RailSegment {
                index: 3,
                start: Vec2::new(-half_w, half_l),
                end: Vec2::new(half_w, half_l),
                normal: Vec2::new(0.0, -1.0),
            },
        ];

        Self {
            width: Meters(half_w * 2.0),
            length: Meters(half_l * 2.0),
            rail_restitution: 0.85,
            cloth_sliding_friction: 0.20,
            cloth_rolling_friction: 0.015,
            pockets,
            rails,
        }
    }

    /// Check if a ball is captured by any pocket.
    pub fn check_pocket_capture(&self, ball: &Ball) -> Option<usize> {
        let ball_pos = ball.position.to_vec2();
        for pocket in &self.pockets {
            if ball_pos.distance(pocket.position) <= pocket.radius.0 {
                return Some(pocket.index);
            }
        }
        None
    }

    /// Resolve ball-rail boundary collision if ball hits a cushion.
    pub fn resolve_rail_collisions(&self, ball: &mut Ball) -> Option<usize> {
        let r = ball.radius.0;
        let pos = ball.position.to_vec2();

        for rail in &self.rails {
            // Distance from rail line
            let dist = (pos - rail.start).dot(rail.normal);
            if dist < r {
                let v_planar = Vec2::new(ball.velocity.x, ball.velocity.y);
                let v_normal = v_planar.dot(rail.normal);

                if v_normal < 0.0 {
                    // Reflect velocity vector across rail normal with restitution
                    let v_normal_post = -self.rail_restitution * v_normal;
                    let v_tangent = v_planar - rail.normal * v_normal;
                    let v_planar_post = v_tangent + rail.normal * v_normal_post;

                    ball.velocity.x = v_planar_post.x;
                    ball.velocity.y = v_planar_post.y;

                    // Push position back inside boundary
                    let overlap = r - dist;
                    ball.position.x += rail.normal.x * overlap;
                    ball.position.y += rail.normal.y * overlap;

                    return Some(rail.index);
                }
            }
        }
        None
    }
}
