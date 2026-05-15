# Task CEN-022 — Score Formula: Combat & Items

> **ID**: `CEN-022`
> **Categoria**: Bilanciamento / Feature
> **Priorità**: 🟢 P3
> **Stima**: ~30min
> **Assegnato a**: Claude CLI
> **Sessione**: 2026-05-15

---

## 🎯 Obiettivo

La formula del punteggio attuale ignora `enemies_defeated` e `items_collected` (già tracciati in `RunStats`) e applica una penalità su `total_steps_taken` che è ridondante con `steps_remaining`. La nuova formula premia il combattimento e la raccolta oggetti, rendendo il punteggio una metrica più significativa della qualità della run.

---

## 📋 Acceptance Criteria

- [ ] Formula aggiornata: `(floors_cleared×100) + (steps_remaining×2) + (force×10) + (enemies_defeated×15) + (items_collected×10)`
- [ ] La penalità `total_steps_taken / 2` è rimossa
- [ ] WinScreen mostra il punteggio calcolato con la nuova formula
- [ ] DeadScreen mostra il punteggio calcolato con la nuova formula
- [ ] Esempio verificabile: 10 floor, F25, 60 steps, 15 nemici, 8 items → `1000 + 120 + 250 + 225 + 80 = 1675`
- [ ] `cargo test` passa senza warning

---

## 📁 File Rilevanti

| File | Ruolo |
|------|-------|
| `src/config.rs` | `RunStats::calculate_score()` — formula da aggiornare |
| `src/ui/win_screen.rs` | Chiama `calculate_score` — verificare parametri passati |
| `src/ui/dead_screen.rs` | Chiama `calculate_score` — verificare parametri passati |

---

## 🧩 Contesto Tecnico

### Firma attuale di `calculate_score` (config.rs)
```rust
impl RunStats {
    pub fn calculate_score(&self, floors_cleared: u8, steps_remaining: i32, force: i32) -> i32 {
        (floors_cleared as i32 * 100)
            + (steps_remaining * 2)
            + (force * 10)
            - (self.total_steps_taken / 2)   // ← da rimuovere
    }
}
```

### Stato desiderato
```rust
impl RunStats {
    pub fn calculate_score(&self, floors_cleared: u8, steps_remaining: i32, force: i32) -> i32 {
        (floors_cleared as i32 * 100)
            + (steps_remaining * 2)
            + (force * 10)
            + (self.enemies_defeated as i32 * 15)
            + (self.items_collected as i32 * 10)
    }
}
```

La firma del metodo non cambia — i caller in `win_screen.rs` e `dead_screen.rs` non richiedono modifiche.

### Chiamate esistenti (invariate)
```rust
// win_screen.rs
let score = stats.calculate_score(floors_cleared, steps_remaining, force);

// dead_screen.rs
let score = run_stats.calculate_score(floors_cleared, steps_remaining.max(0), force);
```

---

## 🔨 Implementazione Suggerita

In `src/config.rs`, metodo `RunStats::calculate_score`:

1. Rimuovere `- (self.total_steps_taken / 2)`
2. Aggiungere `+ (self.enemies_defeated as i32 * 15)`
3. Aggiungere `+ (self.items_collected as i32 * 10)`
4. Verificare che `cargo test` passi.
5. (Opzionale) Fare una run di test e verificare manualmente che il punteggio a fine run sia sensato.

---

## ⚠️ Vincoli e Attenzioni

- `items_collected` conta **tutti** gli item (Ration, Whetstone, Runa) — con CEN-018 completato la Runa è inclusa nel conteggio, il che è corretto.
- `enemies_defeated` viene incrementato in `resolver/mod.rs` ad ogni vittoria in combattimento.
- `total_steps_taken` rimane nel `RunStats` struct (usato altrove? no — ma lasciarlo non causa problemi, serve solo per debugging).

---

## 🔗 Dipendenze

- **Dipende da**: CEN-017 (completato — `RunStats`, `calculate_score`, WinScreen già presenti)
- **Consigliato dopo**: CEN-018 (per includere le Rune nel conteggio items_collected)
- **Blocca**: nessuno

---

## 🤖 Come delegare questo task a un agente

```bash
claude "$(cat tasks/CEN-022-score-formula-combat-items.md)"$'\n\nEsegui questo task nel progetto corrente.'
```
