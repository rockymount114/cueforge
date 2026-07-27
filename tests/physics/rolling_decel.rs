use cueforge_common::{Seconds, Vec3};
use cueforge_physics::{Ball, BallId, MotionState, World, WorldConfig};

#[test]
fn test_rolling_deceleration_to_stationary() {
    let mut world = World::new(WorldConfig::default());

    let mut ball = Ball::new(BallId(0), Vec3::new(0.0, 0.0, 0.0));
    ball.velocity = Vec3::new(0.0, 0.5, 0.0);
    // Set no-slip angular velocity w_x = -v_y / r
    ball.angular_velocity = Vec3::new(-0.5 / ball.radius.0, 0.0, 0.0);
    ball.state = MotionState::Rolling;

    world.add_ball(ball);

    let dt = Seconds(0.001);
    let mut reached_stationary = false;

    for _ in 0..5000 {
        world.step(dt);
        let b = world.get_ball(BallId(0)).unwrap();
        if b.state == MotionState::Stationary {
            reached_stationary = true;
            break;
        }
    }

    assert!(
        reached_stationary,
        "Ball must come to rest under rolling friction"
    );
    let b = world.get_ball(BallId(0)).unwrap();
    assert_eq!(b.velocity, Vec3::ZERO);
}
