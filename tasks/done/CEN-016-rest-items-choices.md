# CEN-016: Items & Rest Choices — Resource Management & Decision Points

**Status**: [ ] Approvato | [/] In Lavorazione | [x] Completato

**Dipendenze**: CEN-015 (Procedural Generation)

**Stima**: 2–3 giorni

---

## 📋 Obiettivo

Aggiungere due dimensioni di scelta strategica:
1. **Items** raccoglibili sui pavimenti (Ration, Whetstone) che creano tradeoff esplorazione vs. beeline
2. **Rest choices** — il Rest screen diventa una schermata interattiva dove il player sceglie tra 3 bonus con diversi trade-off

---

## ✅ Acceptance Criteria

1. **Item types**: Due item collectible:
   - `Ration` (tile ciano): pickup immediato, +10 steps istantaneamente
   - `Whetstone` (tile arancione): tenuto come component `HeldItem`, consumato al rest per +1 force nel calcolo del tier (F4 → F5 per la formula, produce F10 anziché F5)

2. **Item placement**: 1–2 item per floor, posizionati proceduralmente usando `RunSeed`. Evitare spawn (1,1), exit, wall, pillar, enemy.

3. **Item pickup**: Quando il player entra in una tile con item:
   - `Ration`: immediato +10 steps, despawn item
   - `Whetstone`: attach component `HeldItem(Whetstone)` al player, despawn item tile

4. **Rest choices** (player seleziona 1 con tasto 1/2/3):
   ```
   1. Descendi Veloce:      steps + 20, tier recovery standard
   2. Riposo Prolungato:    steps + 40, force unchanged
   3. Allenamento Intensivo: steps + 0,  doppio tier recovery
   ```

5. **Held item effect**: Se player tiene Whetstone al rest:
   - Mostrare indicazione su rest screen: "Whetstone (bonus +1 force tier)"
   - Applicare effetto: `adjusted_force = force + 1; tier_recovery(adjusted_force)`
   - Consumare Whetstone (rimuovere `HeldItem`)

6. **Force tier recovery formula**: Ripreso da rest_plugin esistente:
   ```rust
   fn tier_recovery(force: i32) -> i32 {
       ((force / 5) + 1) * 5
   }
   ```
   Per Allenamento Intensivo, applicare due volte:
   ```rust
   fn double_tier_recovery(force: i32) -> i32 {
       let first = tier_recovery(force);
       tier_recovery(first)
   }
   ```

7. **HUD Rest screen**: Mostrare:
   - Current Floor, Steps Remaining, Current Force
   - **Option 1: Descendi Veloce** → steps: +20, force: tier_recovery(force)
   - **Option 2: Riposo Prolungato** → steps: +40, force: unchanged
   - **Option 3: Allenamento Intensivo** → steps: +0, force: double_tier_recovery(force)
   - Se HeldItem: mostrare "Holding: Whetstone (+1 force tier bonus)"
   - Input: "Premi 1/2/3 per scegliere"

8. **PlayerPersistence update**: Aggiungere campo `held_item: Option<ItemKind>` per cross-floor persistence di Whetstone.

9. **No breaking changes**: Input giocatore (WASD), combattimento, nemici continuano identici.

---

## 🔧 Contesto Tecnico

### Nuovi File

**`src/items/components.rs`:**
```rust
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct Item;

#[derive(Component, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ItemKind {
    Ration,
    Whetstone,
}

#[derive(Component, Clone, Copy)]
pub struct HeldItem(pub ItemKind);
```

**`src/items/mod.rs`:**
```rust
use bevy::prelude::*;
use crate::{map_gen::RoomLayout, config::RunSeed, player::Player};

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Room), spawn_items);
    }
}

fn spawn_items(
    mut commands: Commands,
    config: Res<CenturionConfig>,
    run_seed: Res<RunSeed>,
    layout: Res<RoomLayout>,
) {
    // Genera 1–2 item per floor usando run_seed
    // Chiama generate_item_defs(floor, seed, layout)
}
```

