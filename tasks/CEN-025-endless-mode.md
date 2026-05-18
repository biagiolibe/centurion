# Task CEN-025 — Endless Mode

> **ID**: `CEN-025`
> **Categoria**: Feature
> **Priorità**: 🟡 P2
> **Stima**: ~2h
> **Assegnato a**: Claude CLI
> **Sessione**: 2026-05-18

---

## 🎯 Obiettivo

Attualmente il gioco termina con una WinScreen dopo la vittoria sul floor 10. Il player dovrebbe invece poter continuare all'infinito: boss ogni 10 floor, difficoltà crescente, run che finisce solo per morte o passi esauriti. Il Dead screen diventa il vero "end screen" con il floor raggiunto come metrica principale.

---

## 📋 Acceptance Criteria

- [ ] Dopo la vittoria sul boss (floor 10, 20, 30…) il gioco va in `GameState::Rest` invece di `GameState::WinScreen`
- [ ] Il boss spawna su ogni floor multiplo di 10 (`floor % 10 == 0`), non solo floor 10
- [ ] La formula boss scala con il ciclo: `boss_force = (10 + enemies×2 - runes×5 + (cycle-1)×15).max(1)` dove `cycle = floor / 10`
- [ ] Items e nemici normali continuano a spawnare sui floor 1–9, 11–19, 21–29…
- [ ] Il Dead screen mostra il floor massimo raggiunto (già disponibile via `run_stats.floors_cleared`)
- [ ] `WinScreen` viene rimosso dallo state machine (o tenuto come schermata milestone se si vuole mantenere la celebrazione al floor 10 — vedi nota)
- [ ] `LastCombatOutcome::BossVictory` viene rimosso o ripurposato
- [ ] `cargo test` passa senza warning

**Nota**: Se si vuole mantenere una schermata di celebrazione al floor 10 (primo boss kill), si può tenere `WinScreen` come transizione intermedia che poi redirige a `Rest` con una pressione di tasto, invece di terminare la run.

---

## 📁 File Rilevanti

| File | Ruolo |
|------|-------|
| `src/resolver/flash.rs` | `LastCombatOutcome::BossVictory` → `WinScreen`; da cambiare in `Rest` |
| `src/resolver/mod.rs:95-99` | Imposta `LastCombatOutcome::BossVictory` sul floor 10 |
| `src/enemies/mod.rs:49` | Condizione boss `floor == 10` → `floor % 10 == 0` |
| `src/enemies/mod.rs:50-65` | Formula boss force; aggiungere scaling per ciclo |
| `src/state.rs` | `WinScreen` nello state machine — rimuovere o mantenere come milestone |
| `src/ui/win_screen.rs` | Handler WinScreen — rimuovere o adattare |
| `src/items/mod.rs:32` | Skip items su floor 10 → skip su `floor % 10 == 0` |
| `src/map_gen/` | Floor 10 arena (no pillars, no exit) → tutti i floor boss |

---

## 🧩 Contesto Tecnico

### Flusso attuale (da cambiare)
```
Boss sconfitto → resolve_combat() imposta LastCombatOutcome::BossVictory
  → CombatEvent (0.2s flash)
  → update_flash() vede BossVictory → GameState::WinScreen
  → run terminata
```

### Flusso nuovo
```
Boss sconfitto → resolve_combat() imposta LastCombatOutcome::Victory (normale)
  → CombatEvent (0.2s flash)
  → update_flash() vede Victory → GameState::Room
  → player cammina sull'exit → GameState::Rest → floor+1
```

### Formula boss scalata
```rust
// src/enemies/mod.rs
if config.current_floor % 10 == 0 {
    let cycle = (config.current_floor / 10) as i32;
    let boss_force = (10 + (run_stats.enemies_defeated as i32 * 2)
        - (run_stats.runes_collected as i32 * 5)
        + (cycle - 1) * 15)
        .max(1);
    // ... spawn boss
}
```

Al floor 10 (cycle=1): formula base, nessun bonus extra.
Al floor 20 (cycle=2): +15 rispetto alla formula base.
Al floor 30 (cycle=3): +30, ecc.

### `LastCombatOutcome` dopo la modifica
```rust
pub enum LastCombatOutcome {
    #[default]
    Victory,
    Defeat,
    // BossVictory rimosso
}
```

### Map gen per floor boss
`build_room_proc()` in `map_gen/procgen.rs` o `map_gen/room.rs` ha già la logica per floor 10 (arena aperta, no exit, no pillar). Questa logica va generalizzata a `floor % 10 == 0`.

---

## 🔨 Implementazione Suggerita

1. **`src/resolver/flash.rs`**: rimuovere `BossVictory` dall'enum, unificare con `Victory` nel match
2. **`src/resolver/mod.rs`**: nel ramo `is_last_enemy && floor % 10 == 0`, impostare `LastCombatOutcome::Victory` (non più `BossVictory`)
3. **`src/enemies/mod.rs`**: cambiare `floor == 10` → `floor % 10 == 0`; aggiungere `cycle` alla formula
4. **`src/items/mod.rs`**: cambiare `floor == 10` → `floor % 10 == 0`
5. **`src/map_gen/`**: generalizzare la logica arena a `floor % 10 == 0`
6. **`src/state.rs` + `src/ui/win_screen.rs`**: rimuovere `WinScreen` state (o convertirlo in milestone intermedia non bloccante)
7. **Aggiornare `calculate_score()`** in `config.rs` per includere il floor raggiunto come termine principale

---

## ⚠️ Vincoli e Attenzioni

- **Test `test_room_always_connected`** itera floor 1–9; se la logica arena diventa `floor % 10 == 0`, non impatta questi test (floor 10 era già escluso dal loop)
- **`DespawnOnExit(GameState::Dead)`** sui nemici: verifica che gli enemy despawnino correttamente al Rest anche per floor boss (già gestito da `cleanup_enemies` in `OnEnter(Rest)`)
- **`run_stats.floors_cleared`** in `on_enter_rest` usa `current_floor - 1`; funziona correttamente anche oltre floor 10
- Evitare di usare `WinScreen` come dipendenza in altri sistemi prima di rimuoverlo

---

## 🔗 Dipendenze

- **Dipende da**: CEN-018 (completato — formula boss reattiva già in place)
- **Coordinare con**: CEN-024 (bug forza) — indipendente ma testare insieme
- **Blocca**: nessuno

---

## 🤖 Come delegare questo task a un agente

```bash
claude "$(cat tasks/CEN-025-endless-mode.md)"$'\n\nEsegui questo task nel progetto corrente.'
```
