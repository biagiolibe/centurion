use bevy::prelude::*;
use bevy::state::state_scoped::DespawnOnExit;
use crate::state::GameState;
use crate::config::{CenturionConfig, RunSeed};

pub mod room;
pub mod procgen;
pub use room::{GridPos, TileKind, RoomLayout, build_room, grid_to_world, world_to_grid, TILE_SIZE};
pub use procgen::{build_room_proc, generate_enemy_defs};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MapGenSet;

#[derive(Resource, Clone, Copy, Debug)]
pub struct CurrentExitPos(pub GridPos);

pub struct MapGenPlugin;

impl Plugin for MapGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Room), spawn_room.in_set(MapGenSet));
    }
}

/// Spawn all tiles for the current room.
/// This system runs when entering Room state and creates a 8x8 grid of tile entities.
fn spawn_room(mut commands: Commands, config: Res<CenturionConfig>, run_seed: Res<RunSeed>) {
    // Generate room layout procedurally for current floor
    let (room_layout, exit_pos) = build_room_proc(config.current_floor, run_seed.0);

    // Insert exit position for enemies to use
    commands.insert_resource(CurrentExitPos(exit_pos));

    // Spawn all tiles
    for y in 0..8 {
        for x in 0..8 {
            let pos = GridPos { x, y };
            let tile_kind = room_layout.get(x, y);
            let world_pos = grid_to_world(pos);

            // Determine color based on tile type
            let color = match tile_kind {
                TileKind::Wall => Color::srgb(0.3, 0.3, 0.3),
                TileKind::Floor => Color::srgb(0.15, 0.15, 0.15),
                TileKind::Exit => Color::srgb(0.9, 0.9, 0.2),
            };

            commands.spawn((
                Sprite {
                    color,
                    ..default()
                },
                Transform::from_translation(world_pos.extend(0.0))
                    .with_scale(Vec3::splat(TILE_SIZE)),
                pos,
                tile_kind,
                DespawnOnExit(GameState::Room),
            ));
        }
    }

    // Insert room layout as resource
    commands.insert_resource(room_layout);

    info!(
        "Spawned 8x8 room for floor {} with exit at ({}, {})",
        config.current_floor, exit_pos.x, exit_pos.y
    );
}
