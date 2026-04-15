# CEN-004 — Entità Giocatore: Componenti e Spawn

## Obiettivo
Definire i componenti del giocatore (posizione, passi, forza) e spawnarlo nel centro della stanza al fondo dello stato `Room`.

## Dipendenze
- CEN-001
- CEN-002
- CEN-003

## Componenti / Risorse / Sistemi da Creare

### Components
```rust
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct CurrentSteps(pub i32);

#[derive(Component)]
pub struct Force(pub i32);
```

### Bundle
```rust
#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub current_steps: CurrentSteps,
    pub force: Force,
    pub grid_pos: GridPos,
    pub sprite: Sprite,
    pub transform: Transform,
    pub state_scoped: StateScoped<GameState::Room>,
}
```

### Plugin
- `PlayerPlugin`

### Systems
- `spawn_player`: runs `OnEnter(GameState::Room)`. Spawna il giocatore a `GridPos { x: 1, y: 1 }`

## File da Creare / Modificare
- `src/player/mod.rs` — plugin e re-export (nuovo)
- `src/player/components.rs` — definizione componenti (nuovo)
- `src/plugins/mod.rs` — aggiungere `PlayerPlugin`

## Dettagli Implementativi

### Player Initial Values
- `GridPos`: `{ x: 1, y: 1 }`
- `CurrentSteps`: `100`
- `Force`: `5`

### Player Visual
- Sprite: cerchio (radius ~28px, quindi size ~56px come quadrato bounding box)
- Colore: bianco (`Color::WHITE`)
- Generato con `spawn_circle()` da CEN-001

### Spawn Logic
```rust
fn spawn_player(
    mut commands: Commands,
    config: Res<CenturionConfig>,
) {
    let start_pos = GridPos { x: 1, y: 1 };
    let world_pos = grid_to_world(start_pos);
    
    commands.spawn(PlayerBundle {
        player: Player,
        current_steps: CurrentSteps(100),
        force: Force(5),
        grid_pos: start_pos,
        sprite: Sprite { color: Color::WHITE, ..default() },
        transform: Transform::from_translation(world_pos.extend(0.0)),
        state_scoped: StateScoped(GameState::Room),
    });
}
```

## Criteri di Accettazione
- [ ] Player spawna al grid position (1, 1) all'entrata di `Room`
- [ ] Player circle è visibile in bianco
- [ ] `CurrentSteps` inizia a 100
- [ ] `Force` inizia a 5
- [ ] Player entity è despawnato quando si esce da `Room` (grazie a `StateScoped`)
- [ ] Due entità player non vengono create (spawn idempotente)
