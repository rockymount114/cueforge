use cueforge_common::{Seconds, Vec3};
use cueforge_cue::{strike_cue_ball, CueStick, CueStrike};
use cueforge_physics::{Ball, BallId, World, WorldConfig};

#[test]
fn test_simulation_bit_identical_determinism() {
    let run_simulation = || -> (Vec3, Vec3) {
        let table_config = WorldConfig::default();
        let mut world = World::new(table_config);

        let mut cue_ball = Ball::new(BallId(0), Vec3::new(-0.2, -0.5, 0.0));
        let cue_stick = CueStick::default();
        let strike = CueStrike::center_shot(3.2, 0.785398);

        strike_cue_ball(&cue_stick, &strike, &mut cue_ball);
        world.add_ball(cue_ball);

        let target = Ball::new(BallId(1), Vec3::new(0.0, 0.0, 0.0));
        world.add_ball(target);

        let dt = Seconds(0.001);
        for _ in 0..1000 {
            world.step(dt);
        }

        let b0 = world.get_ball(BallId(0)).unwrap();
        let b1 = world.get_ball(BallId(1)).unwrap();

        (b0.position, b1.position)
    };

    let result1 = run_simulation();
    let result2 = run_simulation();

    // Exact bit-identical check
    assert_eq!(result1.0.x.to_bits(), result2.0.x.to_bits());
    assert_eq!(result1.0.y.to_bits(), result2.0.y.to_bits());
    assert_eq!(result1.1.x.to_bits(), result2.1.x.to_bits());
    assert_eq!(result1.1.y.to_bits(), result2.1.y.to_bits());
}
