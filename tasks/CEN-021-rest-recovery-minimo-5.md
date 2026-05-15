# Task CEN-021 — Rest Recovery: Minimo +5

> **ID**: `CEN-021`
> **Categoria**: Bilanciamento
> **Priorità**: 🟡 P2
> **Stima**: ~30min
> **Assegnato a**: Claude CLI
> **Sessione**: 2026-05-15

---

## 🎯 Obiettivo

La funzione `tier_recovery` porta la force al prossimo multiplo di 5, ma può dare benefici minimi se il player arriva vicino a un tier boundary. Con force 9 l'opzione 1 del rest dà solo +1 (→10), disincentivando il combattimento aggressivo. La modifica garantisce un minimo di +5 per ogni tier recovery.

---

## 📋 Acceptance Criteria

- [ ] `tier_recovery(9)` → 14 (era 10; ora max(10, 9+5) = 14)
- [ ] `tier_recovery(6)` → 11 (era 10; ora max(10, 6+5) = 11)
- [ ] `tier_recovery(5)` → 10 (era 10; ora max(10, 5+5) = 10 — invariato)
- [ ] `tier_recovery(10)` → 15 (era 15; ora max(15, 10+5) = 15 — invariato)
- [ ] `tier_recovery(1)` → 6 (era 5; ora max(5, 1+5) = 6)
- [ ] L'opzione 3 al rest (doppio tier) usa la stessa funzione aggiornata e beneficia automaticamente
- [ ] Il rest screen visualizza correttamente i valori aggiornati
- [ ] `cargo test` passa senza warning

---

## 📁 File Rilevanti

| File | Ruolo |
|------|-------|
| `src/ui/rest_screen.rs` | `tier_recovery()` — funzione locale usata da `spawn_rest_screen` e `handle_rest_choice` |

---

## 🧩 Contesto Tecnico

### Stato attuale
```rust
fn tier_recovery(force: i32) -> i32 {
    ((force / 5) + 1) * 5   // prossimo multiplo di 5
}
// Esempi: 9→10 (+1), 6→10 (+4), 4→5 (+1), 1→5 (+4)
```

### Stato desiderato (Proposta A — garantisce minimo +5)
```rust
fn tier_recovery(force: i32) -> i32 {
    let tier = ((force / 5) + 1) * 5;
    tier.max(force + 5)
}
// Esempi: 9→14 (+5), 6→11 (+5), 4→9 (+5), 1→6 (+5), 5→10 (+5), 10→15 (+5)
```

La logica del rest screen (opzioni 1, 2, 3 e il bonus Whetstone) non cambia — usa `tier_recovery` come before.

---

## 🔨 Implementazione Suggerita

In `src/ui/rest_screen.rs`:

1. Modifica `tier_recovery` aggiungendo `tier.max(force + 5)`.
2. Verifica visivamente che i valori mostrati nelle tre opzioni del rest screen siano corretti.
3. `cargo test` per conferma.

---

## ⚠️ Vincoli e Attenzioni

- `tier_recovery` è una funzione privata locale a `rest_screen.rs` — nessun altro file la usa.
- L'opzione 3 (training intensivo) applica `tier_recovery(tier_recovery(force))` — con la modifica il doppio tier è ancora più generoso, ma rimane equilibrata perché costa tutti i passi bonus.
- Il Whetstone aggiunge +1 a `force` prima del tier — questo bonus si propaga correttamente.

---

## 🔗 Dipendenze

- **Dipende da**: nessuno (indipendente)
- **Blocca**: nessuno

---

## 🤖 Come delegare questo task a un agente

```bash
claude "$(cat tasks/CEN-021-rest-recovery-minimo-5.md)"$'\n\nEsegui questo task nel progetto corrente.'
```
