use cueforge_common::Vec3;
use cueforge_physics::{Ball, BallId};
use cueforge_table::TableSpec;

#[test]
fn test_pocket_capture_detection() {
    let table = TableSpec::new_9ft_pool();
    let corner_pocket = table.pockets[0];

    // Ball inside corner pocket boundary
    let ball_in_pocket = Ball::new(
        BallId(1),
        Vec3::new(corner_pocket.position.x, corner_pocket.position.y, 0.0),
    );
    assert_eq!(table.check_pocket_capture(&ball_in_pocket), Some(0));

    // Ball in center of table
    let ball_center = Ball::new(BallId(0), Vec3::ZERO);
    assert_eq!(table.check_pocket_capture(&ball_center), None);
}
