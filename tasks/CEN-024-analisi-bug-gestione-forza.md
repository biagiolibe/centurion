# Task CEN-024 — Analisi e Fix Bug Gestione Forza

> **ID**: `CEN-024`
> **Categoria**: Bugfix / Analisi
> **Priorità**: 🔴 P1
> **Stima**: ~1.5h
> **Assegnato a**: Claude CLI
> **Sessione**: 2026-05-18

---

## 🎯 Obiettivo

Analizzare e correggere i bug identificati nella gestione della forza del player durante la run. Il flusso forza→persistenza→spwawn è complesso e ha almeno un bug confermato (Whetstone permanente) e altri scenari da verificare.

---

## 📋 Acceptance Criteria

- [ ] Il bonus Whetstone (+1 force) viene applicato **una sola volta** per Whetstone raccolto, non ad ogni floor exit
- [ ] Dopo l'applicazione del bonus, `PlayerPersistence.held_item` viene azzerato (o il kind cambia a non-bonusable)
- [ ] Verificare: la forza del player non può scendere sotto 1 per combattimenti normali su floor 1–9 (o documentare se è intenzionale)
- [ ] Verificare: `CurrentPlayerForce` è sempre sincronizzata correttamente prima di `resolve_combat` — aggiungere ordinamento esplicito se mancante
- [ ] `cargo test` passa senza warning

---

## 📁 File Rilevanti

| File | Ruolo |
|------|-------|
| `src/player/mod.rs` | `on_enter_rest()` — salva persistenza con whetstone bonus |
| `src/player/components.rs` | `PlayerPersistence` — struct cross-floor |
| `src/resolver/mod.rs` | `sync_player_force()`, `resolve_combat()` — ordine sistemi |
| `src/ui/rest_screen.rs` | Gestione scelte rest — verifica se cleared held_item |

---

## 🧩 Contesto Tecnico

### Bug #1 — Whetstone permanente (CONFERMATO)

In `src/player/mod.rs`, `on_enter_rest()`:

```rust
fn on_enter_rest(...) {
    let whetstone_bonus = if held_item.map(|h| h.0) == Some(ItemKind::Whetstone) { 1 } else { 0 };
    commands.insert_resource(PlayerPersistence {
        steps: steps.0,
        force: force.0 + whetstone_bonus,   // bonus applicato
        held_item: held_item.map(|h| h.0),  // ← BUG: Whetstone rimane in held_item!
    });
```

Flusso errato:
1. Floor N exit: `force += 1`, `held_item = Some(Whetstone)` salvato in persistence
2. Floor N+1 spawn: player ha `Force(force+1)` E `HeldItem(Whetstone)`
3. Floor N+1 exit: `on_enter_rest` vede ancora `held_item = Some(Whetstone)` → applica +1 di nuovo
4. Ripete ogni floor → Whetstone dà forza illimitata

**Fix atteso**:
```rust
held_item: if whetstone_bonus > 0 { None } else { held_item.map(|h| h.0) },
```

### Bug #2 — Ordinamento `sync_player_force` vs `resolve_combat` (DA VERIFICARE)

In `src/resolver/mod.rs`:
```rust
// Questi due sistemi sono entrambi `.after(MovementSet)` senza ordine tra loro:
.add_systems(Update, sync_player_force.after(MovementSet))
.add_systems(Update, (resolve_combat, apply_victory_movement).chain().after(MovementSet))
```

`resolve_combat` legge `Res<CurrentPlayerForce>` che viene aggiornato da `sync_player_force`. Se Bevy esegue `resolve_combat` prima di `sync_player_force` nello stesso frame, legge un valore stale.

**Fix atteso**: aggiungere `sync_player_force.before(resolve_combat)` o riunire in chain.

### Bug #3 — Forza a 0 dopo combattimento pari (DA VERIFICARE)

Il player può arrivare a `Force(0)` dopo aver ucciso un nemico con stessa forza (CEN-019 affronta il caso player-attacca-in-parità; verificare anche il caso nemico-attacca-in-parità in `resolve_combat` ramo `attacker_is_player = false`).

### Nota su CEN-019
CEN-019 (Combat Formula — Tie Survival) affronta il caso `resolve(player_force, enemy_force)` con forze uguali → `new_force = 0 → PlayerDies`. Coordinare con questo task per evitare sovrapposizioni.

---

## 🔨 Implementazione Suggerita

1. **Leggere `src/ui/rest_screen.rs`** — verificare se la scelta rest already clears `held_item` dalla persistenza (se sì, Bug #1 è già gestito lì)
2. **Correggere Bug #1** in `on_enter_rest` se confermato
3. **Aggiungere ordinamento esplicito** `sync_player_force` → `resolve_combat` in `ResolverPlugin::build()`
4. **Aggiungere test unitario** per il flusso Whetstone: raccogli whetstone, simula due floor exit, verifica che bonus sia +1 totale non +2

---

## ⚠️ Vincoli e Attenzioni

- **Bevy 0.18**: ordinamento sistemi via `.before()` / `.after()` sulla system set o direttamente tra sistemi
- **Non modificare `resolver/combat.rs`** — quella logica è coperta da CEN-019
- **Runa non è HeldItem** — non è affetta da questi bug

---

## 🔗 Dipendenze

- **Dipende da**: CEN-018 (completato)
- **Coordinare con**: CEN-019 (forza a zero in parità)
- **Blocca**: nessuno direttamente, ma impatta il bilanciamento generale

---

## 🤖 Come delegare questo task a un agente

```bash
claude "$(cat tasks/CEN-024-analisi-bug-gestione-forza.md)"$'\n\nEsegui questo task nel progetto corrente.'
```
