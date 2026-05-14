# CEN-015: Procedural Generation — Run Variety & Deterministic Seeding

**Status**: [ ] Approvato | [/] In Lavorazione | [x] Completato

**Dipendenze**: CEN-014 (Enemy Movement)

**Stima**: 2–3 giorni

---

## 📋 Obiettivo

Implementare generazione procedurale deterministica basata su seed. Ogni run ottiene un seed `u64` casuale; tutti i layout di stanza, posizioni nemici, e forze derivano deterministicamente da quel seed. Il risultato: nessuna run è identica, ma ogni run è riproducibile e condivisibile.

---

## ✅ Acceptance Criteria

1. **RunSeed resource**: `struct RunSeed(u64)` creato in Loading state, rimosso al restart (Dead → Loading).

2. **Sub-seeding deterministico**: Funzione `sub_seed(run_seed, floor, salt) → u64` che genera seed dipendenti dal piano senza RNG.
   ```rust
   fn sub_seed(run_seed: u64, floor: u8, salt: u64) -> u64 {
       run_seed.wrapping_mul(6364136223846793005)
           .wrapping_add(floor as u64).wrapping_add(salt)
   }
   ```

3. **Exit placement**: Posizionato in uno degli 8 border-interior positions (x=1 o 6, y=1..6, oppure y=1 o 6, x=1..6), scelto da `sub_seed(seed, floor, 0) % 8`.

4. **Internal pillars** (wall tiles): 0–3 pilastri posizionati su 9 candidate position (3×3 grid interno al centro). Placement: `sub_seed(seed, floor, salt) % 3 != 0` per ogni candidato. **Validazione**: dopo placement, verifica BFS connectivity da (1,1) all'exit; se disconnesso, rimuovi pilastri uno a uno finché il path riapresi.

5. **Enemy generation**: 2–4 nemici per piano. Count: `2 + (sub_seed(seed, floor, 10) % 3)`. Posizioni randomizzate tra 6×6 interior, evitando: spawn player (1,1), exit tile, wall tiles, pillar tiles, altri nemici. Forze: `base_force + (sub_seed(seed, floor, enemy_index) % variance)`. Comportamenti: scalano con floor (più Guard/Patrol su floor alti).

6. **No layout repeats**: Riavvia il gioco 5 volte, verifica che nessun floor abbia lo stesso layout.

7. **Reproducibility**: Stessa RunSeed produce esattamente lo stesso layout (salvo cambio di codice).

8. **Seed display**: Death screen mostra il seed per condivisione: "Seed: 0x1a2b3c4d5e6f7g8h".

---

## 🔧 Contesto Tecnico

### Nuovo File: `src/map_gen/procgen.rs`

```rust
// Main generation entry point
pub fn generate_room_layout(
    floor: u8,
    run_seed: u64,
) -> (RoomLayout, GridPos) {
    // Crea layout con pilastri, ritorna layout + exit_pos
}

pub fn generate_enemy_defs(
    floor: u8,
    run_seed: u64,
    layout: &RoomLayout,
) -> Vec<EnemyDef> {
    // Genera 2–4 EnemyDef con posizioni, forze, comportamenti
}

// Helpers
fn sub_seed(run_seed: u64, floor: u8, salt: u64) -> u64 { ... }

fn place_pillars(floor: u8, seed: u64, layout: &mut RoomLayout) -> Result<(), String> {
    // Piazza 0–3 pilastri
    // Se BFS fallisce, rimuovi pilastri finché connesso
    // Return Ok(()) o Err("unreachable") se impossibile
}

fn place_exit(floor: u8, seed: u64, layout: &RoomLayout) -> GridPos {
    // Scegli uno degli 8 border-interior tile
}

fn is_reachable(from: GridPos, to: GridPos, layout: &RoomLayout) -> bool {
    // BFS flood-fill da from; return true se to raggiungibile
}

fn shuffle_candidates(
    candidates: Vec<GridPos>,
    seed: u64,
) -> Vec<GridPos> {
    // Shuffle lista usando sub_seed come PRNG state
    // Implementazione: Fisher-Yates con seed come RNG source
}
```

### Modifiche a file esistenti

**`map_gen/room.rs`:**
```rust
// Vecchia firma
pub fn build_room(floor: u8) -> RoomLayout { ... }

// Nuova firma
pub fn build_room(floor: u8, run_seed: u64) -> RoomLayout {
    // Chiama generate_room_layout internamente
}

// Alternativa (meno breaking):
pub fn build_room_proc(floor: u8, run_seed: u64) -> RoomLayout { ... }
// Mantieni build_room(floor) per compatibilità backward (fallback a hardcoded)
```

**`config.rs`:**
```rust
#[derive(Resource)]
pub struct RunSeed(pub u64);

#[derive(Resource)]
pub struct RunStats {
    pub floors_cleared: u8,
    pub total_steps_taken: i32,
    pub enemies_defeated: u32,
    pub items_collected: u32,
}
```

