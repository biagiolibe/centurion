# Task CEN-026 — HUD: Info Boss Dinamico + Rune

> **ID**: `CEN-026`
> **Categoria**: UI
> **Priorità**: 🟡 P2
> **Stima**: ~1h
> **Assegnato a**: Claude CLI
> **Sessione**: 2026-05-18

---

## 🎯 Obiettivo

Il player non ha feedback visivo su come le sue azioni influenzano il boss finale. Vogliamo mostrare nell'HUD (e nel rest screen) la proiezione della forza boss e i fattori che la compongono (nemici sconfitti e rune raccolte), così il player può fare scelte strategiche consapevoli.

---

## 📋 Acceptance Criteria

- [ ] L'HUD in-game mostra tre nuove righe sotto le esistenti (STEPS/FORCE/FLOOR): nemici sconfitti, rune raccolte, forza boss proiettata
- [ ] La forza boss proiettata si aggiorna in tempo reale dopo ogni combattimento e ogni pickup di Runa
- [ ] Il rest screen mostra una riga "Boss al floor 10: ~Fxx" con i fattori correnti
- [ ] I colori delle nuove righe HUD sono visivamente distinti: viola per rune, rosso per boss
- [ ] `cargo test` passa senza warning

---

## 📁 File Rilevanti

| File | Ruolo |
|------|-------|
| `src/ui/hud.rs` | HUD in-game — aggiungere componenti e righe; aggiornare sistema |
| `src/ui/rest_screen.rs` | Rest screen — aggiungere sezione info boss |
| `src/config.rs` | `RunStats` — `enemies_defeated`, `runes_collected` già presenti |

---

## 🧩 Contesto Tecnico

### HUD attuale (`src/ui/hud.rs`)

```
STEPS: 87
FORCE: 8
FLOOR: 3
```

Componenti esistenti: `HudStep`, `HudForce`, `HudFloor`.
Il sistema `update_steps_display` legge `Res<PlayerStats>` e `Res<CenturionConfig>`.

### Layout HUD target

```
STEPS: 87
FORCE: 8
FLOOR: 3
─────────────
⚔  NEMICI: 4
✦  RUNE:   1
☠  BOSS:  ~F13
```

### Formula boss (già implementata in `src/enemies/mod.rs`)
```rust
let boss_force = (10 + enemies_defeated as i32 * 2 - runes_collected as i32 * 5).max(1);
```

### Rest screen attuale (`src/ui/rest_screen.rs`)

Dopo la riga "Steps: X  Force: Y" aggiungere:
```
Boss al floor 10: ~F13  (⚔ 4 nemici | ✦ 1 runa)
ogni runa aggiuntiva: -5 | ogni nemico: +2
```

---

## 🔨 Implementazione Suggerita

### 1. `src/ui/hud.rs` — Nuovi componenti

```rust
#[derive(Component)]
pub struct HudEnemies;

#[derive(Component)]
pub struct HudRunes;

#[derive(Component)]
pub struct HudBoss;
```

### 2. `src/ui/hud.rs` — `spawn_hud`

Dopo le tre righe esistenti, aggiungere un separatore e le tre nuove righe:

```rust
parent.spawn((
    Text::new("─────────────"),
    TextFont { font_size: 18.0, ..default() },
    TextColor(Color::srgb(0.3, 0.3, 0.3)),
));
parent.spawn((
    Text::new("⚔  NEMICI: 0"),
    TextFont { font_size: 20.0, ..default() },
    TextColor(Color::srgb(0.9, 0.9, 0.9)),
    HudEnemies,
));
parent.spawn((
    Text::new("✦  RUNE:   0"),
    TextFont { font_size: 20.0, ..default() },
    TextColor(Color::srgb(0.7, 0.3, 1.0)),   // viola Runa
    HudRunes,
));
parent.spawn((
    Text::new("☠  BOSS:  ~F10"),
    TextFont { font_size: 20.0, ..default() },
    TextColor(Color::srgb(0.9, 0.3, 0.3)),   // rosso boss
    HudBoss,
));
```

### 3. `src/ui/hud.rs` — `update_steps_display`

Aggiungere `Res<RunStats>` ai parametri ed estendere il `ParamSet` con le tre nuove query:

```rust
let boss_est = (10 + run_stats.enemies_defeated as i32 * 2
    - run_stats.runes_collected as i32 * 5)
    .max(1);

// HudEnemies
**text = format!("⚔  NEMICI: {}", run_stats.enemies_defeated);
// HudRunes
**text = format!("✦  RUNE:   {}", run_stats.runes_collected);
// HudBoss
**text = format!("☠  BOSS:  ~F{}", boss_est);
```

### 4. `src/ui/rest_screen.rs` — `spawn_rest_screen`

Aggiungere `run_stats: Res<RunStats>` ai parametri e inserire dopo la riga steps/force:

```rust
let boss_est = (10 + run_stats.enemies_defeated as i32 * 2
    - run_stats.runes_collected as i32 * 5)
    .max(1);

parent.spawn((
    Text::new(format!(
        "Boss al floor 10: ~F{}  (⚔ {} nemici | ✦ {} rune)",
        boss_est, run_stats.enemies_defeated, run_stats.runes_collected
    )),
    TextFont { font_size: 18.0, ..default() },
    TextColor(Color::srgb(1.0, 0.5, 0.2)),
));
parent.spawn((
    Text::new("ogni runa: -5 forza boss  |  ogni nemico: +2 forza boss"),
    TextFont { font_size: 16.0, ..default() },
    TextColor(Color::srgb(0.5, 0.5, 0.5)),
));
```

---

## ⚠️ Vincoli e Attenzioni

- **`ParamSet` in `update_steps_display`**: già usa `ParamSet` con 3 query — estendere a 6 (o ristrutturare con query separate se Bevy non supporta ParamSet > 8)
- **No emoji**: Bevy usa FiraMono built-in che non supporta emoji — usare label ASCII puri (`NEMICI:`, `RUNE:`, `BOSS~:`)
- **`RunStats` non è un evento**: si aggiorna a ogni combattimento e pickup, quindi `update_steps_display` (che gira in Update) la legge sempre fresca
- Non aggiungere la proiezione boss nel Dead screen o Win screen — già ci sono altre info

---

## 🔗 Dipendenze

- **Dipende da**: CEN-018 (completato — `RunStats.runes_collected` e formula boss esistenti)
- **Blocca**: nessuno

---

## 🤖 Come delegare questo task a un agente

```bash
claude "$(cat tasks/CEN-026-hud-boss-dinamico.md)"$'\n\nEsegui questo task nel progetto corrente.'
```
