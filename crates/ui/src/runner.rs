//! Interactive simulation scenario runner.

use cueforge_ai::find_best_shot;
use cueforge_common::{Seconds, Vec3};
use cueforge_cue::{strike_cue_ball, CueStick, CueStrike};
use cueforge_physics::{Ball, BallId, World, WorldConfig};
use cueforge_renderer::render_ascii_table;
use cueforge_rules::{GameVariant, RuleEngine};
use cueforge_statistics::StatisticsTracker;
use cueforge_table::TableSpec;

pub fn run_simulation_demo() {
    println!("==================================================");
    println!("         CueForge - Physics Engine Core           ");
    println!("==================================================\n");

    let table = TableSpec::new_9ft_pool();
    let cue_stick = CueStick::default();
    let mut world = World::new(WorldConfig::default());
    let mut stats = StatisticsTracker::new();

    // Setup 9-Ball rack scenario
    let cue_ball = Ball::new(BallId(0), Vec3::new(0.0, -0.6, 0.0));
    let ball_1 = Ball::new(BallId(1), Vec3::new(0.0, 0.5, 0.0));
    let ball_2 = Ball::new(BallId(2), Vec3::new(-0.03, 0.55, 0.0));
    let ball_3 = Ball::new(BallId(3), Vec3::new(0.03, 0.55, 0.0));

    world.add_ball(cue_ball);
    world.add_ball(ball_1);
    world.add_ball(ball_2);
    world.add_ball(ball_3);

    println!("--- Initial Table State ---");
    println!("{}", render_ascii_table(&world, &table, 40, 20));

    let rule_engine = RuleEngine::new(GameVariant::NineBall, BallId(0));

    // Consult AI module for optimal shot recommendation
    if let Some(recommended) = find_best_shot(&world, &table, 0) {
        println!("AI Shot Recommendation:");
        println!("  Target Ball: {}", recommended.target_ball_id);
        println!("  Pocket Index: {}", recommended.pocket_index);
        println!("  Aim Azimuth: {:.3} rad", recommended.aim_azimuth_rad);
        println!(
            "  Cut Angle: {:.3} rad ({:.1}°)",
            recommended.cut_angle_rad,
            recommended.cut_angle_rad.to_degrees()
        );
        println!();

        // Apply cue strike
        if let Some(cue_ball_ref) = world.get_ball_mut(BallId(0)) {
            strike_cue_ball(&cue_stick, &recommended.recommended_strike, cue_ball_ref);
        }
    } else {
        // Fallback straight shot
        let strike = CueStrike::center_shot(2.5, std::f64::consts::FRAC_PI_2);
        if let Some(cue_ball_ref) = world.get_ball_mut(BallId(0)) {
            strike_cue_ball(&cue_stick, &strike, cue_ball_ref);
        }
    }

    println!("Simulating shot physics forward...");
    let mut total_events = Vec::new();
    let dt = Seconds(0.001);

    for _step in 0..3000 {
        if !world.is_active() {
            break;
        }
        let events = world.step(dt);
        total_events.extend(events);
    }

    println!("\n--- Post-Shot Table State ---");
    println!("{}", render_ascii_table(&world, &table, 40, 20));

    // Rule Evaluation
    let result = rule_engine.evaluate_shot(&total_events);
    println!("--- Shot Evaluation Results ---");
    println!("  Valid Shot: {}", result.is_valid);
    if let Some(first) = result.first_ball_struck {
        println!("  First Struck Ball: {}", first.0);
    } else {
        println!("  First Struck Ball: None");
    }
    println!(
        "  Pocketed Balls: {:?}",
        result
            .pocketed_balls
            .iter()
            .map(|b| b.0)
            .collect::<Vec<_>>()
    );
    println!("  Fouls: {:?}", result.fouls);

    // Track Statistics
    stats.record_shot(&result, 1.2);
    println!("\n--- Match Statistics ---");
    println!("  Total Shots: {}", stats.total_shots);
    println!("  Successful Pots: {}", stats.successful_pots);
    println!("  Pot Accuracy: {:.1}%", stats.pot_accuracy() * 100.0);
    println!("==================================================");
}
