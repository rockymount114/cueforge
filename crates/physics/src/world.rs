//! Deterministic physical world container and fixed-timestep integrator.

use crate::ball::{Ball, BallId, MotionState};
use crate::events::Event;
use cueforge_common::{Seconds, Vec3};

/// Simulation configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct WorldConfig {
    pub sub_step_dt: Seconds,
    pub sliding_friction_coef: f64,
    pub rolling_friction_coef: f64,
    pub spinning_friction_coef: f64,
    pub restitution_ball_ball: f64,
    pub ball_friction_coef: f64,
    pub gravity: f64,
    pub stationary_velocity_threshold: f64,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            sub_step_dt: Seconds(0.001),
            sliding_friction_coef: 0.20,
            rolling_friction_coef: 0.015,
            spinning_friction_coef: 0.005,
            restitution_ball_ball: 0.95,
            ball_friction_coef: 0.05,
            gravity: 9.80665,
            stationary_velocity_threshold: 0.01,
        }
    }
}

/// Physical world state holding active balls and simulation clock.
#[derive(Debug, Clone)]
pub struct World {
    pub balls: Vec<Ball>,
    pub time: Seconds,
    pub config: WorldConfig,
}

impl World {
    pub fn new(config: WorldConfig) -> Self {
        Self {
            balls: Vec::new(),
            time: Seconds(0.0),
            config,
        }
    }

    pub fn add_ball(&mut self, ball: Ball) {
        self.balls.push(ball);
    }

    pub fn get_ball(&self, id: BallId) -> Option<&Ball> {
        self.balls.iter().find(|b| b.id == id)
    }

    pub fn get_ball_mut(&mut self, id: BallId) -> Option<&mut Ball> {
        self.balls.iter_mut().find(|b| b.id == id)
    }

    /// Check if any ball is currently moving.
    pub fn is_active(&self) -> bool {
        self.balls.iter().any(|b| b.is_moving())
    }

    /// Step the simulation forward by `dt` seconds deterministically.
    pub fn step(&mut self, dt: Seconds) -> Vec<Event> {
        let mut events = Vec::new();
        let target_time = self.time.0 + dt.0;
        let sub_dt = self.config.sub_step_dt.0;

        while self.time.0 < target_time {
            let step_size = (target_time - self.time.0).min(sub_dt);
            let step_events = self.internal_sub_step(step_size);
            events.extend(step_events);
            self.time.0 += step_size;
        }

        events
    }

