# CEN-008 — Entità Nemici: Spawn e Comportamento Statico

## Obiettivo
Definire componenti nemici e spawnare nemici statici in posizioni fisse per ogni piano, con visuale distintiva.

## Dipendenze
- CEN-003
- CEN-004

## Componenti / Risorse / Sistemi da Creare

### Components
```rust
#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct EnemyForce(pub i32);

#[derive(Component)]
pub enum EnemyBehavior {
    Static,
}
```

### Bundle
```rust
#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub enemy_force: EnemyForce,
    pub behavior: EnemyBehavior,
    pub grid_pos: GridPos,
    pub sprite: Sprite,
    pub transform: Transform,
    pub state_scoped: StateScoped<GameState::Room>,
}
```

### Plugin
- `EnemiesPlugin`

### Systems
- `spawn_enemies`: runs `OnEnter(GameState::Room)`, spawna nemici secondo `EnemySpawnConfig`

### Resource
```rust
#[derive(Resource)]
pub struct EnemySpawnConfig {
    pub enemies: Vec<(GridPos, i32)>,  // (position, force)
}
```

## File da Creare / Modificare
- `src/enemies/mod.rs` — plugin e re-export (nuovo)
- `src/enemies/components.rs` — componenti (nuovo)
- `src/plugins/mod.rs` — aggiungere `EnemiesPlugin`

## Dettagli Implementativi

### Enemy Visual
- Sprite: quadrato (64x64) grigio scuro (es. `Color::rgb(0.7, 0.3, 0.3)`)
- Inner indicator (facoltativo per MVP ma consigliato): un quadrato più piccolo interno per indicare il livello di forza
- Dimensione inner square ~ forza / 2 pixel (es. forza 4 → 2px inner square)

### Floor 1 Spawn Config
```rust
pub fn enemy_spawn_config(floor: u8) -> EnemySpawnConfig {
    let mut enemies = Vec::new();
    
    match floor {
        1 => {
            enemies.push((GridPos { x: 3, y: 3 }, 3));
            enemies.push((GridPos { x: 5, y: 5 }, 7));
            enemies.push((GridPos { x: 6, y: 3 }, 4));
        }
        2 => {
            // Difficoltà scalata: forza base = 2 + floor
            enemies.push((GridPos { x: 2, y: 2 }, 4));
            enemies.push((GridPos { x: 5, y: 5 }, 9));
            enemies.push((GridPos { x: 6, y: 3 }, 6));
            enemies.push((GridPos { x: 3, y: 6 }, 5));
        }
        _ => {
            // Generativamente per floor > 2
            let base_force = 2 + floor as i32;
            enemies.push((GridPos { x: 2, y: 2 }, base_force + 1));
            enemies.push((GridPos { x: 5, y: 5 }, base_force + 4));
            enemies.push((GridPos { x: 6, y: 3 }, base_force + 2));
        }
    }
    
    EnemySpawnConfig { enemies }
}
```

### Spawn Validation
Prima di spawnare un nemico, controllare che:
1. Posizione è una tile `Floor` o `Exit` (non `Wall`)
2. Posizione NON è il player spawn (1, 1)
3. Posizione NON è l'exit tile

Se una configurazione viola questi vincoli, loggare un warning e skippare quel nemico.

### Spawn Logic
```rust
fn spawn_enemies(
    mut commands: Commands,
    config: Res<CenturionConfig>,
    room_layout: Res<RoomLayout>,
) {
    let enemy_config = enemy_spawn_config(config.current_floor);
    
    for (grid_pos, force) in enemy_config.enemies {
        let world_pos = grid_to_world(grid_pos);
        
        commands.spawn(EnemyBundle {
            enemy: Enemy,
            enemy_force: EnemyForce(force),
            behavior: EnemyBehavior::Static,
            grid_pos,
            sprite: Sprite {
                color: Color::rgb(0.7, 0.3, 0.3),
                ..default()
            },
            transform: Transform::from_translation(world_pos.extend(0.0)),
            state_scoped: StateScoped(GameState::Room),
        });
    }
}
```

## Criteri di Accettazione
- [ ] Floor 1 spawna 3 nemici alle posizioni fisse (3,3), (5,5), (6,3)
- [ ] Nessun nemico spawna su (1,1) o sull'exit tile
- [ ] Nemici sono visivamente distinti dal player e dalle tile
- [ ] Nemici hanno il valore di forza corretto (3, 7, 4 per floor 1)
- [ ] Floor 2 nemici hanno forza più alta
- [ ] Tutti i nemici despawnano all'uscita da `Room`
- [ ] Nessun nemico sovrapposto
