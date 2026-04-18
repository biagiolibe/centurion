use bevy::prelude::*;

/// Tag component for the player entity.
#[derive(Component)]
pub struct Player;

/// Number of movement steps the player has remaining this floor.
#[derive(Component)]
pub struct CurrentSteps(pub i32);

/// Combat power: used to determine fight outcomes and damage taken.
#[derive(Component)]
pub struct Force(pub i32);

/// Synced resource for HUD display — mirrors CurrentSteps and Force components.
#[derive(Resource)]
pub struct PlayerStats {
    pub steps: i32,
    pub force: i32,
}
