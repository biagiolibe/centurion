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
- `[ ]` Sistema di Turni e risorsa globale `Steps`
- `[x]` Generatore di stanze 8x8 deterministico — CEN-003

### 🗺️ Mondo & Generazione
- `[ ]` Algoritmo di distribuzione entità (Nemici, Consumabili, Uscita)
- `[ ]` Progression piano dopo piano con incremento difficoltà

### 🤖 Entità & Meccaniche
- `[ ]` Movimento giocatore (1 passo = -1 Step)
- `[ ]` Combattimento deterministico (Sottrazione Forza/HP)
- `[ ]` Feedback visivo "Flash" quando si colpisce o si viene colpiti

---

## ✅ SEZIONE 4 — COMPLETATI

### Milestones
- `[x]` Definizione del concept iniziale (GDD)
- `[x]` Foundation — App Bevy, Plugin Architecture, Rendering Geometrico — CEN-001
- `[x]` Game State Machine (`GameState`) — CEN-002
- `[x]` Generatore Stanza 8x8 Deterministico — CEN-003

---

*Ultimo aggiornamento: 2026-04-16*