    fn internal_sub_step(&mut self, dt: f64) -> Vec<Event> {
        let mut events = Vec::new();
        let g = self.config.gravity;
        let mu_s = self.config.sliding_friction_coef;
        let mu_r = self.config.rolling_friction_coef;
        let mu_sp = self.config.spinning_friction_coef;
        let v_stop = self.config.stationary_velocity_threshold;

        // 1. Resolve pairwise ball-ball collisions in canonical deterministic order
        let mut candidate_collisions = Vec::new();
        let n = self.balls.len();

        for i in 0..n {
            for j in (i + 1)..n {
                if !self.balls[i].is_active() || !self.balls[j].is_active() {
                    continue;
                }

                let b1 = &self.balls[i];
                let b2 = &self.balls[j];

                let rel_pos = b2.position - b1.position;
                let dist = rel_pos.length();
                let min_dist = b1.radius.0 + b2.radius.0;

                let rel_vel = b2.velocity - b1.velocity;
                let normal = if dist > 1e-9 {
                    rel_pos / dist
                } else {
                    Vec3::new(1.0, 0.0, 0.0)
                };

                let v_rel_normal = rel_vel.dot(normal);

                // Collision check if moving toward each other and overlapping / touching
                if dist <= min_dist && v_rel_normal < 0.0 {
                    let id1 = b1.id.min(b2.id);
                    let id2 = b1.id.max(b2.id);
                    candidate_collisions.push((id1, id2, i, j, normal, -v_rel_normal));
                }
            }
        }

        // Canonical deterministic sort: ascending ID pairs
        candidate_collisions.sort_by_key(|a| (a.0, a.1));

        for (_, _, i, j, normal, rel_speed) in candidate_collisions {
            let e = self.config.restitution_ball_ball;
            let m1 = self.balls[i].mass.0;
            let m2 = self.balls[j].mass.0;

            let j_impulse = -(1.0 + e) * (-rel_speed) / (1.0 / m1 + 1.0 / m2);

            let impulse_vec = normal * j_impulse;

            self.balls[i].velocity -= impulse_vec / m1;
            self.balls[j].velocity += impulse_vec / m2;

            if self.balls[i].state == MotionState::Stationary {
                self.balls[i].state = MotionState::Sliding;
            }
            if self.balls[j].state == MotionState::Stationary {
                self.balls[j].state = MotionState::Sliding;
            }

            events.push(Event::BallBallCollision {
                ball1: self.balls[i].id,
                ball2: self.balls[j].id,
                time: self.time,
                relative_speed: rel_speed,
            });
        }

        // 2. Integrate continuous ball motion under friction
        for ball in self.balls.iter_mut() {
            if !ball.is_active() || ball.state == MotionState::Stationary {
                continue;
            }

            let r = ball.radius.0;
            let vc = ball.cloth_contact_velocity();
            let vc_planar = Vec3::new(vc.x, vc.y, 0.0);
            let speed_c = vc_planar.length();

            if speed_c > 1e-4 {
                // Sliding mode
                if ball.state != MotionState::Sliding {
                    events.push(Event::StateTransition {
                        ball: ball.id,
                        from: ball.state,
                        to: MotionState::Sliding,
                        time: self.time,
                    });
                    ball.state = MotionState::Sliding;
                }

                let dir_c = vc_planar / speed_c;

                // Friction force F_s = -mu_s * m * g * dir_c
                let accel = -dir_c * (mu_s * g);

                // Angular acceleration alpha = 2.5 / r * (r_c x (-mu_s * g * dir_c))
                let z_hat = Vec3::new(0.0, 0.0, 1.0);
                let alpha = z_hat.cross(dir_c) * (2.5 * mu_s * g / r);

                ball.position += ball.velocity * dt + accel * (0.5 * dt * dt);
                ball.velocity += accel * dt;
                ball.angular_velocity += alpha * dt;
            } else {
                // Rolling mode (vc ~ 0)
                let speed_v = Vec3::new(ball.velocity.x, ball.velocity.y, 0.0).length();

                if speed_v > v_stop {
                    if ball.state != MotionState::Rolling {
                        events.push(Event::StateTransition {
                            ball: ball.id,
                            from: ball.state,
                            to: MotionState::Rolling,
                            time: self.time,
                        });
                        ball.state = MotionState::Rolling;
                    }

                    let dir_v = Vec3::new(ball.velocity.x, ball.velocity.y, 0.0) / speed_v;

                    // Rolling resistance deceleration
                    let accel = -dir_v * (mu_r * g);

                    ball.position += ball.velocity * dt + accel * (0.5 * dt * dt);
                    ball.velocity += accel * dt;

                    // Maintain no-slip angular velocity matching linear velocity
                    // v = w x (0, 0, -r) => w_x = -v_y / r, w_y = v_x / r
                    ball.angular_velocity.x = -ball.velocity.y / r;
                    ball.angular_velocity.y = ball.velocity.x / r;
                } else {
                    // Transition to Stationary
                    if ball.state != MotionState::Stationary {
                        events.push(Event::StateTransition {
                            ball: ball.id,
                            from: ball.state,
                            to: MotionState::Stationary,
                            time: self.time,
                        });
                        ball.state = MotionState::Stationary;
                    }
                    ball.velocity = Vec3::ZERO;
                }
            }

            // Spinning friction decay (z-axis english)
            let wz = ball.angular_velocity.z;
            if wz.abs() > 1e-4 {
                let spin_decay = (5.0 * mu_sp * g / (2.0 * r)) * dt;
                if wz.abs() <= spin_decay {
                    ball.angular_velocity.z = 0.0;
                } else {
                    ball.angular_velocity.z -= wz.signum() * spin_decay;
                }
            } else {
                ball.angular_velocity.z = 0.0;
            }

            // Keep balls strictly on cloth level z = 0
            ball.position.z = 0.0;
        }

        events
    }
}
