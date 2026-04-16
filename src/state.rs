use bevy::prelude::*;

/// Game state machine that governs the entire game flow.
///
/// Valid transitions:
/// - Loading  → Room
/// - Room     → CombatEvent | Rest | Dead
/// - CombatEvent → Room
/// - Rest     → Room
/// - Dead     → Loading
///
/// PATTERN: Use `DespawnOnExit<GameState>` on all in-game entities.
/// When exiting a state, Bevy automatically despawns all entities carrying
/// `DespawnOnExit(that_state)`. This ensures clean state transitions with no
/// lingering entities from the previous state.
///
/// Example:
/// ```ignore
/// commands.spawn((
///     MyComponent,
///     DespawnOnExit(GameState::Room),
/// ));
/// ```
#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    /// Initial state: loading configuration and preparing the first room.
    #[default]
    Loading,
    /// Main gameplay state: player movement, enemies, combat.
    Room,
    /// Brief state for combat animation (flash).
    CombatEvent,
    /// Between-floor state: showing stats and waiting for player to descend.
    Rest,
    /// Game over state: showing final run statistics.
    Dead,
}
