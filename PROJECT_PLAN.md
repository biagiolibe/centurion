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
- `[x]` **Fase 2: Procedural Generation — Varietà tra run e seeding** — CEN-015
- `[x]` **Fase 3: Items & Rest Choices — Resource Management** — CEN-016
- `[x]` **Fase 4: Endgame & Win Condition — Score System** — CEN-017

### ⚖️ Bilanciamento (Post-MVP)
- `[x]` **CEN-018 — Boss Reattivo + ItemKind::Runa** 🔴 P1 — `boss_force = 10 + (enemies×2) - (rune×5)`; nuovo item viola che indebolisce il boss
- `[ ]` **CEN-019 — Combat Formula: Tie Survival** 🔴 P1 — Pareggio sopravvive con F1 invece di morte istantanea
- `[ ]` **CEN-020 — Enemy Scaling: Difficoltà Graduale** 🟡 P2 — `base_force = floor+1`, spread `+1` per nemico (era `+2`)
- `[ ]` **CEN-021 — Rest Recovery: Minimo +5** 🟡 P2 — `tier_recovery` garantisce sempre almeno +5 di forza
- `[ ]` **CEN-022 — Score Formula: Includi Combat & Items** 🟢 P3 — `enemies_defeated×15 + items_collected×10`, rimuovere penalità steps
- `[ ]` **CEN-023 — Whetstone: Full Tier Up** 🟢 P3 — Whetstone applica un tier completo di forza al rest invece di +1
- `[ ]` **CEN-024 — Analisi e Fix Bug Gestione Forza** 🔴 P1 — Whetstone permanente (bonus ogni floor), ordinamento `sync_player_force` vs `resolve_combat`
- `[ ]` **CEN-025 — Endless Mode** 🟡 P2 — Gioco continua oltre floor 10; boss ogni 10 floor con scala `+(cycle-1)×15`; WinScreen rimosso
- `[ ]` **CEN-026 — HUD: Info Boss Dinamico + Rune** 🟡 P2 — HUD mostra nemici/rune/boss proiettato in tempo reale; rest screen mostra formula boss corrente

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

*Ultimo aggiornamento: 2026-05-18 — CEN-018 completato; CEN-024, CEN-025, CEN-026 aggiunti*
