# Components (Data Model Reference)

## Ball components

| Component | Type | Notes |
|---|---|---|
| `Position` | `Vec3<Meters>` | table-space, origin at table center, see `docs/physics/CoordinateSystem.md` |
| `Velocity` | `Vec3<MetersPerSecond>` | |
| `AngularVelocity` | `Vec3<RadiansPerSecond>` | drives spin/English effects |
| `Radius` | `Meters` | standard ball radius, configurable per ruleset |
| `Mass` | `Kilograms` | |
| `BallState` | enum: `Stationary`, `Sliding`, `Rolling`, `Airborne`, `Pocketed` | drives which physics system applies each tick |
| `BallId` | newtype `u8` | stable identity used for deterministic ordering |

## Table components

| Component | Type | Notes |
|---|---|---|
| `RailGeometry` | polyline / segment list | cushion shape per `docs/physics/Rail.md` |
| `PocketGeometry` | position + capture radius, per pocket | `docs/physics/Pocket.md` |
| `ClothFriction` | struct `{ rolling, sliding }` coefficients | `docs/physics/Rolling.md`, `Sliding.md` |
| `CushionRestitution` | coefficient | `docs/physics/Rail.md` |

## Cue / shot components

| Component | Type | Notes |
|---|---|---|
| `TipOffset` | `Vec2<Meters>` from ball center | drives English/spin at impact |
| `ImpactForce` | `Newtons` | |
| `CueAngle` | elevation angle, for jump/massé shots | `docs/physics/Jump.md`, `Massé.md` |

## Design rule

Components are plain data — no behavior. Anything that reads or mutates
components lives in a system (`Systems.md`). This keeps the data model
serializable as-is for `replay` and `networking` without custom logic.
