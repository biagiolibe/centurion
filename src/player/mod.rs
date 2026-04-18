use bevy::prelude::*;
use bevy::state::state_scoped::DespawnOnExit;

pub mod components;
pub use components::{Player, CurrentSteps, Force, PlayerStats};

use crate::state::GameState;
use crate::map_gen::{GridPos, grid_to_world, TILE_SIZE};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Room), spawn_player)
            .add_systems(Update, sync_player_stats);
    }
}

/// Spawn the player entity at the start of a room.
/// Player starts at grid position (1, 1) with 100 steps and 5 force.
fn spawn_player(mut commands: Commands) {
    let start_pos = GridPos { x: 1, y: 1 };
    let world_pos = grid_to_world(start_pos);

    commands.spawn((
        Player,
        CurrentSteps(100),
        Force(5),
        start_pos,
        Sprite {
            color: Color::WHITE,
            ..default()
        },
        Transform::from_translation(world_pos.extend(1.0))
            .with_scale(Vec3::splat(TILE_SIZE * 0.875)),
        DespawnOnExit(GameState::Room),
    ));

    commands.insert_resource(PlayerStats {
        steps: 100,
        force: 5,
    });

    info!("Player spawned at grid position (1, 1)");
}

/// Sync player component data to PlayerStats resource for HUD display.
fn sync_player_stats(
    player_q: Query<(&CurrentSteps, &Force), With<Player>>,
    mut stats: ResMut<PlayerStats>,
) {
    if let Ok((steps, force)) = player_q.single() {
        stats.steps = steps.0;
        stats.force = force.0;
    }
}
