# RFC 0007: Game Rules Engine

- **Status**: Accepted
- **Author(s)**: CueForge Core Team
- **Created**: 2026-07-28
- **Related crates**: `crates/rules`

## Summary

This RFC specifies the game state machines, foul evaluation logic, turn switching, scoring systems, and rack resets for official cue sport variants: **8-Ball**, **WPA Official 9-Ball**, and **Straight Pool (14.1 Continuous)**, aligned with WPA (World Pool-Billiard Association) standards.

## Motivation

While `crates/rules` initially provided basic event filtering, formal WPA 9-ball tournament rules require strict enforcement of:
- **Rotation Rule**: Cue ball must hit lowest-numbered active object ball first (`FoulType::WrongBallFirst`).
- **Rail Contact Requirement**: After initial contact, at least one ball must pocket or contact a rail (`FoulType::NoRailContact`).
- **Push Out Mechanics**: Available exclusively immediately after a legal break shot (`announce_push_out` and pass-back options).
- **WPA 9-Ball Re-spotting**: 9-ball is re-spotted on foot spot if pocketed on break (WPA rule) or pocketed during a foul.
- **Three-Foul Loss**: 3 consecutive fouls without a legal shot in between results in loss of frame/game.

## Guide-level explanation

The `RuleEngine` maintains `GameState` across consecutive shots:
- `EightBallState`: Open table, assigned player groups (`Solids` or `Stripes`), 8-ball call-shot target.
- `NineBallState`: WPA rotation enforcement (`lowest_active_ball`), push-out mechanics (`push_out_available`, `push_out_active`), 9-ball re-spotting on break/foul, and 3-foul penalty counter.
- `StraightPoolState`: Cumulative call-shot point tracker, 14-ball rack reset state.

Physical shot event streams (`ShotResult`) are evaluated at the end of each simulation turn to produce `TurnOutcome`:
- `KeepTurn`: Active player potted a legal ball without fouling.
- `SwitchTurn { ball_in_hand }`: Active player missed or committed a foul (giving opponent ball-in-hand or table position).
- `PushOutOffered`: Push out shot completed; opponent has option to shoot from position or pass turn back.
- `GameOver { winner }`: Game-ending condition reached (legal 9-ball pot, 3-foul loss, 8-ball pot).

## Reference-level explanation

Data structures in `crates/rules`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoulType {
    Scratch,
    NoContact,
    WrongBallFirst,
    NoRailContact,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NineBallState {
    pub lowest_active_ball: u32,
    pub consecutive_fouls: [u8; 2],
    pub active_player: usize,
    pub winner: Option<usize>,
    pub is_break_shot: bool,
    pub push_out_available: bool,
    pub push_out_active: bool,
    pub wpa_spot_9ball_on_break: bool,
    pub respawn_9ball_needed: bool,
}
```

State transitions are evaluated deterministically from `ShotResult` after `World::step` completes and all balls come to rest.

## Drawbacks

Adds push-out state handling, but ensures 100% compliance with WPA rules.

## Alternatives considered

- *Amateur league immediate win on break*: Supported as an optional toggle (`wpa_spot_9ball_on_break: false`).

## Unresolved questions

- Snooker variant rules (15 red balls, color sequences) to be covered in RFC 0008.

## Documentation impact

- Update `docs/architecture/Overview.md` and `docs/api/README.md`.
