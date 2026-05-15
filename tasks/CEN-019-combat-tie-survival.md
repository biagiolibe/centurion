# Task CEN-019 — Combat Formula: Tie Survival

> **ID**: `CEN-019`
> **Categoria**: Bilanciamento / Bugfix
> **Priorità**: 🔴 P1
> **Stima**: ~30min
> **Assegnato a**: Claude CLI
> **Sessione**: 2026-05-15

---

## 🎯 Obiettivo

Attualmente un pareggio in combattimento (player_force == enemy_force) uccide il player. Questo è controintuitivo e punitivo: il player che affronta un nemico esattamente alla sua forza non dovrebbe morire istantaneamente. La modifica introduce un margine di grazia: il pareggio sopravvive con F1.

---

## 📋 Acceptance Criteria

- [ ] `resolve(5, 5)` → `PlayerWins { new_force: 1 }` (era `PlayerDies`)
- [ ] `resolve(5, 6)` → `PlayerDies` (invariato)
- [ ] `resolve(5, 3)` → `PlayerWins { new_force: 2 }` (invariato)
- [ ] `resolve(5, 4)` → `PlayerWins { new_force: 1 }` (era `PlayerWins { new_force: 1 }` — già corretto)
- [ ] Il test `player_dies_at_zero` è aggiornato per riflettere il nuovo comportamento
- [ ] `cargo test` passa senza warning

---

## 📁 File Rilevanti

| File | Ruolo |
|------|-------|
| `src/resolver/combat.rs` | Formula `resolve()` e unit test |

---

## 🧩 Contesto Tecnico

### Stato attuale
```rust
pub fn resolve(player_force: i32, enemy_force: i32) -> CombatResult {
    let new_force = player_force - enemy_force;
    if new_force <= 0 {          // ← pareggio (0) = morte
        CombatResult::PlayerDies
    } else {
        CombatResult::PlayerWins { new_force }
    }
}
```

### Stato desiderato
```rust
pub fn resolve(player_force: i32, enemy_force: i32) -> CombatResult {
    let new_force = player_force - enemy_force;
    if new_force < 0 {           // ← solo negativo = morte
        CombatResult::PlayerDies
    } else {
        CombatResult::PlayerWins { new_force: new_force.max(1) }  // pareggio → F1
    }
}
```

---

## 🔨 Implementazione Suggerita

1. In `src/resolver/combat.rs`: cambia `new_force <= 0` in `new_force < 0` e aggiungi `.max(1)` al `new_force` restituito.
2. Aggiorna il test `player_dies_at_zero` → rinominalo `player_ties_survives_with_one` e verifica `PlayerWins { new_force: 1 }`.
3. Aggiungi test `player_dies_when_weaker` per `resolve(5, 6)` → `PlayerDies`.
4. `cargo test` per conferma.

---

## ⚠️ Vincoli e Attenzioni

- Modifica chirurgica: solo `combat.rs`. Nessun altro file richiede modifiche.
- La logica enemy-initiated combat in `resolver/mod.rs` usa la stessa `resolve()` con argomenti invertiti — si beneficia automaticamente della correzione.

---

## 🔗 Dipendenze

- **Dipende da**: nessuno (indipendente)
- **Blocca**: nessuno

---

## 🤖 Come delegare questo task a un agente

```bash
claude "$(cat tasks/CEN-019-combat-tie-survival.md)"$'\n\nEsegui questo task nel progetto corrente.'
```
