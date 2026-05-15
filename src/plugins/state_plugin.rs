use bevy::prelude::*;
use rand::Rng;
use crate::state::GameState;
use crate::config::{RunSeed, RunStats};

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::Loading), loading_to_room)
            .add_systems(Update, debug_state);
    }
}

/// Generate a new run seed and immediately transition from Loading to Room.
fn loading_to_room(mut commands: Commands, mut next: ResMut<NextState<GameState>>) {
    let seed = rand::thread_rng().gen::<u64>();
    commands.insert_resource(RunSeed(seed));
    commands.insert_resource(RunStats::default());
    info!("Loading → Room transition; New run seed: 0x{:016x}", seed);
    next.set(GameState::Room);
}

/// Debug system that logs the current game state every frame.
#[allow(dead_code)]
fn debug_state(_state: Res<State<GameState>>) {
    // Uncomment to see state spam in logs during development
    // info!("GameState: {:?}", _state.get());
}
