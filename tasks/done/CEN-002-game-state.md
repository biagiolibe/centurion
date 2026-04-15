# CEN-002 — Game State Machine (`GameState`)

## Obiettivo
Implementare una state machine Bevy che governa il flusso: Loading → Room → CombatEvent → Rest → Dead.

## Dipendenze
- CEN-001

## Componenti / Risorse / Sistemi da Creare

### State Enum
```rust
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Loading,
    Room,
    CombatEvent,
    Rest,
    Dead,
}
```

### Plugin
- `StatePlugin`: registra la state machine e gestisce le transizioni

### Systems
- `loading_to_room` (runs `OnEnter(GameState::Loading)` inizialmente, poi immediatamente transiziona a `Room`): emette `NextState::<GameState>::Room`

## File da Creare / Modificare
- `src/state.rs` — `GameState` enum (nuovo)
- `src/plugins/state_plugin.rs` — `StatePlugin` (nuovo)
- `src/plugins/mod.rs` — aggiungere `StatePlugin` al `CenturionPlugins`
- `src/main.rs` — aggiungere `.init_state::<GameState>()`

## Dettagli Implementativi

### Valid State Transitions
```
Loading  → Room
Room     → CombatEvent | Rest | Dead
CombatEvent → Room
Rest     → Room
Dead     → Loading
```

Documentare nel codice ogni transizione valida con un commento.

### `StateScoped<GameState>` Pattern
Documentare (in un commento vicino a `GameState` enum) che tutte le entità in-game dovrebbero portare `StateScoped<GameState>` come component, così che:
- Entità in `Room` avranno `StateScoped(GameState::Room)`
- Entità in `Rest` avranno `StateScoped(GameState::Rest)`
- Al cambio stato, Bevy despawna automaticamente tutte le entità con quel marker

Esempio:
```rust
// PATTERN: Usa StateScoped<GameState> su tutte le entità in-game
// Quando si esce da uno stato, Bevy despawna automaticamente tutte le entità
// che portano StateScoped(old_state)
commands.spawn((
    MyComponent,
    StateScoped(GameState::Room),
));
```

## Criteri di Accettazione
- [ ] App lancia in stato `Loading`
- [ ] Entro uno frame, transiziona automaticamente a `Room` senza input
- [ ] Un system con `in_state(GameState::Room)` esegue solo quando in `Room`
- [ ] Un system con `in_state(GameState::CombatEvent)` esegue solo in `CombatEvent`
- [ ] Aggiungere un system di debug che logga lo stato attuale ogni frame (rimovibile dopo)
- [ ] Nessun panic durante cambio stato
