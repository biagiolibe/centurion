use bevy::prelude::*;
use crate::state::GameState;
use crate::player::PlayerPersistence;
use crate::config::CenturionConfig;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::Loading), loading_to_room)
            .add_systems(OnEnter(GameState::Dead), on_enter_dead)
            .add_systems(Update, debug_state);
    }
}

/// Immediately transition from Loading to Room.
/// This system runs once when entering Loading state and immediately transitions to Room.
fn loading_to_room(mut next: ResMut<NextState<GameState>>) {
    info!("Loading → Room transition");
    next.set(GameState::Room);
}

/// Handle entering Dead state: reset and transition to Loading.
fn on_enter_dead(
    mut commands: Commands,
    mut config: ResMut<CenturionConfig>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    info!("Game Over - resetting to floor 1");
    commands.remove_resource::<PlayerPersistence>();
    config.current_floor = 1;
    next_state.set(GameState::Loading);
}

/// Debug system that logs the current game state every frame.
/// This is temporary and can be removed after initial testing (CEN-003+).
#[allow(dead_code)]
fn debug_state(_state: Res<State<GameState>>) {
    // Uncomment to see state spam in logs during development
    // info!("GameState: {:?}", _state.get());
}
