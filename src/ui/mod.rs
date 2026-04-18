use bevy::prelude::*;
use crate::state::GameState;

pub mod hud;
pub use hud::{HudRoot, HudStep, HudForce, HudFloor, spawn_hud, update_steps_display};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Room), spawn_hud)
            .add_systems(Update, update_steps_display);
    }
}
