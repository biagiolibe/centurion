# Centurion: Gameplay Overhaul — Overview & Roadmap

**Status**: In Planning (CEN-014 to CEN-017)  
**Started**: 2026-04-28  
**Target**: Add tactical depth, resource management, and run variety to post-MVP gameplay

---

## 🎯 Vision

The MVP (Tactical Core) is feature-complete but lacks depth: every run is identical, enemies never move, combat is a passive one-transaction mechanic, and the player makes zero meaningful decisions. This overhaul adds three dimensions of depth across four independent but coherent phases.

---

## 📊 The Four Phases

### **Phase 1: Tactical Positioning (CEN-014)**

**Enemy AI — Movimento Dinamico (Patrol & Guard)**

Enemies become agents that react to the player's position. Three behaviors:

- **`Static`** — No movement (existing)
- **`Patrol { axis, direction }`** — Predictable movement along an axis, bounces off walls
- **`Guard { alerted }`** — Dormant until player enters 3-tile line-of-sight, then advances

**Depth added:**
- Player must plan the sequence of enemy engagements
- Detour cost (extra steps) vs. combat cost (force loss) becomes a real decision
- Moving enemies create spatial puzzles

**Files modified/created:**
- `enemies/components.rs` — add `Patrol`, `Guard` variants
- `enemies/movement.rs` (new) — `advance_enemies` system
- `enemies/mod.rs` — integration

**Gameplay impact:** A 3×3 room with three enemies suddenly has 6 different strategies depending on which enemy you engage first.

---

### **Phase 2: Procedural Generation (CEN-015)**

**Run Variety & Deterministic Seeding**

Every run gets a unique `u64` seed. All layout decisions (exit position, internal pillars, enemy placement) derive deterministically from this seed. Runs are varied but reproducible.

**Depth added:**
- No two runs are memorizable
- Internal walls (pillars) force dynamic routing instead of rote paths
- Unexpected enemy configurations require fresh spatial reasoning each floor

**Mechanics:**
- `RunSeed(u64)` resource created on Loading, destroyed on restart
- Sub-seed function: `sub_seed(run_seed, floor, salt) → u64` for deterministic derivation
- Exit placement: random interior border position (8 choices)
- Pillars: 0–3 internal walls placed and validated for connectivity via BFS
- Enemies: 2–4 per floor, positions and forces procedurally determined

**Files modified/created:**
- `map_gen/procgen.rs` (new) — all generation functions
- `config.rs` — `RunSeed` resource
- `map_gen/room.rs` — integrated with seed
- `enemies/mod.rs` — integrated with seed

**Gameplay impact:** Replay value. Sharing seeds with friends for competitive scores.

---

### **Phase 3: Resource Management (CEN-016)**

**Items & Rest Choices — Meaningful Decisions Between Floors**

Two new mechanics:

1. **Items** (collected on floors):
   - `Ration` (cyan) → immediate +10 steps
   - `Whetstone` (orange) → held until rest, grants +1 force bonus to tier recovery

2. **Rest Choices** (3 options, player selects one):
   - **Descend Fast**: +20 steps, standard tier recovery
   - **Extended Rest**: +40 steps, no tier recovery
   - **Intensive Training**: +0 steps, double tier recovery (skip one tier rank)

**Depth added:**
- Items reward exploration (detour cost) vs. beeline trade-off
- Rest choice depends on arrival state (steps critical? force low?)
- Whetstone creates "hold for when needed" tension
- Tier math becomes a skill: understanding when double-tier is worth the step penalty

**Files modified/created:**
- `items/components.rs` + `items/mod.rs` (new) — item system
- `ui/rest_screen.rs` — interactive rest choices (1/2/3 keys)
- `tactics/movement.rs` — item pickup branch
- `player/components.rs` — add `held_item` to `PlayerPersistence`

**Gameplay impact:** Between-floor decision point that shapes next floor's strategy. F4 + Whetstone → F10 is more impactful than F4 → F5.

---

### **Phase 4: Endgame & Win Condition (CEN-017)**

**Defined Run Arc with Score System**

A run now has a goal: reach Floor 10 and defeat the Boss. Score system incentivizes optimization.

**Mechanics:**
- **Floor 10 Boss**: Single enemy at center (4,4) with `force = player_entry_force * 2`
- **Win Condition**: Defeat boss → `WinScreen` (not `Dead`)
- **Score Formula**: `(floors_cleared * 100) + (steps_remaining * 2) + (force * 10) - (total_steps_taken / 2)`
  - Rewards: multi-floor progression, step efficiency, force preservation
  - Penalizes: wasted movement

**Screens:**
- **WinScreen**: Shows floors cleared, final force, score, seed, restart prompt
- **DeadScreen** (updated): Adds score, seed display

**Depth added:**
- Defined arc instead of endless treadmill
- Score allows comparing runs and optimizing strategies
- Boss force scaling ensures efficiency matters

**Files modified/created:**
- `state.rs` — new `GameState::WinScreen`
- `ui/win_screen.rs` (new) — victory screen
- `ui/dead_screen.rs` — add score, seed
- `config.rs` — `RunStats` resource
- `resolver/mod.rs` — check floor 10 endgame condition

**Gameplay impact:** "Beat floor 10 with score > 1500" becomes a meaningful goal with emergent strategies.

---

## 🔗 Dependencies & Implementation Order

