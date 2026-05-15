# 📋 Queue — Centurion: 100 Steps

## In Lavorazione 🚧

(nessuno)

---

## Milestone: Gameplay Depth Overhaul (Post-MVP)

### Phase 1: Tactical Positioning — Enemy AI
14. `[x]` **CEN-014**: Enemy AI — Movimento Dinamico (Patrol & Guard)

### Phase 2: Run Variety — Procedural Generation
15. `[x]` **CEN-015**: Procedural Generation — Seeding e Layout Dinamico

### Phase 3: Resource Management — Items & Rest Choices
16. `[x]` **CEN-016**: Items & Rest Choices — Ration, Whetstone, 3 Scelte

### Phase 4: Endgame — Win Condition & Score
17. `[x]` **CEN-017**: Win Condition — Floor 10 Boss, WinScreen, Score System

### Phase 5: Bilanciamento
18. `[ ]` **CEN-018**: Boss Reattivo + ItemKind::Runa 🔴 P1 — [CEN-018](CEN-018-boss-reattivo-runa.md)
19. `[ ]` **CEN-019**: Combat Formula — Tie Survival 🔴 P1 — [CEN-019](CEN-019-combat-tie-survival.md)
20. `[ ]` **CEN-020**: Enemy Scaling — Difficoltà Graduale 🟡 P2 — [CEN-020](CEN-020-enemy-scaling-graduale.md)
21. `[ ]` **CEN-021**: Rest Recovery — Minimo +5 🟡 P2 — [CEN-021](CEN-021-rest-recovery-minimo-5.md)
22. `[ ]` **CEN-022**: Score Formula — Combat & Items 🟢 P3 — [CEN-022](CEN-022-score-formula-combat-items.md)
23. `[ ]` **CEN-023**: Whetstone — Full Tier Up 🟢 P3 — [CEN-023](CEN-023-whetstone-tier-up.md)

---

## Milestone: Tactical Core (MVP) ✅ COMPLETATO

### Core Infrastructure (Foundation & State)
1. `[x]` **CEN-001**: Foundation — App Bevy, Plugin Architecture, Rendering Geometrico
2. `[x]` **CEN-002**: Game State Machine (`GameState`) — Loading → Room → CombatEvent → Rest → Dead

### World & Grid Foundation
3. `[x]` **CEN-003**: Generatore Stanza 8x8 Deterministico
4. `[x]` **CEN-004**: Entità Giocatore — Componenti e Spawn

### Input & Movement Loop
5. `[x]` **CEN-005**: Input System con leafwing-input-manager
6. `[x]` **CEN-006**: Movimento su Griglia e Consumo Passi
7. `[x]` **CEN-007**: Steps Counter HUD

### Combat & Entities
8. `[x]` **CEN-008**: Entità Nemici — Spawn e Comportamento Statico
9. `[x]` **CEN-009**: Risoluzione Combattimento Deterministico
10. `[x]` **CEN-010**: Animazione Flash (bevy_tweening)

### Progression & Closure
11. `[x]` **CEN-011**: Interazione Uscita e Progressione Piano
12. `[x]` **CEN-012**: Schermata Rest
13. `[x]` **CEN-013**: Schermata Dead e Statistiche Run

---

## Archiviati ✅

- `[x]` Definizione del concept iniziale (GDD)
- `[x]` Inizializzazione struttura progetto
- `[x]` **CEN-001**: Foundation — App Bevy, Plugin Architecture, Rendering Geometrico
- `[x]` **CEN-002**: Game State Machine (`GameState`)
- `[x]` **CEN-003**: Generatore Stanza 8x8 Deterministico
- `[x]` **CEN-004**: Entità Giocatore — Componenti e Spawn
- `[x]` **CEN-005**: Input System con MessageWriter (Bevy 0.18 native)
- `[x]` **CEN-006**: Movimento su Griglia e Consumo Passi
- `[x]` **CEN-007**: Steps Counter HUD
- `[x]` **CEN-008**: Entità Nemici — Spawn e Comportamento Statico
- `[x]` **CEN-009**: Risoluzione Combattimento Deterministico
- `[x]` **CEN-010**: Animazione Flash (manual timer-based, no bevy_tweening)
- `[x]` **CEN-011**: Interazione Uscita e Progressione Piano

---

## Dipendenze Critiche

```
CEN-001 → CEN-002 → CEN-003 → CEN-004
                                 ├── CEN-005 → CEN-006 → CEN-007
                                 │                └── CEN-009 → CEN-010
                                 └── CEN-008 ──────┘       └── CEN-011
                                                                  ├── CEN-012
                                                                  └── CEN-013
```

---

*Ultimo aggiornamento: 2026-05-15 — Phase 5 Bilanciamento aggiunta (CEN-018 a CEN-022)*
