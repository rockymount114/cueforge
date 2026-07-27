# AI Coach (Overview)

## Purpose

The AI Coach is the umbrella feature that ties together shot evaluation,
pattern recognition, position prediction, and safety analysis
(`ShotEvaluation.md`, `PatternRecognition.md`, `PositionPrediction.md`,
`SafetyAnalysis.md`) into player-facing guidance: "here's the best shot,"
"here's why your position play went wrong," "here's a drill to fix this."

## Design principle: physics-grounded, not black-box

Unlike a pure machine-learning approach trained only on outcomes, the AI
Coach is built on top of CueForge's deterministic physics core — it can
literally simulate candidate shots (`crates/physics`) to evaluate them,
rather than only pattern-matching against a training set. Learned
components (if any, e.g. for human-like difficulty estimation) sit on top
of this simulated ground truth, not in place of it.

## Components

| Component | Doc | Role |
|---|---|---|
| Shot evaluation | `ShotEvaluation.md` | scores candidate shots (make probability, resulting position) |
| Pattern recognition | `PatternRecognition.md` | identifies the table layout pattern (clusters, safeties, run-out feasibility) |
| Ghost ball visualization | `GhostBall.md` | player-facing aim visualization |
| Position prediction | `PositionPrediction.md` | predicts cue ball position after a shot |
| Safety analysis | `SafetyAnalysis.md` | evaluates defensive shot options |
| Difficulty generation | `DifficultyGeneration.md` | generates training drills at a target difficulty |

## Status

Design-stage; implementation targeted at Roadmap milestone M5, gated on
RFC 0005 (`rfcs/0005-ai-coach.md`).
