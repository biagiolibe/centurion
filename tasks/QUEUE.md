# 📋 Queue — Centurion: 100 Steps

## In Lavorazione 🚧

(nessuno)

---

## Milestone: Tactical Core (MVP)

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

*Ultimo aggiornamento: 2026-04-20 — CEN-011 completato*