### Item Generation Function (in `map_gen/procgen.rs`)

```rust
pub struct ItemDef {
    pub pos: GridPos,
    pub kind: ItemKind,
}

pub fn generate_item_defs(
    floor: u8,
    run_seed: u64,
    layout: &RoomLayout,
) -> Vec<ItemDef> {
    // Count: 1 + (sub_seed(seed, floor, 200) % 2)  → 1 o 2 item
    // Posizioni: shuffle valide interior tiles, evita spawn(1,1), exit, wall, pillar, enemy
    // Tipo: 50% Ration, 50% Whetstone (basato su sub_seed)
}
```

### Modifiche a file esistenti

**`player/components.rs`:**
```rust
#[derive(Resource, Clone)]
pub struct PlayerPersistence {
    pub steps: i32,
    pub force: i32,
    pub held_item: Option<ItemKind>,  // NEW
}
```

**`player/mod.rs`:**
```rust
// Rimuovi la recovery automatica da on_enter_rest
// on_enter_rest ora solo salva lo stato (steps, force, held_item) a PlayerPersistence
// La recovery vera avviene nel rest choice handler (nuovo)

fn on_enter_rest(
    mut commands: Commands,
    mut player_q: Query<(&CurrentSteps, &Force, Option<&HeldItem>), With<Player>>,
    mut persistence: ResMut<PlayerPersistence>,
) {
    let (steps, force, held_item) = player_q.single_mut();
    *persistence = PlayerPersistence {
        steps: steps.0,
        force: force.0,
        held_item: held_item.map(|h| h.0),
    };
    // Player despawned qui come prima
}
```

**`tactics/movement.rs`:**
```rust
// Aggiungi item pickup branch dopo movimento riuscito:

fn apply_movement(
    // ... existing params ...
    mut item_q: Query<(Entity, &GridPos, &ItemKind), With<Item>>,
) {
    // ... existing movement logic ...
    
    // AFTER successful move to new_pos (before exit check):
    if let Some((item_entity, _, kind)) = item_q.iter().find(|(_, pos, _)| pos == &new_pos) {
        match kind {
            ItemKind::Ration => {
                steps.0 += 10;  // Immediate effect
            },
            ItemKind::Whetstone => {
                commands.entity(player_entity).insert(HeldItem(*kind));
            }
        }
        commands.entity(item_entity).despawn();  // Remove item
    }
    
    // Continue with floor check, exit check, etc.
}
```

**`ui/rest_screen.rs`:**
```rust
// Completamente rewritten (Interactive version)

fn spawn_rest_screen(
    mut commands: Commands,
    config: Res<CenturionConfig>,
    persistence: Res<PlayerPersistence>,
    run_seed: Res<RunSeed>,
) {
    // Root container
    // Title: "FLOOR {floor} CLEARED"
    // Stats section: steps remaining, force, force-after-rest
    // Choices section (3 option buttons or text with keybind display)
    // HeldItem display (if any)
    // Input prompt: "Press 1/2/3"
}

fn handle_rest_choice(
    mut keyboard: ResMut<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut persistence: ResMut<PlayerPersistence>,
) {
    if keyboard.just_pressed(KeyCode::Digit1) {
        apply_rest_choice(RestChoice::DescendFast, &mut persistence);
        next_state.set(GameState::Room);
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        apply_rest_choice(RestChoice::ExtendedRest, &mut persistence);
        next_state.set(GameState::Room);
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        apply_rest_choice(RestChoice::IntensiveTraining, &mut persistence);
        next_state.set(GameState::Room);
    }
}

enum RestChoice {
    DescendFast,
    ExtendedRest,
    IntensiveTraining,
}

fn apply_rest_choice(choice: RestChoice, persistence: &mut PlayerPersistence) {
    match choice {
        RestChoice::DescendFast => {
            persistence.steps += 20;
            persistence.force = apply_held_item_and_recover(
                persistence.force,
                persistence.held_item,
                false,  // single tier
            );
        },
        RestChoice::ExtendedRest => {
            persistence.steps += 40;
            // Force unchanged
            persistence.held_item = None;  // Consume held item but no benefit
        },
        RestChoice::IntensiveTraining => {
            // Steps unchanged
            persistence.force = apply_held_item_and_recover(
                persistence.force,
                persistence.held_item,
                true,  // double tier
            );
        }
    }
    persistence.held_item = None;  // Always consume held item if any
}

fn apply_held_item_and_recover(
    force: i32,
    held_item: Option<ItemKind>,
    double_tier: bool,
) -> i32 {
    let adjusted = if held_item == Some(ItemKind::Whetstone) {
        force + 1
    } else {
        force
    };
    
    if double_tier {
        let first = tier_recovery(adjusted);
        tier_recovery(first)
    } else {
        tier_recovery(adjusted)
    }
}

fn tier_recovery(force: i32) -> i32 {
    ((force / 5) + 1) * 5
}
```