**`plugins/state_plugin.rs`:**
```rust
// In loading_to_room system o setup:
fn setup_run_seed(
    mut commands: Commands,
) {
    let seed = rand::thread_rng().gen::<u64>();
    commands.insert_resource(RunSeed(seed));
}

// On entering Dead state (before exiting):
// - Save seed per display su dead screen
// - On entering Loading:
// - RemoveResource::<RunSeed>()
```

**`map_gen/mod.rs`:**
```rust
pub fn spawn_room(
    mut commands: Commands,
    config: Res<CenturionConfig>,
    run_seed: Res<RunSeed>,
) {
    let layout = build_room(config.current_floor, run_seed.0);
    // ... continua con spawn tiles
}
```

**`enemies/mod.rs`:**
```rust
pub fn spawn_enemies(
    mut commands: Commands,
    config: Res<CenturionConfig>,
    run_seed: Res<RunSeed>,
    layout: Res<RoomLayout>,
) {
    let defs = generate_enemy_defs(config.current_floor, run_seed.0, &layout);
    for def in defs {
        // spawn enemy con behavior
    }
}
```

**`ui/dead_screen.rs`:**
```rust
// Aggiungi RunSeed al display
// Mostra "Seed: 0x{:x}" format
```

### RNG Handling

- **RunSeed initialization**: Usa `use rand::Rng; rand::thread_rng().gen::<u64>()` in Loading
- **No rand in resolver**: Zero RNG nel combattimento
- **Sub-seeding PRNG**: Implementazione di Fisher-Yates shuffle per `shuffle_candidates` usando `sub_seed` come state (no `rand::` inside procgen per purezza, o usa `rand::` solo per il seed iniziale in Loading)

---

## 📊 Generation Details

### Exit Placement Example (Floor 3)

```
candidate_positions = [
    (1, 1), (1, 6),  // top-left, top-right (borders)
    (6, 1), (6, 6),  // bottom-left, bottom-right
    (1, 3), (1, 4),  // mid-left
    (6, 3), (6, 4),  // mid-right
]
index = sub_seed(seed, 3, 0) % 8 = (seed*mult + 3) % 8 = 5
exit = candidate[5] = (1, 3)
```

Risultato: exit varia ogni floor, non è prevedibile.

### Pillar Placement Example (Floor 1)

```
candidates = [
    (2,2), (3,2), (4,2),
    (2,3), (3,3), (4,3),
    (2,4), (3,4), (4,4),
]

for i, pos in candidates {
    if sub_seed(seed, 1, 50+i) % 3 != 0:
        place pillar at pos
}

Result: ~6 pilastri piazzati (66% chance per candidate)
Connectivity check: se path (1,1)→exit disconnected, remove last pillar, retry
```

### Enemy Count & Force (Floor 3)

```
count = 2 + (sub_seed(seed, 3, 10) % 3) = 2 + 1 = 3 nemici
base_force = 2 + 3 = 5

enemy_0:
  pos = shuffle_positions[0]  // primo della lista shuffled
  force = 5 + (sub_seed(seed, 3, 100) % 5) = 5 + 2 = 7
  
enemy_1:
  pos = shuffle_positions[1]
  force = 5 + (sub_seed(seed, 3, 101) % 5) = 5 + 4 = 9
  
enemy_2:
  pos = shuffle_positions[2]
  force = 5 + (sub_seed(seed, 3, 102) % 5) = 5 + 1 = 6
```

---

## 🧪 Testing Plan

1. **Determinismo**: Start same seed twice, verifica esatto layout (tile-by-tile, enemy position-by-position).
2. **Varianza**: Start game 5 volte, verifica 5 layout diversi.
3. **Connectivity**: 20 floor completati senza "trapped in room".
4. **Seed display**: Muori, leggi seed su dead screen, ricopia in test, ricomincia, verifica identico layout.
5. **Edge cases**: 
   - Pillar generation non blocca path da spawn a exit
   - Enemy count scala correttamente con floor (floor 10+ ha più nemici)
   - Forza scala (nemici diventano più forti su floor alti)

---

## 📝 Note

- **Shuffle algorithm**: Fisher-Yates manuale usando `sub_seed` come state, oppure usa `SmallRng::seed_from_u64()` da `rand` per semplicità (meno breaking che scrivere PRNG da zero).
- **Pillar chance**: 66% per candidato genera ~6 pilastri su 9. Se prefisci meno pillars, usa `% 4 != 0` (75% skip).
- **Exit always walkable**: Assicurati che exit_pos non sia una wall tile o pillar.
- **BFS early termination**: Implementa BFS che esce appena trova exit (non flood all'intera mappa).

---

*Versione: 1.0 | Creato: 2026-04-28*
