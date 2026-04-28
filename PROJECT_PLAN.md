# Project Plan — Centurion: 100 Steps

Questo documento traccia l'evoluzione del progetto dalle idee alla realizzazione.

## Ciclo di Vita dei Task

```
PROPOSTE  →  (revisione)  →  BACKLOG  →  (sviluppo)  →  COMPLETATI
```

| Simbolo | Significato |
|---------|-------------|
| `[ ]`   | Task approvato nel backlog |
| `[/]`   | Task in lavorazione |
| `[x]`   | Task completato |
| `[-]`   | Task annullato / scartato |
| `[?]`   | Proposta (in attesa di valutazione) |

---

## 🗂️ SEZIONE 1 — PROPOSTE

### Gameplay
- `[?]` Daily Seed: Classifica mondiale basata sulla stessa run giornaliera.
- `[?]` Classi del Centurione: diverse abilità passive (es. più forza ma meno passi totali).
- `[?]` Shop degli Déi: altari che offrono passi in cambio di sacrifici permanenti di forza.

---

## 🔵 SEZIONE 2 — BACKLOG (Operativo)

### 🏗️ Architettura & Sistemi Core
- `[x]` Setup progetto Rust + Bevy (minimal geometric style) — CEN-001
- `[x]` Sistema di Turni e risorsa globale `Steps` — CEN-002
- `[x]` Generatore di stanze 8x8 deterministico — CEN-003

### 🗺️ Mondo & Generazione
- `[x]` Algoritmo di distribuzione entità (Nemici, Consumabili, Uscita) — CEN-004 a CEN-013
- `[x]` **Fase 1: Enemy AI — Movimento dinamico (Patrol & Guard)** — CEN-014
- `[ ]` **Fase 2: Procedural Generation — Varietà tra run e seeding** — CEN-015
- `[ ]` **Fase 3: Items & Rest Choices — Resource Management** — CEN-016
- `[ ]` **Fase 4: Endgame & Win Condition — Score System** — CEN-017

### 🤖 Entità & Meccaniche (MVP ✅)
- `[x]` Movimento giocatore (1 passo = -1 Step)
- `[x]` Combattimento deterministico (Sottrazione Forza/HP)
- `[x]` Feedback visivo "Flash" quando si colpisce o si viene colpiti

---

## ✅ SEZIONE 4 — COMPLETATI

### Milestones
- `[x]` Definizione del concept iniziale (GDD)
- `[x]` Foundation — App Bevy, Plugin Architecture, Rendering Geometrico — CEN-001
- `[x]` Game State Machine (`GameState`) — CEN-002
- `[x]` Generatore Stanza 8x8 Deterministico — CEN-003

---

*Ultimo aggiornamento: 2026-04-28*
