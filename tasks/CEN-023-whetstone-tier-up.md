# Task CEN-023 — Whetstone: Full Tier Up

> **ID**: `CEN-023`
> **Categoria**: Bilanciamento (Minor)
> **Priorità**: 🟢 P3
> **Stima**: ~15min
> **Assegnato a**: Claude CLI
> **Sessione**: 2026-05-15

---

## 🎯 Obiettivo

La Whetstone applica attualmente +1 force all'ingresso nel Rest. Cambiare in un tier up completo (`tier_recovery(force)`) per renderla un oggetto più significativo.

---

## 📋 Acceptance Criteria

- [ ] Pickup Whetstone con force 9 → entra in rest con force 15 (tier_recovery(9) = 10, poi tier_recovery(10) = 15? No: tier_recovery(9+1=tier) → vedi nota)
- [ ] Più precisamente: force salvata in persistence = `tier_recovery(force.0)` se holding Whetstone
- [ ] Esempio: force 7 + Whetstone → persistence.force = tier_recovery(7) = 10
- [ ] Esempio: force 12 + Whetstone → persistence.force = tier_recovery(12) = 15
- [ ] Il rest screen mostra la forza già scalata e le opzioni di rest partono dal nuovo valore
- [ ] `cargo test` passa senza warning

---

## 📁 File Rilevanti

| File | Ruolo |
|------|-------|
| `src/player/mod.rs` | `on_enter_rest` — dove il bonus Whetstone è applicato |
| `src/ui/rest_screen.rs` | `tier_recovery` — formula da rendere accessibile o da inline |

---

## 🧩 Contesto Tecnico

### Stato attuale (`player/mod.rs`, `on_enter_rest`)
```rust
let whetstone_bonus = if held_item.map(|h| h.0) == Some(ItemKind::Whetstone) { 1 } else { 0 };
commands.insert_resource(PlayerPersistence {
    force: force.0 + whetstone_bonus,
    ...
});
```

### Stato desiderato
```rust
let whetstone_force = if held_item.map(|h| h.0) == Some(ItemKind::Whetstone) {
    ((force.0 / 5) + 1) * 5   // tier_recovery inline (formula identica a rest_screen.rs)
} else {
    force.0
};
commands.insert_resource(PlayerPersistence {
    force: whetstone_force,
    ...
});
```

La formula `((force / 5) + 1) * 5` è già usata in `rest_screen.rs::tier_recovery` — inline qui per evitare di rendere pubblica la funzione.

---

## ⚠️ Vincoli e Attenzioni

- Non modificare `tier_recovery` in `rest_screen.rs` — usare la formula inline.
- Il display nel rest screen ("Whetstone (+1 force applicato)") va aggiornato a "Whetstone (tier up applicato)" o simile.

---

## 🔗 Dipendenze

- **Dipende da**: nessuno
- **Blocca**: nessuno

---

## 🤖 Come delegare questo task a un agente

```bash
claude "$(cat tasks/CEN-023-whetstone-tier-up.md)"$'\n\nEsegui questo task nel progetto corrente.'
```
