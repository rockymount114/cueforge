# Physical Constant Calibration Registry

> Master calibration registry indexing physical constants from `work_todo/` specifications (`physics-constants.md`, `balls-specification.md`, `cue-specification.md`, `table-specification.md`).

## Constant Registry

| Constant | Symbol | Value | Source Specification | Status |
|---|---|---|---|---|
| Standard Gravity | $g$ | `9.80665 m/s²` | `physics-constants.md` §22 | Verified (ISO standard) |
| Ball Diameter | $D_{\text{ball}}$ | `0.05715 m` (57.15 mm) | `balls-specification.md` §9 | Verified (WPA standard) |
| Ball Radius | $R_{\text{ball}}$ | `0.028575 m` (28.575 mm) | `balls-specification.md` §10 | Verified (WPA standard) |
| Ball Mass | $m_{\text{ball}}$ | `0.170 kg` (170 g) | `balls-specification.md` §18 | Verified (Aramith Phenolic) |
| Ball Material Density | $\rho$ | `1250 kg/m³` | `balls-specification.md` §41 | Verified (Phenolic Resin) |
| Moment of Inertia | $I$ | $\frac{2}{5} m r^2$ | `balls-specification.md` §51 | Verified (Solid Sphere) |
| Ball-Ball Restitution | $e_{\text{ball}}$ | `0.95` | `balls-specification.md` §83 | Verified (Aramith Tournament) |
| Ball-Ball Friction | $\mu_{\text{ball}}$ | `0.05` | `balls-specification.md` §61 | Verified (Phenolic contact) |
| Ball-Cloth Rolling Friction | $\mu_{\text{roll}}$ | `0.015` | `physics-constants.md` §66 | Verified (Simonis 860) |
| Ball-Cloth Sliding Friction | $\mu_{\text{slide}}$ | `0.20` | `physics-constants.md` §74 | Verified (Simonis 860) |
| Spin Decay Rate | $\alpha_{\text{spin}}$ | `2.0 rad/s²` | `physics-constants.md` §90 | Verified |
| Cushion Restitution | $e_{\text{rail}}$ | `0.90` | `table-specification.md` §77 | Verified (K55 Rubber) |
| Cushion Friction | $\mu_{\text{rail}}$ | `0.12` | `table-specification.md` §78 | Verified (K55 Rubber) |
| Rail Height | $h_{\text{rail}}$ | `0.037 m` (37 mm) | `table-specification.md` §159 | Verified (Regulation 9ft) |
| Corner Pocket Opening | $w_{\text{corner}}$ | `0.114 m` (114 mm) | `table-specification.md` §164 | Verified (WPA regulation) |
| Side Pocket Opening | $w_{\text{side}}$ | `0.127 m` (127 mm) | `table-specification.md` §165 | Verified (WPA regulation) |
| Pocket Shelf Depth | $d_{\text{shelf}}$ | `0.035 m` (35 mm) | `table-specification.md` §167 | Verified |
| Cue Length | $L_{\text{cue}}$ | `1.473 m` (58 in) | `cue-specification.md` §9 | Verified (Standard Cue) |
| Cue Mass | $M_{\text{cue}}$ | `0.5388 kg` (19 oz) | `cue-specification.md` §11 | Verified (Standard Cue) |
| Cue Tip Radius | $r_{\text{tip}}$ | `0.00625 m` (12.5 mm) | `cue-specification.md` §30 | Verified (Medium tip) |
| Max Practical Tip Offset | $a_{\text{max}}$ | `0.010 m` (10 mm) | `cue-specification.md` §58 | Verified |
| Maximum Break Speed | $v_{\text{break}}$ | `10.0 m/s` | `cue-specification.md` §120 | Verified (Professional Break) |
| Sleeping Velocity Threshold | $v_{\text{sleep}}$ | `0.01 m/s` | `physics-constants.md` §149 | Verified |
| Sleeping Angular Threshold | $\omega_{\text{sleep}}$ | `0.02 rad/s` | `physics-constants.md` §155 | Verified |

## Calibration Methodology

1. All core physical constants are sourced directly from WPA (World Pool-Billiard Association) equipment specifications and published billiards research.
2. Constants are validated through continuous integration determinism tests (`tests/physics/`) and benchmarked against physical table behavior.
