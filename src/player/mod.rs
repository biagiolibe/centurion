use bevy::prelude::*;
use bevy::state::state_scoped::DespawnOnExit;

pub mod components;
pub use components::{Player, CurrentSteps, Force, PlayerStats, PlayerPersistence};

use crate::state::GameState;
use crate::map_gen::{GridPos, grid_to_world, TILE_SIZE};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Room), spawn_player)
            .add_systems(Update, sync_player_stats)
            .add_systems(OnEnter(GameState::Rest), on_enter_rest);
    }
}

/// Spawn the player entity at the start of a room.
/// Player starts at grid position (1, 1) with values from PlayerPersistence (if available).
fn spawn_player(
    mut commands: Commands,
    persistence: Option<Res<PlayerPersistence>>,
) {
    let start_pos = GridPos { x: 1, y: 1 };
    let world_pos = grid_to_world(start_pos);

    let (steps, force) = match persistence {
        Some(ref p) => (p.steps, p.force),
        None => (100, 5),
    };

    commands.spawn((
        Player,
        CurrentSteps(steps),
        Force(force),
        start_pos,
        Sprite {
            color: Color::WHITE,
            ..default()
        },
        Transform::from_translation(world_pos.extend(1.0))
            .with_scale(Vec3::splat(TILE_SIZE * 0.875)),
        DespawnOnExit(GameState::Dead),
    ));

    commands.insert_resource(PlayerStats {
        steps,
        force,
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

/// Save player state before exiting a floor, then despawn the player entity.
fn on_enter_rest(
    mut commands: Commands,
    player_q: Query<(Entity, &CurrentSteps, &Force), With<Player>>,
) {
    if let Ok((entity, steps, force)) = player_q.single() {
        commands.insert_resource(PlayerPersistence {
            steps: steps.0,
            force: force.0,
        });
        commands.entity(entity).despawn();
    }
}
