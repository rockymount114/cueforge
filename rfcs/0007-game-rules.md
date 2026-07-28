# RFC 0007: Game Rules Engine

- **Status**: Accepted
- **Author(s)**: CueForge Core Team
- **Created**: 2026-07-28
- **Related crates**: `crates/rules`

## Summary

This RFC specifies the game state machines, foul evaluation logic, turn switching, scoring systems, and rack resets for official cue sport variants: **8-Ball**, **9-Ball**, and **Straight Pool (14.1 Continuous)**.

## Motivation

While `crates/rules` currently provides basic shot event filtering (scratch detection, first ball struck), formal competitive gameplay requires deterministic game state management:
- Group assignment (Solids vs. Stripes in 8-Ball).
- Target ball ordering (lowest-numbered ball target in 9-Ball).
- Rack reset logic (continuous 14-ball rack resets in 14.1).
- Consecutive foul counters (e.g. 3-foul loss in 9-Ball).

## Guide-level explanation

The `RuleEngine` maintains `GameState` across consecutive shots:
- `EightBallState`: Open table, assigned player groups (`Solids` or `Stripes`), 8-ball call-shot target.
- `NineBallState`: Lowest-numbered active ball requirement, push-out state, 3-foul penalty counter per player.
- `StraightPoolState`: Cumulative call-shot point tracker, 14-ball rack reset state (leaving cue ball and 15th object ball in position).

Call-shot declarations and physical shot event streams (`ShotResult`) are evaluated at the end of each simulation turn to produce `TurnOutcome`:
- `KeepTurn`: Active player potted a legal ball without fouling.
- `SwitchTurn`: Active player missed or committed a foul (giving opponent ball-in-hand or table position).
- `GameOver { winner }`: Game-ending condition reached.

## Reference-level explanation

Data structures added to `crates/rules`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BallGroup {
    Solids,
    Stripes,
    EightBall,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EightBallGameState {
    pub player_groups: [Option<BallGroup>; 2],
    pub table_open: bool,
    pub winner: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NineBallGameState {
    pub lowest_active_ball: u32,
    pub consecutive_fouls: [u8; 2],
    pub push_out_allowed: bool,
    pub winner: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StraightPoolGameState {
    pub scores: [i32; 2],
    pub target_score: i32,
    pub consecutive_fouls: [u8; 2],
    pub winner: Option<usize>,
}
```

State transitions are evaluated deterministically from `ShotResult` after `World::step` completes and all balls come to rest.

## Drawbacks

Additional game state bookkeeping adds minor complexity to simulation state snapshots, but keeps game logic decoupled from renderer/UI crates.

## Alternatives considered

- *Embedding rule checking in `crates/ui`*: Rejected because UI should only render simulation/game state, adhering to the standard crate dependency hierarchy.

## Unresolved questions

- Snooker variant rules (15 red balls, color sequences) to be covered in a follow-up RFC (RFC 0008).

## Documentation impact

- Update `docs/architecture/Overview.md` and `docs/api/README.md`.
