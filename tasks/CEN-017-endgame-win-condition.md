# CEN-017: Endgame, Win Condition & Score System

**Status**: [ ] Approvato | [/] In Lavorazione | [x] Completato

**Dipendenze**: CEN-016 (Rest Choices & Items)

**Stima**: 1–2 giorni

---

## 📋 Obiettivo

Definire la conclusione di una run e introdurre un sistema di punteggio che incentiva l'ottimizzazione. Una run ha un obiettivo chiaro (Floor 10 = vittoria) e un numero (score) che permette al player di confrontare le proprie prestazioni.

---

## ✅ Acceptance Criteria

1. **Floor 10 = Final Boss**: Al raggiungimento del floor 10, spawna un unico nemico (Boss) al centro (4,4) con:
   ```
   boss_force = player_entry_force * 2
   ```
   Layout di floor 10: fisso (no pilastri procedurali), nessun altro nemico.

2. **Win condition**: Quando il player sconfigge il boss (ultimo nemico su floor 10):
   - Transition a `GameState::WinScreen` (nuovo stato)
   - Dead screen non appare

3. **RunStats resource** (nuovo): Traccia stats per il calcolo del punteggio:
   ```rust
   #[derive(Resource, Default)]
   pub struct RunStats {
       pub floors_cleared: u8,
       pub total_steps_taken: i32,
       pub enemies_defeated: u32,
       pub items_collected: u32,
   }
   ```
   - `floors_cleared`: incrementa quando player entra in Rest (floor + 1)
   - `total_steps_taken`: accumula ogni -1 step in movement
   - `enemies_defeated`: incrementa su `PlayerWins` in resolve_combat
   - `items_collected`: incrementa su item pickup

4. **Score formula** (calcolata al raggiungimento di WinScreen o DeadScreen):
   ```
   score = (floors_cleared * 100) + (steps_remaining * 2) + (force * 10) - (total_steps_taken / 2)
   ```
   - floors_cleared: incentiva progressione
   - steps_remaining: incentiva efficienza di movimento (beeline vs. explore)
   - force: incentiva non perdere combattimenti
   - total_steps_taken: penalizza deambulazione inutile

5. **WinScreen**: Nuovo stato che mostra:
   - "🏆 CENTURION ASCENDS" (titolo vittoria, colore dorato/giallo)
   - "Floors Cleared: N"
   - "Final Force: N"
   - "Steps Remaining: N"
   - "Score: NNNN"
   - "Seed: 0x..."
   - "Press R to restart"

6. **DeadScreen update**: Aggiungi score al dead screen esistente. Formato:
   - Titolo "CENTURION FALLS" (rosso)
   - Stats come prima
   - "+ Score: NNNN"
   - "Seed: 0x..."
   - "Press R to restart"

7. **Seed persistence**: Sia WinScreen che DeadScreen mostrano il RunSeed per reproducibility/sharing.

8. **No breaking changes**: Floors 1–9 continuano identici; floor 10 è solo il boss finale.

---

## 🔧 Contesto Tecnico

### Stato Machine Update

**`state.rs`:**
```rust
pub enum GameState {
    Loading,
    Room,
    CombatEvent,
    Rest,
    Dead,
    WinScreen,  // NEW
}
```

### Nuovo File: `ui/win_screen.rs`

```rust
use bevy::prelude::*;
use crate::{config::{CenturionConfig, RunStats}, GameState, player::PlayerPersistence};

pub struct WinScreenPlugin;

impl Plugin for WinScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::WinScreen), spawn_win_screen)
            .add_systems(Update, handle_win_screen_input.run_if(in_state(GameState::WinScreen)))
            .add_systems(OnExit(GameState::WinScreen), despawn_win_screen);
    }
}

fn spawn_win_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<CenturionConfig>,
    stats: Res<RunStats>,
    persistence: Res<PlayerPersistence>,
    run_seed: Res<RunSeed>,
) {
    let score = calculate_score(&stats, persistence.steps, persistence.force);
    
    // Root container (full screen, centered, dark background)
    // Title: "🏆 CENTURION ASCENDS" (yellow, 48px)
    // Stats section:
    //   "Floors Cleared: {}"
    //   "Final Force: {}"
    //   "Steps Remaining: {}"
    //   "Score: {}" (bright, 32px)
    // "Seed: 0x{:016x}" (gray, 16px)
    // "Press R to restart" (gray, 20px)
}

fn handle_win_screen_input(
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut config: ResMut<CenturionConfig>,
    mut commands: Commands,
) {
    if input.just_pressed(KeyCode::KeyR) {
        config.current_floor = 1;
        commands.remove_resource::<RunSeed>();
        commands.remove_resource::<RunStats>();
        commands.remove_resource::<PlayerPersistence>();
        next_state.set(GameState::Loading);
    }
}

fn despawn_win_screen(mut commands: Commands, screen_q: Query<Entity, With<WinScreenMarker>>) {
    for entity in screen_q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
pub struct WinScreenMarker;

fn calculate_score(stats: &RunStats, steps_remaining: i32, force: i32) -> i32 {
    (stats.floors_cleared as i32 * 100)
        + (steps_remaining * 2)
        + (force * 10)
        - (stats.total_steps_taken / 2)
}
```

