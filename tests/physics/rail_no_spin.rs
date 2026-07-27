use cueforge_common::Vec3;
use cueforge_physics::{Ball, BallId};
use cueforge_table::TableSpec;

#[test]
fn test_rail_rebound_restitution() {
    let table = TableSpec::new_9ft_pool();
    let mut ball = Ball::new(BallId(0), Vec3::new(-0.63, 0.0, 0.0));
    ball.velocity = Vec3::new(-2.0, 0.0, 0.0);

    let rail_hit = table.resolve_rail_collisions(&mut ball);
    assert!(
        rail_hit.is_some(),
        "Ball near boundary moving outwards must collide with rail"
    );
    assert!(
        ball.velocity.x > 0.0,
        "Ball must bounce off rail back towards center"
    );
}
