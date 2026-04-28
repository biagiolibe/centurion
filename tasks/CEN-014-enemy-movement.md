# CEN-014: Enemy AI — Movimento Dinamico (Patrol & Guard)

**Status**: [ ] Approvato | [/] In Lavorazione | [x] Completato

**Dipendenze**: CEN-013 (Schermata Dead)

**Stima**: 2–3 giorni

---

## 📋 Obiettivo

Implementare il sistema di movimento nemici con due comportamenti intelligenti — `Patrol` e `Guard` — per creare decisioni tattiche di posizionamento. I nemici diventeranno agenti che reagiscono, trasformando il gioco da "naviga il labirinto" a "pianifica la sequenza di combattimenti".

---

## ✅ Acceptance Criteria

1. **Varianti di comportamento**: `EnemyBehavior` enum ha tre varianti:
   - `Static` — invariato, nemico fermo
   - `Patrol { axis: Axis, direction: i8 }` — muovimento prevedibile lungo asse (Horizontal/Vertical)
   - `Guard { alerted: bool }` — fermo finché LOS a 3 tile dal player, poi avanza

2. **Turno nemici**: Dopo ogni mossa del player:
   - Tutti gli Enemy si muovono una volta
   - Patrol rimbalza sui muri (inverte direzione, muove 1 tile)
   - Guard esegue ray cast ortogonale a 3 tile; se vede il player → alerted = true → muoviti verso player
   - Se nemico si muove su posizione player → emette `CombatIntent { attacker: enemy, defender: player }`

3. **Deterministicità**: Zero RNG nel codice nemici/resolver. Tutto è prevedibile dato lo stato iniziale.

4. **Gameplay**: 
   - Floor 1: 1 Guard at (3,3) + 2 Static enemies
   - Floor 2: 1 Patrol (Horizontal) + 1 Guard + 1 Static
   - Floor 3+: Mix di Patrol, Guard e Static
   - Player sperimenta minacce ambientali (non solo "cammina su nemico = scontro")

5. **Nessun break**: Nemici existing continuano a funzionare; il player input non cambia.

---

## 🔧 Contesto Tecnico

### Strutture da modificare

**`enemies/components.rs`:**
```rust
// Estendi EnemyBehavior
pub enum Axis { Horizontal, Vertical }

pub enum EnemyBehavior {
    Static,
    Patrol { axis: Axis, direction: i8 },  // -1 = left/up, +1 = right/down
    Guard { alerted: bool },
}

// Nuovo struct per definizioni di nemici (usato anche in Fase 2)
pub struct EnemyDef {
    pub pos: GridPos,
    pub force: i32,
    pub behavior: EnemyBehavior,
}
```

**`enemies/mod.rs`:**
- Aggiungi `EnemyDef` alle definizioni di `enemy_positions(floor)`
- Aggiorna `spawn_enemies` per attaccare `EnemyBehavior` a ogni entità Enemy

**`tactics/mod.rs`:**
- Aggiungi nuovo `SystemSet` `EnemyTurnSet` ordinato `after(MovementSet)`

### Nuovo file: `enemies/movement.rs`

```rust
pub fn advance_enemies(
    player_q: Query<&GridPos, With<Player>>,
    mut enemy_q: Query<(&mut GridPos, &EnemyBehavior), With<Enemy>>,
    layout: Res<RoomLayout>,
    mut combat_writer: MessageWriter<CombatIntent>,
) {
    let player_pos = player_q.single();
    
    for (mut enemy_pos, behavior) in &mut enemy_q {
        let target = compute_enemy_move(enemy_pos, behavior, player_pos, &layout);
        
        if target == *player_pos {
            // Enemy cammina sul player → CombatIntent
            // Attacker = enemy, defender = player
            // La resolve() funziona simmetricamente
        } else if is_walkable(target, &layout) {
            *enemy_pos = target;
        }
    }
}

fn compute_enemy_move(
    enemy_pos: &GridPos,
    behavior: &EnemyBehavior,
    player_pos: &GridPos,
    layout: &RoomLayout,
) -> GridPos {
    match behavior {
        EnemyBehavior::Static => *enemy_pos,
        EnemyBehavior::Patrol { axis, direction } => {
            let candidate = match axis {
                Axis::Horizontal => GridPos {
                    x: (enemy_pos.x as i32 + direction) as u8,
                    y: enemy_pos.y,
                },
                Axis::Vertical => GridPos {
                    x: enemy_pos.x,
                    y: (enemy_pos.y as i32 + direction) as u8,
                },
            };
            
            if !is_walkable(candidate, layout) {
                // Rimbalza: inverti direzione e ritenta (rimani fermo se fallisce)
                *enemy_pos
            } else {
                candidate
            }
        },
        EnemyBehavior::Guard { alerted } => {
            // Ray cast a 3 tile ortogonali
            // Se hit player e nessun wall → alerted = true
            // Se alerted, step verso player (manhattan)
            
            if *alerted || player_in_los(enemy_pos, player_pos, layout) {
                step_toward(enemy_pos, player_pos)
            } else {
                *enemy_pos
            }
        }
    }
}

fn player_in_los(enemy: &GridPos, player: &GridPos, layout: &RoomLayout) -> bool {
    // Ray cast: 4 direzioni, fino a 3 tile, controlla wall
    // return true se player è visibile
}

fn step_toward(enemy: &GridPos, player: &GridPos) -> GridPos {
    // Manhattan: preferisci chiudere l'asse più distante
}
```

