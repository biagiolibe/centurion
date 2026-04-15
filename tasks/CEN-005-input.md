# CEN-005 — Input System con leafwing-input-manager

## Obiettivo
Configurare il sistema di input usando `leafwing-input-manager` per mappare tasti (WASD e frecce) a 4 azioni cardinali, ed emettere eventi `MoveIntent` quando l'input è rilevato.

## Dipendenze
- CEN-004

## Componenti / Risorse / Sistemi da Creare

### Enum
```rust
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerAction {
    MoveNorth,
    MoveSouth,
    MoveEast,
    MoveWest,
}
```

### Event
```rust
#[derive(Event)]
pub struct MoveIntent {
    pub direction: IVec2,
}
```

### Plugin
- `InputPlugin`

### Systems
- `read_player_input`: reads `ActionState<PlayerAction>`, emits `MoveIntent` events

## File da Creare / Modificare
- `src/input.rs` (nuovo)
- `src/plugins/mod.rs` — aggiungere `InputPlugin`

## Dettagli Implementativi

### Input Bindings
```rust
fn input_map() -> InputMap<PlayerAction> {
    InputMap::new([
        (KeyCode::ArrowUp, PlayerAction::MoveNorth),
        (KeyCode::ArrowDown, PlayerAction::MoveSouth),
        (KeyCode::ArrowLeft, PlayerAction::MoveWest),
        (KeyCode::ArrowRight, PlayerAction::MoveEast),
        (KeyCode::KeyW, PlayerAction::MoveNorth),
        (KeyCode::KeyS, PlayerAction::MoveSouth),
        (KeyCode::KeyA, PlayerAction::MoveWest),
        (KeyCode::KeyD, PlayerAction::MoveEast),
    ])
}
```

### Direction Mapping
```rust
fn action_to_direction(action: PlayerAction) -> IVec2 {
    match action {
        PlayerAction::MoveNorth => IVec2::new(0, -1),
        PlayerAction::MoveSouth => IVec2::new(0, 1),
        PlayerAction::MoveEast => IVec2::new(1, 0),
        PlayerAction::MoveWest => IVec2::new(-1, 0),
    }
}
```

### System Logic
```rust
fn read_player_input(
    query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut move_events: EventWriter<MoveIntent>,
) {
    for action_state in query.iter() {
        if action_state.just_pressed(&PlayerAction::MoveNorth) {
            move_events.send(MoveIntent { direction: action_to_direction(PlayerAction::MoveNorth) });
        }
        // ripeti per le altre 3 azioni
    }
}
```

Nota: usare `just_pressed` non `pressed` per evitare repeat rate — un tasto premuto = un movimento.

### InputManagerBundle Setup
La spawn di player (CEN-004) deve aggiungere:
```rust
InputManagerBundle::<PlayerAction> {
    input_map: input_map(),
    ..default()
}
```

al `PlayerBundle`.

## Criteri di Accettazione
- [ ] Premere una freccia o tasto WASD emette un evento `MoveIntent` (verificare con un debug system)
- [ ] Solo una azione per keypress (no repeat, no diagonal)
- [ ] Ogni tasto mappa esattamente a una direzione cardinale
- [ ] Il movimento non avviene ancora — solo gli eventi sono emessi
- [ ] Rilasciare il tasto non emette evento
