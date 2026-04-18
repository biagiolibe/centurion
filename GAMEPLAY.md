# Centurion: 100 Steps — Schema Gameplay Completo

---

## 1. Macchina a stati

```
                    ┌─────────┐
                    │ LOADING │  (istantaneo, passa subito a Room)
                    └────┬────┘
                         │
                         ▼
               ┌─────────────────┐
          ┌───▶│      ROOM       │◀────────────────┐
          │    └────────┬────────┘                 │
          │             │                          │
          │    ┌────────┴────────┐                 │
          │    │ Player moves    │                 │
          │    │ on grid         │                 │
          │    └────────┬────────┘                 │
          │             │                          │
          │         ┌───┴───┬───────┐              │
          │         │       │       │              │
          │    Steps=0  Exit  Enemy                │
          │         │       │       │              │
          │         ▼       ▼       ▼              │
          │      DEAD    REST  COMBAT EVENT       │
          │                    (flash <1 frame)
          │                         │
          │         ┌───────────────┘
          │         │
          │    ┌────▼─────┐   SPACE
          └────│   REST    │─────────────┐
               └───────────┘  (floor++)  │
                    ▲                    │
                    │                    │
                    └────────────────────┘

DEAD state:
  ▲
  │ Steps=0 o Force≤0
  │ (combat loss)
  │
  └─ R: reset completo → LOADING
```

---

## 2. Layout della stanza (8×8, grid-based)

```
     0   1   2   3   4   5   6   7
  0  [W] [W] [W] [W] [W] [W] [W] [W]
  1  [W] [P]  .   .   .   .   .  [W]
  2  [W]  .   .   .   .   .   .  [W]
  3  [W]  .   .  [E]  .   .  [E] [W]    Legenda:
  4  [W]  .   .   .  [X]  .   .  [W]    P = Player (spawn 1,1)
  5  [W]  .   .   .   .  [E]  .  [W]    E = Enemy (visibili)
  6  [W]  .   .   .   .   .   .  [W]    X = Exit tile
  7  [W] [W] [W] [W] [W] [W] [W] [W]    W = Wall (impassabile)
                                        . = Floor (calpestabile)

HUD (overlay top-left, white text, 24px):
┌───────────────┐
│ STEPS: 100    │
│ FORCE: 5      │
│ FLOOR: 1      │
└───────────────┘

Exit tile position per piano:
  Floor 1: (4, 4)
  Floor 2: (5, 4)
  Floor 3: (6, 4)
  Floor 4+: (4 + ((floor-1) % 3), 4)
```

---

## 3. Risorse del giocatore

| Risorsa | Descrizione | Inizio | Recupero |
|---|---|---|---|
| **Steps** | Movimento disponibile per il piano | 100 | +20 al Rest se completi il piano |
| **Force** | Potenza in combattimento, "HP globale" | 5 | Tier up al Rest (vedi sez. 5) |
| **Floor** | Numero del piano (difficoltà) | 1 | +1 quando raggiungi Exit |

Ogni movimento costa 1 Step (indipendentemente da direzione o outcome).

---

## 4. Meccanica di movimento e combattimento

### 4.1 — Loop di un turno

```
  [Giocatore preme tasto (WASD/Arrow)]
              │
              ▼
     MoveIntent emesso
              │
              ▼
  ┌──────────────────────────────────┐
  │    apply_movement() system       │
  │                                  │
  │  new_pos = current_pos +         │
  │            direction             │
  │                                  │
  │  (1) Bounds check 0..8?          │
  │      NO → ignora, return         │
  │                                  │
  │  (2) Wall at new_pos?            │
  │      YES → ignora (no cost)      │
  │                                  │
  │  (3) Enemy at new_pos?           │
  │      YES → CombatIntent          │
  │                                  │
  │  (4) Floor/Exit at new_pos?      │
  │      YES → muovi + steps-1       │
  │                                  │
  └──────────────────────────────────┘
        │                  │
        │                  ▼
        │          Steps == 0?
        │              │
        │             YES ──▶ GameState::Dead
        │
        ▼
  [Check exit tile sotto player?]
        │
       YES ──▶ floor++ ──▶ GameState::Rest
```

### 4.2 — Risoluzione combattimento (CEN-009)

Quando il giocatore si avvicina a un nemico:

