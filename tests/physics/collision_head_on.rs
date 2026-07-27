use cueforge_common::{Seconds, Vec3};
use cueforge_physics::{Ball, BallId, Event, MotionState, World, WorldConfig};

#[test]
fn test_head_on_collision_momentum_transfer() {
    let mut world = World::new(WorldConfig::default());

    // Cue ball moving at 2.0 m/s toward target ball along Y axis
    let mut cue_ball = Ball::new(BallId(0), Vec3::new(0.0, 0.0, 0.0));
    cue_ball.velocity = Vec3::new(0.0, 2.0, 0.0);
    cue_ball.state = MotionState::Sliding;

    let target_ball = Ball::new(BallId(1), Vec3::new(0.0, 0.1, 0.0));

    world.add_ball(cue_ball);
    world.add_ball(target_ball);

    let mut collision_occurred = false;
    let dt = Seconds(0.001);

    for _ in 0..500 {
        let events = world.step(dt);
        for event in events {
            if let Event::BallBallCollision { ball1, ball2, .. } = event {
                assert!(
                    (ball1 == BallId(0) && ball2 == BallId(1))
                        || (ball1 == BallId(1) && ball2 == BallId(0))
                );
                collision_occurred = true;
            }
        }
        if collision_occurred {
            break;
        }
    }

    assert!(collision_occurred, "Head-on collision must occur");

    let b0 = world.get_ball(BallId(0)).unwrap();
    let b1 = world.get_ball(BallId(1)).unwrap();

    // In near-elastic equal mass head-on collision, target ball receives almost all velocity forward
    assert!(
        b1.velocity.y > 1.5,
        "Target ball must gain majority of forward momentum"
    );
    assert!(
        b0.velocity.y < b1.velocity.y,
        "Cue ball must slow down after impact"
    );
}
