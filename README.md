# Centurion: 100 Steps

Tactical roguelike dungeon crawler — Rust + Bevy 0.18.

Il giocatore naviga stanze 8×8 su griglia, combatte nemici con matematica deterministica e gestisce un budget di passi per salire di piano in piano.

## Stack

- **Engine**: [Bevy 0.18](https://bevyengine.org/)
- **Linguaggio**: Rust (stable)
- **Target**: Desktop (macOS/Windows/Linux) + WASM

## Quick Start

```bash
cargo run          # sviluppo (dynamic linking)
cargo test         # unit test
cargo build --release  # release / WASM-ottimizzato
```

## Documentazione progetto

| File | Contenuto |
|------|-----------|
| `CLAUDE.md` | Guida architetturale per Claude Code |
| `TECH_DESIGN.md` | Design tecnico: ECS, stati, plugin, convenzioni |
| `PROJECT_PLAN.md` | Backlog e milestone di sviluppo |
| `WORKFLOW_GUIDE.md` | Metodo di lavoro agentico (Meridian) |
| `tasks/QUEUE.md` | Coda task attiva |
| `tasks/TASK_BLUEPRINT.md` | Template per nuovi task |

## Meccaniche core

- **Griglia 8×8** — nessun movimento diagonale
- **Combattimento deterministico** — `forza_player - forza_nemico`, zero RNG
- **Budget passi** — ogni mossa consuma 1 passo; esaurirli = morte
- **Progressione** — 10 piani generati proceduralmente con seed condivisibile