```
  CombatIntent { attacker: player, defender: enemy }
        │
        ▼
  ┌──────────────────────┐
  │ resolve() {          │
  │  new_force =         │
  │    P.force - E.force │
  │ }                    │
  └──────────────────────┘
        │
    ┌───┴───┐
    │       │
new_force new_force
   > 0      ≤ 0
    │       │
    ▼       ▼
 Player  DEAD
 wins    (perdi)

Se Player vince:
  • Player.force = new_force
  • Nemico despawnato
  • Player si sposta sulla tile del nemico
  • Transiziona a GameState::CombatEvent (flash)
  • Dopo flash → torna a Room
```

**Esempi Floor 1:**
```
Player F5 vs Enemy F3 → new_force = 2 (vinci, Force → 2)
Player F5 vs Enemy F7 → new_force = -2 (DEAD)
Player F2 vs Enemy F2 → new_force = 0 (DEAD, perché ≤0)
```

---

## 5. Progressione verticale — Force Tier System

### 5.1 — Meccanica di Rest

Al raggiungimento dell'Exit e transizione a `GameState::Rest`, la Force viene
portata al **prossimo multiplo di 5 superiore**.

```rust
fn rest_force(force: i32) -> i32 {
    ((force / 5) + 1) * 5
}
```

### 5.2 — Tabella di transizione

| Arrivi con F | Riparte con F | Tipo | Note |
|---|---|---|---|
| 1–4 | 5 | penalizzato | piano molto danneggiato |
| 5 | 10 | tier boost | evitato i combattimenti |
| 6–9 | 10 | parziale | qualche danno |
| 10 | 15 | tier boost | uscito intatto dal secondo piano |
| 11–14 | 15 | parziale | danno leggero |
| 15 | 20 | tier boost | — |

### 5.3 — Incentivo di design

**Arrivare con Force multiplo di 5 esatto fa un salto di tier.**

- Se arrivi a `Floor 1 Exit` con F5 esatto → Rest → F10
- Se arrivi a `Floor 1 Exit` con F4 → Rest → F5 (nessun bonus)

Questo premia chi **combatte con precisione** o **evita strategicamente** i nemici giusti.

### 5.4 — Difficoltà nemica per piano

```
Floor 1 enemies: F3, F7, F4 (max threat: 7)
Floor 2 enemies: F4, F9, F6, F5 (max threat: 9)
Floor 3 enemies: F5, F10, F7 (max threat: 10)
Floor 4+ enemies: base_force = 2 + floor
                   positions: standard (2,2), (5,5), (6,3), ...
```

La difficoltà scala con il piano, ma il Force tier system lo compensa parzialmente.

**Scenario tipico di run 3 piani:**

```
Floor 1:
  Start: F5, 100 steps
  Encounter: E(3), E(7), E(4)
  Choice: beat E(3) → F2, 99 steps
  Reach Exit: F2, 74 steps
  Rest: F2 → F5

Floor 2:
  Start: F5, 74 steps
  Encounter: E(4), E(9), E(6), E(5)
  Choice: beat E(4) → F1, avoid others → 60 steps to Exit
  Reach Exit: F1, 60 steps
  Rest: F1 → F5 (penalizzato, non tier boost)

Floor 3:
  Start: F5, 60 steps
  Encounter: E(5), E(10), E(7)
  ALL nemici richiedono F>5, sono pericolosi
  Outcome: muori per Step esauriti (F5 è alto abbastanza da non perdere in combattimento facile)
  Dead screen: Floors cleared: 2, Steps taken: 40, Cause: Out of steps
```

---

## 6. Schermata REST (tra piani)

Appare al raggiungimento dell'Exit, centrata, white text su sfondo nero.

```
┌──────────────────────────────────┐
│                                  │
│        FLOOR 1 CLEARED           │  (32px, white)
│                                  │
│    STEPS REMAINING: 74           │  (24px, white)
│    FORCE: 5                      │  (24px, white)
│    FORCE after rest: 10          │  (24px, yellow — preview tier)
│                                  │
│    Press SPACE to descend        │  (20px, gray)
│                                  │
└──────────────────────────────────┘
```

