# CEN-013 — Schermata Dead e Statistiche Run

## Obiettivo
Mostrare una schermata finale con statistiche della run (piani chiariti, passi totali, causa della morte), e permettere il restart con `R`.

## Dipendenze
- CEN-009
- CEN-011
- CEN-012

## Componenti / Risorse / Sistemi da Creare

### Enums
```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeathCause {
    OutOfSteps,
    KilledByEnemy { enemy_force: i32 },
}
```

### Resource
```rust
#[derive(Resource, Clone)]
pub struct RunStats {
    pub floors_cleared: u8,
    pub steps_taken: i32,
    pub steps_remaining: i32,
    pub cause: DeathCause,
}
```

### Components
```rust
#[derive(Component)]
pub struct DeadScreenRoot;

#[derive(Component)]
pub struct DeadText;
```

### Plugin
- `DeadScreenPlugin` (parte di `HudPlugin`)

### Systems
- `populate_run_stats`: runs `OnEnter(GameState::Dead)`, calcola le statistiche
- `spawn_dead_screen`: runs `OnEnter(GameState::Dead)`, mostra il testo
- `dead_input`: runs `Update` `in_state(GameState::Dead)`, legge R per restart

## File da Creare / Modificare
- `src/ui/dead_screen.rs` — dead screen logic (nuovo)
- `src/ui/mod.rs` — aggiungere il plugin
- `src/plugins/mod.rs` — se necessario

## Dettagli Implementativi

### RunStats Population
```rust
fn populate_run_stats(
    mut commands: Commands,
    config: Res<CenturionConfig>,
    persistence: Res<PlayerPersistence>,
    last_outcome: Res<LastCombatOutcome>,
) {
    // floors_cleared è il piano precedente (current_floor - 1)
    let floors_cleared = if config.current_floor > 1 {
        config.current_floor - 1
    } else {
        0
    };
    
    let cause = match *last_outcome {
        LastCombatOutcome::Defeat => DeathCause::KilledByEnemy { enemy_force: 0 },
        // Come determinare l'enemy force? Potremmo:
        // 1. Salvare l'ultimo nemico incontrato in una resource
        // 2. Loggare sempre il valore nel combattimento
        // Per MVP, semplificare: se steps > 0 allora out of steps, altrimenti killed by unknown enemy
        _ => DeathCause::OutOfSteps,
    };
    
    let (cause, steps_taken) = if persistence.steps <= 0 {
        (DeathCause::OutOfSteps, 100 - persistence.steps)
    } else {
        (DeathCause::KilledByEnemy { enemy_force: 0 }, 100 - persistence.steps)
    };
    
    let stats = RunStats {
        floors_cleared,
        steps_taken,
        steps_remaining: persistence.steps,
        cause,
    };
    
    commands.insert_resource(stats);
}
```

### Dead Screen Layout
```
CENTURION FALLS

Floors cleared: [N]
Steps taken: [X]
Steps remaining: [Y]
Cause: [Out of steps | Killed by Force N enemy]

Press R to restart
```

Titolo in rosso grande (36px), statistiche in bianco (24px), prompt in grigio (20px).

### Spawn Logic
```rust
fn spawn_dead_screen(
    mut commands: Commands,
    stats: Res<RunStats>,
) {
    let root = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        DeadScreenRoot,
        StateScoped(GameState::Dead),
    )).id();
    
    commands.entity(root).with_children(|parent| {
        // Title in red
        parent.spawn((
            Text::new("CENTURION FALLS"),
            TextFont { font_size: 36.0, ..default() },
            TextColor(Color::rgb(1.0, 0.0, 0.0)),
            DeadText,
        ));
        
        // Floors cleared
        parent.spawn((
            Text::new(format!("Floors cleared: {}", stats.floors_cleared)),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::WHITE),
            DeadText,
        ));
        
        // Steps taken
        parent.spawn((
            Text::new(format!("Steps taken: {}", stats.steps_taken)),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::WHITE),
            DeadText,
        ));
        
        // Steps remaining
        parent.spawn((
            Text::new(format!("Steps remaining: {}", stats.steps_remaining)),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::WHITE),
            DeadText,
        ));
        
        // Cause
        let cause_text = match stats.cause {
            DeathCause::OutOfSteps => "Cause: Out of steps".to_string(),
            DeathCause::KilledByEnemy { enemy_force } => {
                format!("Cause: Killed by Force {} enemy", enemy_force)
            }
        };
        parent.spawn((
            Text::new(cause_text),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::WHITE),
            DeadText,
        ));
        
        // Restart prompt
        parent.spawn((
            Text::new("Press R to restart"),
            TextFont { font_size: 20.0, ..default() },
            TextColor(Color::rgb(0.7, 0.7, 0.7)),
            DeadText,
        ));
    });
}
```

### Restart Logic
```rust
fn dead_input(
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut config: ResMut<CenturionConfig>,
    mut persistence: ResMut<PlayerPersistence>,
    mut stats: ResMut<RunStats>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        // Reset global state
        config.current_floor = 1;
        persistence.steps = 100;
        persistence.force = 5;
        // Potrebbe essere utile resettare anche LastCombatOutcome, ma onExit(Dead) potrebbe farlo
        
        next_state.set(GameState::Loading);
    }
}
```

### Reset Pattern
Quando si transiziona da `Dead` a `Loading`:
1. `OnExit(Dead)` despawna tutte le entità `StateScoped(Dead)`
2. `OnEnter(Loading)` immediatamente transiziona a `Room`
3. `OnEnter(Room)` spawna la nuova room e player con valori di default da `PlayerPersistence`

Assicurare che nessuna entità dalla run precedente rimanga (zombie HUD, nemici, tile).

## Criteri di Accettazione
- [ ] Dead screen appare al cambio a `Dead` state
- [ ] Titolo "CENTURION FALLS" è in rosso
- [ ] Floors cleared mostra il numero corretto
- [ ] Steps taken = 100 - steps_remaining
- [ ] "Out of steps" appare se steps esauriti
- [ ] "Killed by Force X" appare se ucciso da nemico (bonus per MVP)
- [ ] Premere R transiziona a `Loading`
- [ ] Nuovo run inizia con floor 1, steps 100, force 5
- [ ] Nessuna entità dalla run precedente rimane
- [ ] Si può fare multiple run senza crash
