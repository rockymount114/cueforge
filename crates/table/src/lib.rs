//! `cueforge-table`
//!
//! Table geometry, cloth/cushion parameters, pocket geometry, and rail contact resolution.

use cueforge_common::{Meters, Vec2};
use cueforge_physics::Ball;

/// Cloth weave type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClothWeave {
    Worsted,
    Napped,
}

/// Tournament or venue cloth preset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClothPreset {
    Simonis860,
    Simonis760,
    LeagueCloth,
    OldCloth,
}

/// 9-Ball table cloth specifications and environment-adjusted friction parameters.
#[derive(Debug, Clone, PartialEq)]
pub struct ClothSpec {
    pub name: String,
    pub material: String,
    pub weave: ClothWeave,
    pub has_nap: bool,
    pub base_rolling_friction: f64,
    pub base_sliding_friction: f64,
    pub linear_deceleration: f64,
    pub spin_decay: f64,
    pub restitution_loss: f64,
    pub speed_multiplier: f64,
    pub humidity_percent: f64,
    pub temperature_celsius: f64,
    pub hex_color: String,
}

impl Default for ClothSpec {
    fn default() -> Self {
        Self::simonis_860()
    }
}

impl ClothSpec {
    /// Official Simonis 860 Tournament Blue worsted cloth standard (WPA 9-ball tournament standard).
    pub fn simonis_860() -> Self {
        Self {
            name: "Simonis 860".into(),
            material: "90% Wool / 10% Nylon".into(),
            weave: ClothWeave::Worsted,
            has_nap: false,
            base_rolling_friction: 0.015,
            base_sliding_friction: 0.20,
            linear_deceleration: 0.22,
            spin_decay: 2.0,
            restitution_loss: 0.03,
            speed_multiplier: 1.0,
            humidity_percent: 45.0,
            temperature_celsius: 22.0,
            hex_color: "#0055a5".into(), // Tournament Blue
        }
    }

    /// Fast Simonis 760 cloth preset.
    pub fn simonis_760() -> Self {
        Self {
            name: "Simonis 760".into(),
            material: "70% Wool / 30% Nylon".into(),
            weave: ClothWeave::Worsted,
            has_nap: false,
            base_rolling_friction: 0.012,
            base_sliding_friction: 0.18,
            linear_deceleration: 0.18,
            spin_decay: 1.8,
            restitution_loss: 0.02,
            speed_multiplier: 1.15,
            humidity_percent: 45.0,
            temperature_celsius: 22.0,
            hex_color: "#0f52ba".into(),
        }
    }

    /// Standard league table cloth preset.
    pub fn league_cloth() -> Self {
        Self {
            name: "Standard League Cloth".into(),
            material: "75% Wool / 25% Nylon".into(),
            weave: ClothWeave::Napped,
            has_nap: true,
            base_rolling_friction: 0.022,
            base_sliding_friction: 0.23,
            linear_deceleration: 0.26,
            spin_decay: 2.4,
            restitution_loss: 0.04,
            speed_multiplier: 0.88,
            humidity_percent: 50.0,
            temperature_celsius: 20.0,
            hex_color: "#0d5c3a".into(), // Classic Green
        }
    }

    /// Old bar table cloth preset.
    pub fn old_cloth() -> Self {
        Self {
            name: "Old Bar Table Cloth".into(),
            material: "Heavy Napped Wool".into(),
            weave: ClothWeave::Napped,
            has_nap: true,
            base_rolling_friction: 0.030,
            base_sliding_friction: 0.28,
            linear_deceleration: 0.32,
            spin_decay: 3.0,
            restitution_loss: 0.06,
            speed_multiplier: 0.70,
            humidity_percent: 60.0,
            temperature_celsius: 18.0,
            hex_color: "#155293".into(),
        }
    }

    /// Compute effective rolling friction taking humidity, temperature, and speed multiplier into account.
    pub fn effective_rolling_friction(&self) -> f64 {
        let hum_factor = 1.0 + 0.005 * (self.humidity_percent - 45.0);
        let temp_factor = 1.0 - 0.003 * (self.temperature_celsius - 22.0);
        (self.base_rolling_friction * hum_factor * temp_factor) / self.speed_multiplier
    }

