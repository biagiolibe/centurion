# CEN-006 — Movimento su Griglia e Consumo Passi

## Obiettivo
Implementare la validazione del movimento sulla griglia, consumo di `CurrentSteps`, transizione a morte se esauriti, e emissione di eventi `CombatIntent` per tile occupate da nemici.

## Dipendenze
- CEN-003
- CEN-004
- CEN-005

## Componenti / Risorse / Sistemi da Creare

### Events
```rust
#[derive(Event)]
pub struct CombatIntent {
    pub attacker: Entity,
    pub defender: Entity,
}
```

### Plugin
- `TacticsPlugin`

### Systems
- `apply_movement`: reads `MoveIntent` events, validates, moves player, consumes steps, emits `CombatIntent`

### Functions
- `fn can_move_to(pos: GridPos, layout: &RoomLayout) -> bool` — ritorna false se muro

## File da Creare / Modificare
- `src/tactics/mod.rs` — plugin e re-export (nuovo)
- `src/tactics/movement.rs` — logica movimento (nuovo)
- `src/plugins/mod.rs` — aggiungere `TacticsPlugin`

## Dettagli Implementativi

### Validation
```rust
fn can_move_to(pos: GridPos, layout: &RoomLayout) -> bool {
    let tile = layout.get(pos.x, pos.y);
    tile != TileKind::Wall
}
```

### Movement Resolution
Quando viene letto un `MoveIntent`:
1. Calcola il nuovo GridPos: `new_pos = current_pos + direction.as_ivec2()` (con bounds check 0..8)
2. Se `new_pos` è fuori bounds, ignora l'evento
3. Queryare `RoomLayout` e controllare `can_move_to(new_pos)`
4. Se non è possibile muoversi, ignora senza consumare step
5. Se è possibile:
   - Queryare se c'è un nemico a `new_pos`
   - Se NON c'è nemico:
     - Aggiorna `GridPos` del giocatore
     - Aggiorna `Transform` usando `grid_to_world()`
     - Decrementa `CurrentSteps` di 1
   - Se c'è un nemico:
     - Emetti `CombatIntent { attacker: player_entity, defender: enemy_entity }`
     - NON fare il movimento ancora — il combat resolver lo farà

### Step Exhaustion
Dopo il decremento:
```rust
if player_steps.0 <= 0 {
    next_state.set(GameState::Dead);
}
```

## Criteri di Accettazione
- [ ] Player si muove di 1 tile per keypress
- [ ] Player non può entrare in una tile `Wall`
- [ ] Entrare in un `Wall` non consuma step
- [ ] Ogni movimento valido decrementa `CurrentSteps` di 1
- [ ] `CurrentSteps` che raggiunge 0 transiziona a `Dead`
- [ ] Entrare in una tile con nemico emette `CombatIntent` (non il movimento)
- [ ] Player non può muoversi fuori dalla griglia 8x8
