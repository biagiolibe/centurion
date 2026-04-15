# CEN-011 — Interazione Uscita e Progressione Piano

## Obiettivo
Rilevare quando il player raggiunge la tile Exit, salvare i valori persistent (steps e forza), incrementare il piano, e transizionare a `Rest`.

## Dipendenze
- CEN-006
- CEN-009

## Componenti / Risorse / Sistemi da Creare

### Resource
```rust
#[derive(Resource, Clone, Copy)]
pub struct PlayerPersistence {
    pub steps: i32,
    pub force: i32,
}
```

### Plugin
- `ExitPlugin` (parte di `TacticsPlugin`)

### Systems
- `check_exit`: runs `Update` `in_state(GameState::Room)`, dopo `apply_movement`
- `save_player_state`: runs `OnExit(GameState::Room)`, salva steps/force in `PlayerPersistence`

## File da Creare / Modificare
- `src/tactics/mod.rs` — aggiungere sistema e resource
- `src/plugins/mod.rs` — se `ExitPlugin` è separato

## Dettagli Implementativi

### Exit Detection
```rust
fn check_exit(
    player_query: Query<&GridPos, With<Player>>,
    room_layout: Res<RoomLayout>,
    mut next_state: ResMut<NextState<GameState>>,
    mut config: ResMut<CenturionConfig>,
) {
    if let Ok(player_pos) = player_query.get_single() {
        if room_layout.get(player_pos.x, player_pos.y) == TileKind::Exit {
            config.current_floor += 1;
            next_state.set(GameState::Rest);
        }
    }
}
```

### State Saving
```rust
fn save_player_state(
    player_query: Query<(&CurrentSteps, &Force), With<Player>>,
    mut persistence: ResMut<PlayerPersistence>,
) {
    if let Ok((current_steps, force)) = player_query.get_single() {
        persistence.steps = current_steps.0;
        persistence.force = force.0;
    }
}
```

### Difficulty Scaling
La difficoltà dei nemici scala già in `enemy_spawn_config(floor)` (CEN-008):
```rust
pub fn enemy_spawn_config(floor: u8) -> EnemySpawnConfig {
    // Floor 1: forces 3, 7, 4
    // Floor 2+: base_force = 2 + floor, aggiungere offset
    let base_force = 2 + floor as i32;
    // ...
}
```

Verificare che il valore scale visibilmente (es. floor 1 max force = 7, floor 2 max force = 10).

### PlayerPersistence Restore
Quando il player re-entra in `Room` dopo Rest (CEN-012), il sistema di spawn di player dovrà consultare `PlayerPersistence` per i valori iniziali:

```rust
fn spawn_player(
    mut commands: Commands,
    persistence: Res<PlayerPersistence>,
) {
    let (start_steps, start_force) = if persistence.steps > 0 {
        (persistence.steps, persistence.force)
    } else {
        (100, 5)  // Valori default se non è stata salvata nulla
    };
    
    // Spawn con questi valori
}
```

## Criteri di Accettazione
- [ ] Stepping su Exit tile transiziona a `Rest`
- [ ] `CenturionConfig.current_floor` incrementa da 1 a 2
- [ ] `PlayerPersistence` contiene steps e force corretti dopo uscita Room
- [ ] Floor 2 nemici hanno force visibilmente più alta
- [ ] PlayerPersistence viene resettata a (100, 5) quando si avvia una nuova run (CEN-013)
- [ ] Nessun step/force perso durante il cambio piano
