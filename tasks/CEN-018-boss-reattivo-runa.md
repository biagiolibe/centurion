# Task CEN-018 — Boss Reattivo + ItemKind::Runa

> **ID**: `CEN-018`
> **Categoria**: Feature / Bilanciamento
> **Priorità**: 🔴 P1
> **Stima**: ~2h
> **Assegnato a**: Claude CLI
> **Sessione**: 2026-05-15

---

## 🎯 Obiettivo

Il boss al floor 10 è attualmente **imbattibile**: `boss_force = player_entry_force * 2` fa sì che il player perda sempre (`player - 2×player < 0`). Dobbiamo sostituire questa formula con una **formula reattiva** che premia la strategia.

La nuova force del boss dipende da come il player ha giocato la run:
- Ogni nemico sconfitto potenzia il boss (ha "imparato" dal dungeon)
- Ogni Runa raccolta indebolisce il boss (conoscenza arcana)

Questo crea la tensione core: combattere tanto dà più forza ma un boss più duro.

---

## 📋 Acceptance Criteria

- [ ] La formula boss è `boss_force = 10 + (enemies_defeated * 2) - (runes_collected * 5)` con floor a 1
- [ ] Esiste un nuovo `ItemKind::Runa` che spawna sui floor 1–9 (non floor 10)
- [ ] Al pickup di una Runa, `RunStats.runes_collected` si incrementa
- [ ] La force del boss **non è mai visibile prima del floor 10** — il player la scopre all'arrivo
- [ ] Il boss spawna con la force calcolata dalla formula, non da `persistence.force * 2`
- [ ] Con 0 nemici e 0 rune: boss ha force 10 (minimo)
- [ ] La Runa ha un colore visivamente distinto da Ration (cyan) e Whetstone (arancio) — proposta: viola `(0.7, 0.3, 1.0)`
- [ ] `cargo test` passa senza warning

---

## 📁 File Rilevanti

| File | Ruolo |
|------|-------|
| `src/config.rs` | `RunStats` — aggiungere campo `runes_collected: u32` |
| `src/items/components.rs` | `ItemKind` enum — aggiungere variante `Runa` |
| `src/items/mod.rs` | Colore spawn Runa; skip floor 10 già presente |
| `src/tactics/movement.rs` | Pickup item — incrementare `run_stats.runes_collected` |
| `src/enemies/mod.rs` | `spawn_enemies` floor 10 — nuova formula boss_force |
| `src/map_gen/procgen.rs` | `generate_item_defs` — includere Runa nel pool |
| `src/ui/rest_screen.rs` | Nessuna modifica — la Runa non ha effetto al rest |

---

## 🧩 Contesto Tecnico

### Formula boss (attuale — BUGGED)
```rust
// src/enemies/mod.rs, ramo floor 10
let boss_force = persistence.force * 2;
// → sempre imbattibile: player_force - 2*player_force = negativo
```

### Formula boss (nuova)
```rust
let boss_force = (10 + (run_stats.enemies_defeated as i32 * 2)
    - (run_stats.runes_collected as i32 * 5))
    .max(1);
```

### RunStats attuale (config.rs)
```rust
pub struct RunStats {
    pub floors_cleared: u8,
    pub total_steps_taken: i32,
    pub enemies_defeated: u32,
    pub items_collected: u32,
}
```

### ItemKind attuale (items/components.rs)
```rust
pub enum ItemKind {
    Ration,
    Whetstone,
}
```

---

## 🔨 Implementazione Suggerita

### 1. `config.rs` — Aggiungere `runes_collected`
```rust
pub struct RunStats {
    pub floors_cleared: u8,
    pub total_steps_taken: i32,
    pub enemies_defeated: u32,
    pub items_collected: u32,
    pub runes_collected: u32,  // NEW
}
```

### 2. `items/components.rs` — Aggiungere `Runa`
```rust
pub enum ItemKind {
    Ration,
    Whetstone,
    Runa,  // NEW: indebolisce il boss di 5 force al pickup
}
```
`ItemDef` non richiede modifiche (già usa `ItemKind`).

### 3. `items/mod.rs` — Colore Runa nello spawn
```rust
let color = match def.kind {
    ItemKind::Ration    => Color::srgb(0.0, 0.9, 0.9),   // cyan
    ItemKind::Whetstone => Color::srgb(1.0, 0.6, 0.0),   // arancio
    ItemKind::Runa      => Color::srgb(0.7, 0.3, 1.0),   // viola NEW
};
```

### 4. `map_gen/procgen.rs` — Runa nel pool item
In `generate_item_defs`, il pool ora include `Runa` con probabilità 1/3:
```rust
let kind = match sub_seed(run_seed, floor, 202 + i as u64) % 3 {
    0 => ItemKind::Ration,
    1 => ItemKind::Whetstone,
    _ => ItemKind::Runa,
};
```

### 5. `tactics/movement.rs` — Pickup Runa
```rust
match *kind {
    ItemKind::Ration => {
        steps.0 += 10;
        run_stats.items_collected += 1;
    }
    ItemKind::Whetstone => {
        commands.entity(player_entity).insert(HeldItem(*kind));
        run_stats.items_collected += 1;
    }
    ItemKind::Runa => {
        run_stats.runes_collected += 1;
        run_stats.items_collected += 1;
        info!("Picked up Runa! Boss weakened by 5.");
    }
}
```

### 6. `enemies/mod.rs` — Nuova formula boss
Il sistema ha già `Res<RunStats>` disponibile. Aggiungere il parametro:
```rust
fn spawn_enemies(
    // ... parametri esistenti ...
    run_stats: Res<RunStats>,   // NEW
) {
    if config.current_floor == 10 {
        let boss_force = (10 + (run_stats.enemies_defeated as i32 * 2)
            - (run_stats.runes_collected as i32 * 5))
            .max(1);

        commands.spawn((
            Enemy,
            EnemyForce(boss_force),
            EnemyBehavior::Static,
            GridPos { x: 4, y: 4 },
            // ... resto invariato ...
        ));
        info!("Boss spawned at (4,4) with reactive force {} (enemies: {}, runes: {})",
            boss_force, run_stats.enemies_defeated, run_stats.runes_collected);
        return;
    }
    // ...
}
```

---

## ⚠️ Vincoli e Attenzioni

- **Runa non ha effetto su HeldItem/Rest**: non agisce al rest screen, solo pickup diretto.
- **Floor 10 già skippa gli item**: `items/mod.rs` ha già `if config.current_floor == 10 { return; }` — la Runa non spawna sul floor finale.
- **Bevy 0.18**: usare `Res<RunStats>` (non `ResMut`) nella query enemies, la formula è read-only.
- **floor a 1**: `boss_force.max(1)` garantisce che il boss abbia sempre almeno F1.
- **Nota sul HUD rest_screen.rs**: La Runa non è holdable (non usa `HeldItem`) — va ignorata nel rest screen.

---

## 🔗 Dipendenze

- **Dipende da**: CEN-017 (completato — `RunStats`, `WinScreen`, `ramo floor 10` già presenti)
- **Blocca**: CEN-019 (Combat formula) — indipendente ma testare insieme

---

## 🤖 Come delegare questo task a un agente

```bash
claude "$(cat tasks/CEN-018-boss-reattivo-runa.md)"$'\n\nEsegui questo task nel progetto corrente.'
```