### Modifiche a file esistenti

**`state.rs`:**
```rust
// Aggiungi nuovo stato GameState::WinScreen
// Documenta transizione: CombatEvent → WinScreen (su floor 10, last enemy defeated)
```

**`config.rs`:**
```rust
#[derive(Resource, Default, Debug)]
pub struct RunStats {
    pub floors_cleared: u8,
    pub total_steps_taken: i32,
    pub enemies_defeated: u32,
    pub items_collected: u32,
}

impl RunStats {
    pub fn calculate_score(&self, steps_remaining: i32, force: i32) -> i32 {
        (self.floors_cleared as i32 * 100)
            + (steps_remaining * 2)
            + (force * 10)
            - (self.total_steps_taken / 2)
    }
}
```

**`tactics/movement.rs`:**
```rust
// In apply_movement, quando steps decrementa:
fn apply_movement(
    // ... existing params ...
    mut stats: ResMut<RunStats>,
) {
    // ... movement logic ...
    
    // Dopo successful move (prima di exit check):
    if moved {
        stats.total_steps_taken += 1;  // Increment by steps consumed
        steps.0 -= 1;
    }
}
```

**`resolver/mod.rs`:**
```rust
// In resolve_combat system:
fn resolve_combat(
    // ... existing params ...
    mut stats: ResMut<RunStats>,
    mut next_state: ResMut<NextState<GameState>>,
    config: Res<CenturionConfig>,
) {
    // ... existing resolve logic ...
    
    // After PlayerWins:
    if let CombatResult::PlayerWins { new_force } = result {
        stats.enemies_defeated += 1;
        
        // Check floor 10 endgame
        if config.current_floor == 10 && enemy_q.is_empty() {
            // Last enemy defeated on floor 10
            next_state.set(GameState::WinScreen);
            return;  // Skip Rest screen
        }
        
        // Otherwise, proceed to Rest as normal
        next_state.set(GameState::Rest);
    }
}
```

**`ui/dead_screen.rs`:**
```rust
// Aggiungi score display:
fn spawn_dead_screen(
    // ... existing params ...
    stats: Res<RunStats>,
) {
    let score = stats.calculate_score(steps_remaining, force);
    
    // ... existing title/stats ...
    
    // NEW: Add score line
    // "Score: {}" (white, 24px)
    
    // ... existing seed display ...
    // "Seed: 0x{:016x}"
}
```

**`items/mod.rs`:**
```rust
// In spawn_items, quando item è piazzato, increment stats:
// Nota: il counting avviene in tactics/movement al pickup, non qui
```

**`tactics/movement.rs`:**
```rust
// In apply_movement, item pickup branch:
if let Some((item_entity, _, kind)) = item_q.iter().find(...) {
    match kind {
        ItemKind::Ration => {
            stats.items_collected += 1;  // NEW
            steps.0 += 10;
        },
        ItemKind::Whetstone => {
            stats.items_collected += 1;  // NEW
            commands.entity(player_entity).insert(HeldItem(*kind));
        }
    }
    commands.entity(item_entity).despawn();
}
```

**`player/mod.rs` (on_enter_rest):**
```rust
// Increment floors_cleared quando entra Rest:
fn on_enter_rest(
    // ... existing params ...
    mut stats: ResMut<RunStats>,
) {
    stats.floors_cleared = config.current_floor;  // Sync with current floor
    // ... existing rest logic ...
}
```

