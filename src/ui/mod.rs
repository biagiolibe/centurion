use bevy::prelude::*;
use crate::state::GameState;

pub mod hud;
pub mod rest_screen;
pub mod dead_screen;
pub use hud::{HudRoot, HudStep, HudForce, HudFloor, spawn_hud, update_steps_display};
pub use rest_screen::RestScreenPlugin;
pub use dead_screen::DeadScreenPlugin;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RestScreenPlugin, DeadScreenPlugin))
            .add_systems(OnEnter(GameState::Room), spawn_hud)
            .add_systems(Update, update_steps_display);
    }
}