**Logica:**
- `current_floor - 1` (il piano appena completato)
- Steps e Force da `PlayerPersistence`
- SPACE → transiziona a `GameState::Room`
- Step counter nel nuovo Floor 2 parte da 74
- Force del nuovo Floor 2 parte dal valore tier-upped (F10 in questo caso)

---

## 7. Schermata DEAD

Appare quando Steps = 0 o Force ≤ 0 (sconfitta in combattimento).
Centrata, testo in colori diversi a seconda della causa.

```
┌──────────────────────────────────┐
│                                  │
│      CENTURION FALLS             │  (36px, rosso vivo)
│                                  │
│    Floors cleared: 2             │  (24px, white)
│    Steps taken: 40               │  (24px, white)
│    Steps remaining: 0            │  (24px, white)
│    Cause: Out of steps           │  (24px, white)
│                                  │
│    Press R to restart            │  (20px, gray)
│                                  │
└──────────────────────────────────┘

Alternate Cause:
    Cause: Killed by Force 7       │  (se sconfitto in combattimento)
```

**Logica restart (R):**
1. Reset `CenturionConfig.current_floor = 1`
2. Reset `PlayerPersistence.steps = 100`, `.force = 5`
3. Transiziona a `GameState::Loading` → Room
4. Nuovo ciclo di gioco

---

## 8. Tensioni di design core

| Dimensione | Meccanica | Conflitto |
|---|---|---|
| **Steps** | Budget di movimento per piano | Avanzare verso Exit vs. evitare nemici |
| **Force** | Potere di combattimento | Combattere (rischiare danno) vs. evitare (bruciare Steps) |
| **Floor** | Difficoltà crescente | Nemici più forti vs. Force tier recovery lenta |

**Zona di gioco:** ogni piano è un trade-off. Steps scarseggiano, Force non si recupera bene se danneggiata. Il giocatore deve pianificare il percorso dalla spawn all'Exit evitando nemici potenti o affrontandoli in modo calcolato.

---

## 9. Architettura Bevy 0.18

### Plugin structure
```
CenturionPlugins
  ├── CenturionRenderPlugin      (visual, tiles, sprites)
  ├── StatePlugin                 (state machine)
  ├── MapGenPlugin                (room generation)
  ├── PlayerPlugin                (player entity, spawn)
  ├── InputPlugin                 (WASD/arrow input → MoveIntent)
  ├── TacticsPlugin               (apply_movement, CombatIntent)
  ├── EnemiesPlugin               (spawn_enemies per piano)
  ├── ResolverPlugin              (resolve_combat, force calculus)
  └── HudPlugin                   (steps/force/floor display, rest/dead screens)
```

### State flow
```
Loading → Room ←→ Rest
          ↓
        CombatEvent (transitorio, flash <1 frame)
          ↓
        Room (ritorno)

Room → Dead (se Steps=0 o Force≤0)
```

### Entity lifecycle
- Player: spawn in Room, despawn on exit via `DespawnOnExit(GameState::Room)`
- Enemies: spawn in Room, despawn on exit o quando sconfitti
- Tiles: spawn in Room, despawn on exit
- HUD: spawn in Room, despawn on exit
- Rest screen: spawn in Rest, despawn on exit
- Dead screen: spawn in Dead, despawn on exit

---

## 10. Checklist MVP

- [x] CEN-001: Foundation (Bevy, plugins, rendering)
- [x] CEN-002: Game state machine (Loading → Room → Dead, etc.)
- [x] CEN-003: Room generation (8×8, walls, exit)
- [x] CEN-004: Player entity (spawn, GridPos, Force, Steps)
- [x] CEN-005: Input system (WASD/arrow → MoveIntent)
- [x] CEN-006: Movement (apply_movement, step decay, bounds check)
- [x] CEN-007: HUD (steps/force/floor display, real-time update)
- [ ] CEN-008: Enemies (spawn_enemies per floor, static behavior)
- [ ] CEN-009: Combat resolution (resolve(), force calculus)
- [ ] CEN-010: Combat flash (CombatEvent state, visual feedback)
- [ ] CEN-011: Exit detection (check_exit, floor increment)
- [ ] CEN-012: Rest screen (tier recovery, stats display)
- [ ] CEN-013: Dead screen (run stats, restart)

---

**Version 1.0 — 2026-04-18**  
Schema definitivo con Force Tier System, integrazione Bevy 0.18 API, design goals chiari.
