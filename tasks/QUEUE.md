# 📋 Queue — Centurion: 100 Steps

## In Lavorazione 🚧

- `[ ]` In attesa di assegnazione del primo task.

---

## Milestone: Tactical Core (MVP)

### Core Infrastructure (Foundation & State)
1. `[ ]` **CEN-001**: Foundation — App Bevy, Plugin Architecture, Rendering Geometrico
2. `[ ]` **CEN-002**: Game State Machine (`GameState`) — Loading → Room → CombatEvent → Rest → Dead

### World & Grid Foundation
3. `[ ]` **CEN-003**: Generatore Stanza 8x8 Deterministico
4. `[ ]` **CEN-004**: Entità Giocatore — Componenti e Spawn

### Input & Movement Loop
5. `[ ]` **CEN-005**: Input System con leafwing-input-manager
6. `[ ]` **CEN-006**: Movimento su Griglia e Consumo Passi
7. `[ ]` **CEN-007**: Steps Counter HUD

### Combat & Entities
8. `[ ]` **CEN-008**: Entità Nemici — Spawn e Comportamento Statico
9. `[ ]` **CEN-009**: Risoluzione Combattimento Deterministico
10. `[ ]` **CEN-010**: Animazione Flash (bevy_tweening)

### Progression & Closure
11. `[ ]` **CEN-011**: Interazione Uscita e Progressione Piano
12. `[ ]` **CEN-012**: Schermata Rest
13. `[ ]` **CEN-013**: Schermata Dead e Statistiche Run

---

## Archiviati ✅

- `[x]` Definizione del concept iniziale (GDD)
- `[x]` Inizializzazione struttura progetto

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

*Ultimo aggiornamento: 2026-04-15*