    /// Compute effective sliding friction taking environment into account.
    pub fn effective_sliding_friction(&self) -> f64 {
        let hum_factor = 1.0 + 0.003 * (self.humidity_percent - 45.0);
        let temp_factor = 1.0 - 0.002 * (self.temperature_celsius - 22.0);
        (self.base_sliding_friction * hum_factor * temp_factor) / self.speed_multiplier
    }

    /// Compute effective spin decay (rad/s²).
    pub fn effective_spin_decay(&self) -> f64 {
        let hum_factor = 1.0 + 0.004 * (self.humidity_percent - 45.0);
        self.spin_decay * hum_factor
    }
}

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

/// Complete specifications of a pool/billiards table based on WPA standards.
#[derive(Debug, Clone, PartialEq)]
pub struct TableSpec {
    /// Width along X axis (short dimension), 1.270m (50 in) for 9ft table.
    pub width: Meters,
    /// Length along Y axis (long dimension), 2.540m (100 in) for 9ft table.
    pub length: Meters,
    /// Height of cushion rail above bed, default 0.037m (37 mm).
    pub rail_height: Meters,
    /// Restitution coefficient of cushion rail bounces (K55 rubber = 0.90).
    pub rail_restitution: f64,
    /// Friction coefficient of cushion rail cloth (0.12).
    pub rail_friction: f64,
    /// Pocket shelf depth (0.035m = 35 mm).
    pub shelf_depth: Meters,
    /// Cloth specifications and friction model.
    pub cloth: ClothSpec,
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
    /// Create standard 9-foot regulation pool table equipped with K55 cushions and Simonis 860 Tournament Blue cloth.
    pub fn new_9ft_pool() -> Self {
        let half_w = 0.635; // 1.270m total width / 2
        let half_l = 1.270; // 2.540m total length / 2
                            // Corner pocket opening = 114 mm (radius = 0.057 m) per table-specification.md
        let corner_r = Meters(0.057);
        // Side pocket opening = 127 mm (radius = 0.0635 m) per table-specification.md
        let side_r = Meters(0.0635);

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
            rail_height: Meters(0.037),
            rail_restitution: 0.90,
            rail_friction: 0.12,
            shelf_depth: Meters(0.035),
            cloth: ClothSpec::simonis_860(),
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
            let dist = (pos - rail.start).dot(rail.normal);
            if dist < r {
                let v_planar = Vec2::new(ball.velocity.x, ball.velocity.y);
                let v_normal = v_planar.dot(rail.normal);

                if v_normal < 0.0 {
                    let v_normal_post = -self.rail_restitution * v_normal;
                    let v_tangent = v_planar - rail.normal * v_normal;
                    let v_planar_post = v_tangent + rail.normal * v_normal_post;

                    ball.velocity.x = v_planar_post.x;
                    ball.velocity.y = v_planar_post.y;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simonis_860_cloth_defaults() {
        let cloth = ClothSpec::simonis_860();
        assert_eq!(cloth.name, "Simonis 860");
        assert_eq!(cloth.weave, ClothWeave::Worsted);
        assert!(!cloth.has_nap);
        assert_eq!(cloth.base_rolling_friction, 0.015);
        assert_eq!(cloth.base_sliding_friction, 0.20);
        assert_eq!(cloth.effective_rolling_friction(), 0.015);
        assert_eq!(cloth.effective_sliding_friction(), 0.20);
    }

    #[test]
    fn test_table_spec_parameters() {
        let table = TableSpec::new_9ft_pool();
        assert_eq!(table.width.0, 1.270);
        assert_eq!(table.length.0, 2.540);
        assert_eq!(table.rail_restitution, 0.90);
        assert_eq!(table.rail_friction, 0.12);
        assert_eq!(table.rail_height.0, 0.037);
        assert_eq!(table.shelf_depth.0, 0.035);
        assert_eq!(table.pockets[0].radius.0, 0.057); // Corner pocket
        assert_eq!(table.pockets[2].radius.0, 0.0635); // Side pocket
    }
}
