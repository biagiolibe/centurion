# CEN-009 — Risoluzione Combattimento Deterministico

## Obiettivo
Implementare la logica di combattimento deterministica pura: `Player.Force - Enemy.Force = NewPlayerForce`. Se ≤0 il giocatore muore, se >0 il nemico è rimosso e il giocatore si muove.

## Dipendenze
- CEN-006
- CEN-008

## Componenti / Risorse / Sistemi da Creare

### Plugin
- `ResolverPlugin`

### Systems
- `resolve_combat`: reads `CombatIntent` events, risolve il combattimento, aggiorna stato del giocatore o transiziona a `Dead`

### Functions
- `fn resolve(player_force: i32, enemy_force: i32) -> CombatResult` — pura, unit-testabile

### Enum
```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CombatResult {
    PlayerWins { new_force: i32 },
    PlayerDies,
}
```

## File da Creare / Modificare
- `src/resolver/mod.rs` — plugin e re-export (nuovo)
- `src/resolver/combat.rs` — logica combattimento (nuovo)
- `src/plugins/mod.rs` — aggiungere `ResolverPlugin`

## Dettagli Implementativi

### Pure Resolution Function
```rust
pub fn resolve(player_force: i32, enemy_force: i32) -> CombatResult {
    let new_force = player_force - enemy_force;
    if new_force <= 0 {
        CombatResult::PlayerDies
    } else {
        CombatResult::PlayerWins { new_force }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_player_wins() {
        assert_eq!(resolve(5, 3), CombatResult::PlayerWins { new_force: 2 });
    }
    
    #[test]
    fn test_player_dies_negative() {
        assert_eq!(resolve(5, 7), CombatResult::PlayerDies);
    }
    
    #[test]
    fn test_player_dies_zero() {
        assert_eq!(resolve(5, 5), CombatResult::PlayerDies);
    }
}
```

### Combat Resolution System
```rust
fn resolve_combat(
    mut commands: Commands,
    mut combat_events: EventReader<CombatIntent>,
    mut player_query: Query<(&mut Force, &mut GridPos), With<Player>>,
    enemy_query: Query<&EnemyForce, With<Enemy>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in combat_events.read() {
        let Ok((mut player_force, mut player_pos)) = player_query.get_mut(event.attacker) else {
            continue;
        };
        
        let Ok(enemy_force) = enemy_query.get(event.defender) else {
            continue;
        };
        
        let result = resolve(player_force.0, enemy_force.0);
        
        match result {
            CombatResult::PlayerWins { new_force } => {
                player_force.0 = new_force;
                // Despawn nemico
                commands.entity(event.defender).despawn();
                // Muovi il player
                if let Ok(enemy_pos) = enemy_query.get(event.defender) {
                    // Recupera la posizione del nemico prima del despawn
                    // NOTA: questo è un problema di ordering. Vedere sotto.
                }
                // Transiziona a CombatEvent per il flash
                next_state.set(GameState::CombatEvent);
            }
            CombatResult::PlayerDies => {
                next_state.set(GameState::Dead);
            }
        }
    }
}
```

### Nota su Enemy Position Recovery
Il problema è che il nemico sarà despawnato prima che possiamo leggere la sua posizione. Soluzione: nel `CombatIntent` event, aggiungere direttamente la posizione target:

```rust
#[derive(Event)]
pub struct CombatIntent {
    pub attacker: Entity,
    pub defender: Entity,
    pub target_pos: GridPos,  // NEW: posizione dove il giocatore finirà
}
```

Oppure, guardare il nemico PRIMA di despawnarlo:

```rust
let Ok(enemy_force) = enemy_query.get(event.defender) else { continue; };
let enemy_pos = /* query GridPos del nemico */;

let result = resolve(player_force.0, enemy_force.0);
match result {
    CombatResult::PlayerWins { new_force } => {
        player_force.0 = new_force;
        player_pos.x = enemy_pos.x;
        player_pos.y = enemy_pos.y;
        player_query.update_transform(); // update visual transform dopo pos change
        commands.entity(event.defender).despawn();
        next_state.set(GameState::CombatEvent);
    }
    CombatResult::PlayerDies => {
        next_state.set(GameState::Dead);
    }
}
```

### Transizione a CombatEvent
Dopo la risoluzione, transiziona sempre a `GameState::CombatEvent` (non direttamente a `Room`). CEN-010 gestirà il timing e il ritorno a `Room`.

## Criteri di Accettazione
- [ ] Player Force 5 vs Enemy Force 3 → Player vince, Force diventa 2
- [ ] Player Force 5 vs Enemy Force 7 → Player muore, transizione a `Dead`
- [ ] Player Force 5 vs Enemy Force 5 → Player muore (damage = 0, letale)
- [ ] `resolve()` ha unit test per tutti i casi
- [ ] Nemico è despawnato dopo vittoria
- [ ] Player GridPos aggiornato alla posizione del nemico dopo vittoria
- [ ] Player visuale (Transform) aggiornato dopo movimento
- [ ] Transizione a `CombatEvent` per il flash (CEN-010)
