# CEN-010 — Animazione Flash (bevy_tweening)

## Obiettivo
Implementare un breve flash di colore (white-to-red on enemy defeat, white-to-yellow then back on victory) usando `bevy_tweening`, sincronizzato con lo stato `CombatEvent`.

## Dipendenze
- CEN-009

## Componenti / Risorse / Sistemi da Creare

### Plugin
- `FlashPlugin` (parte di `ResolverPlugin`)

### Systems
- `start_combat_flash`: runs `OnEnter(GameState::CombatEvent)`, inserts `Animator` sul player
- `finish_combat_flash`: runs `Update` `in_state(GameState::CombatEvent)`, checks completion e transiziona

## File da Creare / Modificare
- `src/resolver/flash.rs` — flash animation logic (nuovo)
- `src/resolver/mod.rs` — includere `FlashPlugin`

## Dettagli Implementativi

### Flash Parameters
- Duration: 200ms
- Easing: `EaseFunction::Linear`
- Target: `Sprite::color` component

### Flash Colors Based on Outcome
Come sapere se il player ha vinto o perso? Abbiamo bisogno di propagare l'informazione dello stato di battaglia. Opzione:
1. Aggiungere una resource `LastCombatOutcome` che viene scritta da `resolve_combat` e letta da `start_combat_flash`
2. Oppure, usare un event `CombatResolved { result: CombatResult }` emesso insieme a `CombatIntent`

Scegliamo l'opzione 2 per mantenere il flusso event-driven.

### Resource per Outcome
```rust
#[derive(Resource, Default, Clone, Copy)]
pub enum LastCombatOutcome {
    #[default]
    None,
    Victory,
    Defeat,
}
```

### Tween Color Sequences
**Victory Flash** (white → yellow → white, 200ms):
```rust
let tween = Sequence::new([
    Tween::new(
        EaseFunction::Linear,
        Duration::from_millis(100),
        ColorLens {
            start: Color::WHITE,
            end: Color::rgb(1.0, 1.0, 0.0),
        },
    ),
    Tween::new(
        EaseFunction::Linear,
        Duration::from_millis(100),
        ColorLens {
            start: Color::rgb(1.0, 1.0, 0.0),
            end: Color::WHITE,
        },
    ),
]);
```

**Defeat Flash** (white → red, 200ms, no return — direct to `Dead`):
```rust
let tween = Tween::new(
    EaseFunction::Linear,
    Duration::from_millis(200),
    ColorLens {
        start: Color::WHITE,
        end: Color::rgb(1.0, 0.0, 0.0),
    },
);
```

### System Implementation
```rust
fn start_combat_flash(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    outcome: Res<LastCombatOutcome>,
) {
    let Ok(player_entity) = player_query.get_single() else {
        return;
    };
    
    let tween = match *outcome {
        LastCombatOutcome::Victory => {
            // Sequence: white → yellow → white
            Sequence::new([...])
        }
        LastCombatOutcome::Defeat => {
            // Tween: white → red
            Tween::new(...)
        }
        LastCombatOutcome::None => return,
    };
    
    let animator = Animator::new(tween);
    commands.entity(player_entity).insert(animator);
}

fn finish_combat_flash(
    mut commands: Commands,
    player_query: Query<&Animator, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
    outcome: Res<LastCombatOutcome>,
) {
    if let Ok(animator) = player_query.get_single() {
        if animator.is_finished() {
            match *outcome {
                LastCombatOutcome::Victory => {
                    next_state.set(GameState::Room);
                }
                LastCombatOutcome::Defeat => {
                    next_state.set(GameState::Dead);
                }
                LastCombatOutcome::None => {}
            }
            commands.entity(player).remove::<Animator>();
        }
    }
}
```

### Integration with Combat System
In `resolve_combat` (CEN-009), dopo aver risolto il combattimento:
1. Scrivere l'outcome nella resource `LastCombatOutcome`
2. Transizionare a `CombatEvent`

```rust
match result {
    CombatResult::PlayerWins { new_force } => {
        last_outcome.0 = LastCombatOutcome::Victory;
        next_state.set(GameState::CombatEvent);
    }
    CombatResult::PlayerDies => {
        last_outcome.0 = LastCombatOutcome::Defeat;
        next_state.set(GameState::CombatEvent);
    }
}
```

## Criteri di Accettazione
- [ ] CombatEvent state dura almeno 200ms (il tempo della tween)
- [ ] Vittoria flash: player sprite diventa giallo per 100ms, poi bianco
- [ ] Sconfitta flash: player sprite diventa rosso per 200ms
- [ ] Flash completato → automaticat transizione a Room (vittoria) o Dead (sconfitta)
- [ ] Nessun lag / stuttering durante la tween
- [ ] Animator è rimosso dal player dopo il completamento