```
Phase 1 (CEN-014)
  ├─ Requires: MVP complete (CEN-013 ✅)
  └─ Delivers: Enemy movement, positioning tactics

Phase 2 (CEN-015)
  ├─ Requires: CEN-014 (enemy behaviors exist)
  └─ Delivers: Seed-based generation, pillar placement, run variety

Phase 1 + 2 together = Milestone 1 (Core tactical loop with variety)

Phase 3 (CEN-016)
  ├─ Requires: CEN-015 (seeds for item placement)
  └─ Delivers: Items, interactive rest, resource choices

Phase 4 (CEN-017)
  ├─ Requires: CEN-016 (items, stats tracking)
  └─ Delivers: Win condition, score system, endgame
```

**Recommended sequence:**
1. CEN-014 + CEN-015 together (~5 days) → Tactical + Variety
2. CEN-016 (~2–3 days) → Resource layer
3. CEN-017 (~1–2 days) → Endgame wrap

Total: ~8–10 days for full overhaul.

---

## 🎮 Example Run (Post-Overhaul)

```
Start run with seed 0x1a2b3c4d
┌─────────────────────────────────────────────────────┐
│ Floor 1:                                            │
│ - Layout: pillar at (3,2), exit at (6,4)           │
│ - Enemies: Guard at (2,3), Patrol at (5,5)         │
│ - Pick up Ration at (4,6) → +10 steps (60 total)   │
│ - Engage Guard, win → 60 remaining steps            │
│ - Reach exit → Rest                                 │
│                                                     │
│ Rest Choice 1: Descend Fast                         │
│ - Entry: 60 steps, F8                              │
│ - Next: 80 steps, F10                              │
├─────────────────────────────────────────────────────┤
│ Floor 2:                                            │
│ - Layout: pillars at (2,2) and (5,5), exit (1,5)   │
│ - Enemies: 4x Guard and Patrol mix (harder)        │
│ - Route around to avoid Guard's LOS                │
│ - Combat vs Patrol → win, but lose 3 force (F7)    │
│ - Pick up Whetstone at (3,4) → hold                │
│ - Reach exit with 35 steps → Rest                  │
│                                                     │
│ Rest Choice 3: Intensive Training                  │
│ - Entry: 35 steps, F7 (holding Whetstone)          │
│ - Adjusted: F8 → F15 (double tier with Whetstone) │
│ - Next: 35 steps, F15 (strong for floor 3)        │
├─────────────────────────────────────────────────────┤
│ ... continue through floors 3–9 ...                │
├─────────────────────────────────────────────────────┤
│ Floor 10 Boss:                                      │
│ - Arena: (4,4) center, no pillars, no minions      │
│ - Boss Force = 15 * 2 = F30                        │
│ - Player has F15 → Combat: new_force = 15 - 30 = -15 → LOSE
│                                                     │
│ Death: Floor 9 cleared, 40 steps wasted            │
│ Score = (9*100) + (45*2) + (15*10) - (155/2)      │
│       = 900 + 90 + 150 - 77 = 1063                │
│                                                     │
│ Seed: 0x1a2b3c4d (saved for retry)                │
└─────────────────────────────────────────────────────┘
```

In this run, the player:
- Optimized pathing through phase 1 tactics
- Explored slightly in phase 1 for Ration (timing trade-off)
- Held Whetstone for phase 2 to spike force (resource management)
- Nearly beat the game but miscalculated force needed for the boss
- Score is recorded; they can retry with same seed to beat their previous attempt

---

## 📈 Gameplay Arc

```
Pre-overhaul:                  Post-overhaul:
Exploration → Combat → Rest    Exploration → Combat → Rest → Boss
    ↓            ↓              ↓            ↓         ↓
   Static     Transaction    Tactical   Strategy   Climax
   Layout     (deterministic) Routing   (choice)   (efficiency test)
    ↓            ↓              ↓           ↓         ↓
   Loop        No options    Positioning  Items    Win/Lose
                                          Choice   with Score
```

The loop gains **meaningful decisions** at every stage, and gameplay evolves from "navigate → fight → repeat" to "optimize route → manage resources → face climax."

---

## ✅ Verification Checklist (Per Phase)

### Phase 1 (CEN-014)
- [ ] Guard doesn't move until player in LOS
- [ ] Patrol bounces off walls correctly
- [ ] Enemy→player CombatIntent resolves correctly
- [ ] 20 floors no crash

### Phase 2 (CEN-015)
- [ ] Identical seeds produce identical layouts
- [ ] Different seeds produce different layouts
- [ ] Path from spawn to exit always exists (BFS validation)
- [ ] Seed display on dead screen

### Phase 3 (CEN-016)
- [ ] Ration pickup adds 10 steps immediately
- [ ] Whetstone held and persists to rest
- [ ] Rest choices modify stats correctly
- [ ] Whetstone bonus (+1 force tier) applied correctly

### Phase 4 (CEN-017)
- [ ] Floor 10 spawns boss with `force = entry_force * 2`
- [ ] Boss defeat triggers WinScreen
- [ ] Score calculated correctly
- [ ] Seed displayed on win/dead screens
- [ ] R restart cleans up resources

---

## 📝 Notes

- **Determinism guarantee**: No RNG in `resolver/combat.rs`; only used for initial `RunSeed` in Loading state
- **Modularity**: Each phase can be developed independently; earlier phases unaffected by later ones
- **Bevy 0.18 compliance**: All systems use message passing, `DespawnOnExit`, state ordering, etc.
- **Backwards compatibility**: Existing MVP mechanics (movement, combat, HUD) untouched; additions are purely additive

---

*Document created: 2026-04-28 | Task files: CEN-014, CEN-015, CEN-016, CEN-017*
