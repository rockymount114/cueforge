# Cloth Modeling & Specifications

> Specifications based on official 9-ball tournament standards (`work_todo/9ball_cloth_spec.md`).

## Scope

CueForge models cloth dynamics as configurable physical friction, deceleration, spin decay, and environmental response parameters exposed through `ClothSpec` in `crates/table`.

## Tournament Standard: Simonis 860 Worsted Cloth

The default cloth specification in CueForge is **Simonis 860**, the international tournament standard for professional 9-ball competition.

| Parameter | Value | Description |
|---|---|---|
| Material | 90% Wool / 10% Nylon | Worsted weave, napless surface |
| Weave | Worsted | Directionless smooth roll |
| Base Rolling Friction ($\mu_r$) | `0.015` | Steady-state rolling resistance |
| Base Sliding Friction ($\mu_s$) | `0.20` | Sliding friction during draw/follow transition |
| Linear Deceleration | `0.22 m/s²` | Ball deceleration rate on bed |
| Spin Decay | `2.0 rad/s²` | Angular velocity dissipation rate |
| Restitution Energy Retained | `97%` (3% loss) | High energy transfer retention |
| Relative Humidity | `45%` | Tournament ideal ambient humidity |
| Temperature | `22°C` | Tournament ideal ambient temperature |
| Color | `#0055a5` | Tournament Blue |

## Environmental Dynamics (Humidity & Temperature)

Ambient humidity and temperature modify effective friction coefficients dynamically:

$$\mu_{r,\text{eff}} = \frac{\mu_{r,\text{base}} \cdot \left(1 + 0.005 \cdot (H - 45)\right) \cdot \left(1 - 0.003 \cdot (T - 22)\right)}{S_{\text{mult}}}$$

$$\mu_{s,\text{eff}} = \frac{\mu_{s,\text{base}} \cdot \left(1 + 0.003 \cdot (H - 45)\right) \cdot \left(1 - 0.002 \cdot (T - 22)\right)}{S_{\text{mult}}}$$

- **High Humidity ($> 50\%$)**: Increases cloth moisture, raising friction and slowing table speed.
- **Warmer Temperature ($> 22^\circ\text{C}$)**: Dries cloth fibers, reducing friction and speeding up table.

## Presets Supported (`crates/table`)

1. **`Simonis 860`**: Tournament Blue worsted cloth (Medium-Fast).
2. **`Simonis 760`**: Ultra-fast tournament worsted cloth.
3. **`Standard League Cloth`**: Napped wool/nylon cloth (Classic Green).
4. **`Old Bar Table Cloth`**: High friction, slow napped cloth.