### Integration Points

1. **`enemies/mod.rs::EnemiesPlugin`**: 
   - Aggiungi sistema `advance_enemies` in `EnemyTurnSet` dopo `MovementSet`
   - Assicurati che enemy spawn attacchi `EnemyBehavior` component

2. **`resolver/mod.rs`**: 
   - Quando un nemico cambia attacker, la `resolve()` rimane invariata (simmetrica)
   - Potresti aggiungere un `attacker_kind: AttackerKind { Player, Enemy }` a CombatIntent per routing (opzionale)

3. **`tactics/movement.rs`**: 
   - Nessun cambio richiesto; il movimento player rimane indipendente

---

## 🎮 Comportamento Dettagliato

### Patrol

```
Initial: pos=(3,3), axis=Horizontal, direction=+1

Turn 1: candidate=(4,3), walkable → pos=(4,3)
Turn 2: candidate=(5,3), walkable → pos=(5,3)
Turn 3: candidate=(6,3), walkable → pos=(6,3)
Turn 4: candidate=(7,3), is_wall → rimbalza, pos=(6,3), direction=-1
Turn 5: candidate=(5,3), walkable → pos=(5,3)
```

Prevedibile, permette al player di cronometrare passaggi sicuri.

### Guard

```
Initial: pos=(3,3), alerted=false

Turn 1: Player at (5,3) — no LOS (es. muro tra), rimani fermo
Turn 2: Player si muove a (3,5) — adiacente diagonale, non in LOS ortogonale, rimani fermo
Turn 3: Player a (3,4) — LOS diretto orizzontale a 1 tile, ALERTED=true
Turn 4: Step verso (3,4) → (3,3) o (2,3) o (4,3) (preferisci asse con delta > 0)
...
```

Crea minaccia dinamica: player deve evitare la linea di vista.

---

## 📊 Enemy Definitions Update

Esempi di `enemy_positions()` output dopo aggiornamento:

```rust
// Floor 1
vec![
    EnemyDef { pos: (3,3), force: 3, behavior: Guard { alerted: false } },
    EnemyDef { pos: (5,5), force: 7, behavior: Static },
    EnemyDef { pos: (6,3), force: 4, behavior: Static },
]

// Floor 2
vec![
    EnemyDef { pos: (2,2), force: 4, behavior: Patrol { axis: Horizontal, direction: 1 } },
    EnemyDef { pos: (5,5), force: 9, behavior: Guard { alerted: false } },
    EnemyDef { pos: (6,3), force: 6, behavior: Static },
    EnemyDef { pos: (3,6), force: 5, behavior: Static },
]
```

---

## 🧪 Testing Plan

1. **Floor 1, Guard behavior**: Player si avvicina da fondo-sinistra, verifica che Guard non si muove finché il player non raggiunge la linea di vista.
2. **Floor 2, Patrol**: Osserva che il nemico rimbalza sui muri e non rimane bloccato.
3. **Enemy move after player move**: Pressione un tasto, player si muove, poi i nemici si muovono — verifica ordine di sistema con `dbg!` log.
4. **Enemy walks into player**: Crea una situazione (es. Guard avanza verso player in stanza stretta) dove il nemico si muove su player; verifica `CombatIntent` corretta.
5. **Nessun crash**: 20 floor completati senza panic.

---

## 📝 Note

- Patrol rimbalza invertendo direzione ma rimanendo nella stessa tile (se candidate è muro). Questo è corretto per prevedibilità.
- Guard LOS: ray cast ortogonale a 3 tile è sufficiente; no diagonale.
- Determinismo: usare solo GridPos, niente timestamp o timer per le decisioni nemiche.
- La `resolve()` rimane invariata — il fatto che l'attacker sia un nemico non cambia il calcolo forza.

---

*Versione: 1.0 | Creato: 2026-04-28*
