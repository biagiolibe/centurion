# Task CEN-020 — Enemy Scaling: Difficoltà Graduale

> **ID**: `CEN-020`
> **Categoria**: Bilanciamento
> **Priorità**: 🟡 P2
> **Stima**: ~30min
> **Assegnato a**: Claude CLI
> **Sessione**: 2026-05-15

---

## 🎯 Obiettivo

La curva di difficoltà dei nemici è attualmente troppo ripida. Al floor 1 il player (F5) può battere solo il nemico più debole (F3) e rischia morte immediata dagli altri (F5, F7, F9). Ridurre base e spread della force nemici rende ogni floor affrontabile senza trivializzare la progressione.

---

## 📋 Acceptance Criteria

- [ ] Floor 1: nemici con force 2, 3, 4, 5 (era 3, 5, 7, 9)
- [ ] Floor 5: nemici con force 6, 7, 8, 9 (era 7, 9, 11, 13)
- [ ] Floor 9: nemici con force 10, 11, 12, 13 (era 11, 13, 15, 17)
- [ ] I test esistenti su enemy positions passano invariati
- [ ] `cargo test` passa senza warning

---

## 📁 File Rilevanti

| File | Ruolo |
|------|-------|
| `src/map_gen/procgen.rs` | `generate_enemy_defs()` — formula force nemici (righe ~147-148) |

---

## 🧩 Contesto Tecnico

### Stato attuale (`generate_enemy_defs` in `procgen.rs`)
```rust
let base_force = floor as i32 + 2;   // floor 1 → 3, floor 9 → 11

shuffled
    .into_iter()
    .take(count)
    .enumerate()
    .map(|(i, pos)| {
        let force = base_force + (i as i32 * 2);  // spread: +0, +2, +4, +6
        // floor 1: 3, 5, 7, 9
```

### Stato desiderato
```rust
let base_force = floor as i32 + 1;   // floor 1 → 2, floor 9 → 10

        let force = base_force + i as i32;         // spread: +0, +1, +2, +3
        // floor 1: 2, 3, 4, 5
```

---

## 🔨 Implementazione Suggerita

In `src/map_gen/procgen.rs`, funzione `generate_enemy_defs`:

1. Cambia `let base_force = floor as i32 + 2;` → `let base_force = floor as i32 + 1;`
2. Cambia `let force = base_force + (i as i32 * 2);` → `let force = base_force + i as i32;`
3. `cargo test` per conferma (i test esistenti verificano posizioni, non force — passano invariati).

---

## ⚠️ Vincoli e Attenzioni

- Modifica chirurgica: solo 2 righe in `procgen.rs`.
- Il boss al floor 10 usa una formula separata in `enemies/mod.rs` — non è toccato da questo task.
- La difficoltà risultante va testata manualmente: floor 1 deve essere sfidante ma non letale per un player che combatte i nemici più deboli.

---

## 🔗 Dipendenze

- **Dipende da**: nessuno (indipendente)
- **Blocca**: nessuno

---

## 🤖 Come delegare questo task a un agente

```bash
claude "$(cat tasks/CEN-020-enemy-scaling-graduale.md)"$'\n\nEsegui questo task nel progetto corrente.'
```