**`plugins/mod.rs`:**
```rust
// Add ItemsPlugin to CenturionPlugins
```

**`player/mod.rs` (on_room_spawn)**:
```rust
// When spawning player from persistence, inizialize HeldItem if present
if let Some(item_kind) = persistence.held_item {
    commands.entity(player_entity).insert(HeldItem(item_kind));
}
```

### Asset / Rendering

**Item tiles**: Ration = cyan square (56px), Whetstone = orange square. Usa `spawn_square` helper da rendering.rs o inline sprite.

---

## 🎮 Gameplay Flow

```
Player on floor → moves around → trovaCration(+10 steps) | trova Whetstone(held)
                → reaches exit → Rest screen

Rest screen:
  Current: Steps 45, Force 8, HeldItem: Whetstone
  
  1. Descendi Veloce:      45 + 20 = 65 steps,  Force (8+1)/5 tier → F10
  2. Riposo Prolungato:    45 + 40 = 85 steps,  Force 8 (unchanged)
  3. Allenamento Intensivo: 45 + 0 = 45 steps,  Force (8+1)→F10→F15

Player presses 1 → enters floor with 65 steps, 10 force, no held item
```

---

## 🧪 Testing Plan

1. **Item pickup**: Spawn Ration, cammina su di essa, verifica steps +10 immediato.
2. **Whetstone carry**: Pickup Whetstone, vai al rest, verifica HeldItem display.
3. **Rest choice 1 (Descendi Veloce)**: Arrive with F8, scelta 1, spawn next floor, verifica steps = prev + 20, force = F10.
4. **Rest choice 2 (Extended Rest)**: Arrive with F8, 20 steps, scelta 2, verifica steps = 60, force = F8 (unchanged).
5. **Rest choice 3 (Intensive)**: Arrive with F3, scelta 3, verifica force = F5→F10 (double tier), steps = prev + 0.
6. **Whetstone effect**: Arrive with F4, holding Whetstone, scelta 1, verifica force = (4+1)→F10 anziché F5.
7. **Multi-floor run**: 3 floor con vari item + choice combinations, no crash.

---

## 📝 Note

- **Input**: Da KeyCode::Digit1 a Digit3 (tasti numerici 1, 2, 3 sulla tastiera principale, non numpad).
- **HeldItem persistence**: Serializzabile in `PlayerPersistence` per salvataggi futuri.
- **Whetstone display**: Mostrare chiaramente "Holding Whetstone: bonus +1 force rank" per non confondere il player.
- **Rest screen respawning**: Rest screen va in `OnExit(GameState::Room)` / `OnEnter(GameState::Rest)`.
- **Next floor spawn**: Quando transitions da Rest → Room (Digit press), il player spawn system crea nuovo player da `PlayerPersistence` updated con la scelta.

---

*Versione: 1.0 | Creato: 2026-04-28*
