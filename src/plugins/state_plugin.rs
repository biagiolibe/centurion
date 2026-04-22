use bevy::prelude::*;
use crate::state::GameState;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::Loading), loading_to_room)
            .add_systems(Update, debug_state);
    }
}

/// Immediately transition from Loading to Room.
fn loading_to_room(mut next: ResMut<NextState<GameState>>) {
    info!("Loading → Room transition");
    next.set(GameState::Room);
}

/// Debug system that logs the current game state every frame.
#[allow(dead_code)]
fn debug_state(_state: Res<State<GameState>>) {
    // Uncomment to see state spam in logs during development
    // info!("GameState: {:?}", _state.get());
}
