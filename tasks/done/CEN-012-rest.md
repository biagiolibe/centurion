# CEN-012 — Schermata Rest

## Obiettivo
Mostrare uno schermo di pausa tra i piani, visualizzando statistiche e attendendo l'input SPACE per continuare al piano successivo.

## Dipendenze
- CEN-011
- CEN-002

## Componenti / Risorse / Sistemi da Creare

### Components
```rust
#[derive(Component)]
pub struct RestScreenRoot;

#[derive(Component)]
pub struct RestText;
```

### Plugin
- `RestScreenPlugin` (parte di `HudPlugin`)

### Systems
- `spawn_rest_screen`: runs `OnEnter(GameState::Rest)`, crea il testo
- `rest_input`: runs `Update` `in_state(GameState::Rest)`, legge SPACE, transiziona

## File da Creare / Modificare
- `src/ui/rest_screen.rs` — rest screen logic (nuovo)
- `src/ui/mod.rs` — aggiungere il plugin
- `src/plugins/mod.rs` — se necessario

## Dettagli Implementativi

### Rest Screen Layout
```
FLOOR [N] CLEARED
STEPS REMAINING: [X]
FORCE: [Y]

Press SPACE to descend
```

Centered on screen, white text, large font (32px per il titolo, 24px per il resto).

### Spawn Logic
```rust
fn spawn_rest_screen(
    mut commands: Commands,
    config: Res<CenturionConfig>,
    persistence: Res<PlayerPersistence>,
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
        RestScreenRoot,
        StateScoped(GameState::Rest),
    )).id();
    
    commands.entity(root).with_children(|parent| {
        // Title: FLOOR [N] CLEARED
        parent.spawn((
            Text::new(format!("FLOOR {} CLEARED", config.current_floor - 1)),
            TextFont { font_size: 32.0, ..default() },
            TextColor(Color::WHITE),
            RestText,
        ));
        
        // Stats
        parent.spawn((
            Text::new(format!("STEPS REMAINING: {}", persistence.steps)),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::WHITE),
            RestText,
        ));
        
        parent.spawn((
            Text::new(format!("FORCE: {}", persistence.force)),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::WHITE),
            RestText,
        ));
        
        // Prompt
        parent.spawn((
            Text::new("Press SPACE to descend"),
            TextFont { font_size: 20.0, ..default() },
            TextColor(Color::rgb(0.7, 0.7, 0.7)),
            RestText,
        ));
    });
}
```

### Input Handling
```rust
fn rest_input(
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Room);
    }
}
```

### Transizione a Room
Quando si transiziona da `Rest` a `Room`:
1. `StateScoped(Rest)` despawna il rest screen
2. `OnEnter(Room)` triggerizza `spawn_room` e `spawn_player`
3. `spawn_player` legge `PlayerPersistence` e restora steps/force

## Criteri di Accettazione
- [ ] Rest screen appare centrato al cambio a `Rest` state
- [ ] Floor number è `current_floor - 1` (il piano che è stato appena completato)
- [ ] Steps e Force mostrano i valori corretti da `PlayerPersistence`
- [ ] Premere SPACE transiziona a `Room`
- [ ] Il nuovo piano ha nemici di difficoltà corretta
- [ ] Player re-entra con steps e force corretti
- [ ] Rest screen entities sono despawnate quando si torna a Room