**`plugins/state_plugin.rs` (loading → room):**
```rust
// In loading_to_room system:
fn loading_to_room(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Crea RunStats fresh
    commands.insert_resource(RunStats::default());
    
    // Crea RunSeed
    let seed = rand::thread_rng().gen::<u64>();
    commands.insert_resource(RunSeed(seed));
    
    next_state.set(GameState::Room);
}
```

### Boss Generation (in `enemies/mod.rs` o `map_gen/procgen.rs`)

```rust
pub fn generate_enemy_defs(
    floor: u8,
    run_seed: u64,
    layout: &RoomLayout,
    player_force: i32,  // NEW parameter on floor 10
) -> Vec<EnemyDef> {
    if floor == 10 {
        // Boss fight: single enemy at center
        return vec![
            EnemyDef {
                pos: GridPos { x: 4, y: 4 },
                force: player_force * 2,
                behavior: EnemyBehavior::Static,
            }
        ];
    }
    
    // Normal generation for floors 1–9
    // ... existing procgen logic ...
}
```

Tuttavia, per ottenere `player_force` al momento della generazione, è necessario che il player sia già spawnato. Alternative:

**A) Lazy spawn boss**: Durante `OnEnter(Room)` su floor 10, dopo che il player è spawnato, spawn il boss con force = player_force * 2.

**B) Store in persistence**: Il player_force dal floor precedente è salvato in `PlayerPersistence.force`; usalo per calcolare boss_force.

Opzione B è più pulita:

```rust
fn spawn_enemies(
    mut commands: Commands,
    config: Res<CenturionConfig>,
    run_seed: Res<RunSeed>,
    layout: Res<RoomLayout>,
    persistence: Res<PlayerPersistence>,  // NEW
) {
    let defs = if config.current_floor == 10 {
        vec![EnemyDef {
            pos: GridPos { x: 4, y: 4 },
            force: persistence.force * 2,
            behavior: EnemyBehavior::Static,
        }]
    } else {
        generate_enemy_defs(config.current_floor, run_seed.0, &layout)
    };
    
    // ... spawn defs ...
}
```

---

## 🎮 Gameplay Flow

```
Floor 1–9:  Normal loop (movement, combat, items, rest choices)
Floor 10:   Player spawns, sees single enemy at (4,4) with force = entry_force * 2
            Combat vs boss
            If player wins → WinScreen (score calculated)
            If player loses → DeadScreen (score calculated from run so far)

Score example:
  Run: 10 floors cleared, 60 steps remaining, 25 force, 200 steps taken
  Score = (10 * 100) + (60 * 2) + (25 * 10) - (200 / 2)
         = 1000 + 120 + 250 - 100
         = 1270

WinScreen shows:
  🏆 CENTURION ASCENDS
  Floors Cleared: 10
  Final Force: 25
  Steps Remaining: 60
  Score: 1270
  Seed: 0x1a2b3c4d5e6f7g8h
  Press R to restart
```

---

## 🧪 Testing Plan

1. **Floor 10 spawn**: Reach floor 10, verifica che il boss spawni a (4,4) con force = entry_force * 2.
2. **Boss fight win**: Combatti il boss con force abbastanza alta, sconfiggilo, verifica WinScreen appare.
3. **Score calculation**: Nota il punteggio, calcola manualmente, verifica match.
4. **Seed display**: Vedi seed su WinScreen, ricopia in run successiva, verifica identico layout.
5. **R restart**: Premi R, verifica game torna a Loading, RunSeed e RunStats sono rimossi.
6. **DeadScreen score**: Muori a floor 5, verifica score su dead screen è calcolato correttamente da stats parziali.

---

## 📝 Note

- **Floor 10 layout**: Usa layout fisso (no pilastri). Opzione: disabilita pillar placement se floor == 10, o crea hard-coded layout.
- **Boss is Static**: Il boss non si muove; è un test di pure force. Il player ha tutto il tempo di avvicinarsi.
- **Score balance**: I pesi (100 per floor, 2 per step, 10 per force, -0.5 per step taken) possono essere tuned in base al feedback.
- **Persistence reset**: On R press in WinScreen/DeadScreen, rimuovi RunSeed, RunStats, PlayerPersistence per una run pulita.

---

*Versione: 1.0 | Creato: 2026-04-28*
