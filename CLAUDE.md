# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Centurion: 100 Steps** is a tactical roguelike dungeon crawler built with Rust + Bevy 0.18. Players navigate deterministic 8×8 grid rooms, fight enemies, manage a step budget, and climb procedurally generated floors. Core mechanic: no randomness in combat — all outcomes are deterministic math.

## Commands

```bash
# Development (dynamic linking for fast recompiles)
cargo run

# Release build (WASM-optimized: LTO, size-optimized)
cargo build --release

# Run unit tests
cargo test

# WASM target
cargo build --target wasm32-unknown-unknown --release

# Lint
cargo clippy

# Format
cargo fmt
```

## Architecture

### Plugin Structure

`main.rs` creates an 800×800 Bevy app and registers `CenturionPlugins`, a `PluginGroup` defined in `src/plugins/mod.rs`:

```
CenturionPlugins
  ├── CenturionRenderPlugin   — spawn_square() / spawn_circle() helpers
  ├── StatePlugin             — GameState setup
  ├── MapGenPlugin            — deterministic 8×8 room generation
  ├── PlayerPlugin            — spawn, stat sync, rest logic
  ├── InputPlugin             — WASD/Arrow → MoveIntent
  ├── TacticsPlugin           — MoveIntent → movement + CombatIntent
  ├── EnemiesPlugin           — per-floor enemy spawning
  ├── ResolverPlugin          — combat math + 0.2s flash animation
  └── HudPlugin               — HUD, Rest screen, Dead screen
```

### State Machine

```
Loading → Room ←→ Rest
            ↓
        CombatEvent (0.2s flash)
            ↓
         Room (victory) | Dead (defeat)
Dead → Loading (restart)
```

Defined in `src/state.rs`. All in-game entities use `DespawnOnExit(GameState)` for clean state transitions.

### Data Flow Per Tick

1. `read_player_input()` (`input.rs`) — keyboard → `MoveIntent`
2. `apply_movement()` (`tactics/movement.rs`) — `MoveIntent` → update `GridPos`, decrement `CurrentSteps`, emit `CombatIntent`
3. `resolve_combat()` (`resolver/mod.rs`) — `CombatIntent` → call `resolve()` in `resolver/combat.rs`
4. `update_flash()` (`resolver/flash.rs`) — 0.2s color lerp on player sprite
5. `update_steps_display()` (`ui/hud.rs`) — sync `PlayerStats` resource to HUD text

### Key Components & Resources

| Name | Kind | Purpose |
|---|---|---|
| `GameState` | Enum | State machine |
| `CenturionConfig` | Resource | `current_floor`, window dimensions |
| `PlayerPersistence` | Resource | Cross-floor state (force, steps) |
| `PlayerStats` | Resource | Synced display values for HUD |
| `RoomLayout` | Resource | 8×8 tile grid for current room |
| `GridPos` | Component | Grid position (0..8 × 0..8) |
| `TileKind` | Component | Floor / Wall / Exit |
| `CurrentSteps` | Component | Remaining steps this floor |
| `Force` | Component | Player combat power |
| `EnemyForce` | Component | Enemy combat power |
| `MoveIntent` | Message | Directional input signal |
| `CombatIntent` | Message | Attacker, defender, target position |

### Combat System

`resolver/combat.rs::resolve()` — deterministic: `result = player_force - enemy_force`. Unit tests live in that file. No RNG involved; seeded `rand` is only used in map generation (`map_gen/room.rs`).

### Grid / World Coordinate System

`map_gen/room.rs` owns `grid_to_world()` and `build_room()`. The grid is 8×8 with 64px tiles. Entities are rendered at ~87.5% tile size (56px squares).

### Rest Screen Logic

`player/mod.rs::on_enter_rest()` applies tier recovery: player Force is rounded up to the next multiple of 5, plus a +20 steps bonus for the next floor.

## Key Design Constraints

- **No diagonal movement** — only cardinal directions (WASD / Arrow keys).
- **Deterministic combat** — never introduce RNG into `resolver/`.
- **State-scoped despawn** — tag new in-game entities with `DespawnOnExit(GameState::Room)` (or appropriate state) to avoid leaks across transitions.
- **Grid bounds** — always validate movement stays within 0..8 on both axes before updating `GridPos`.
