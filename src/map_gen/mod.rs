use bevy::prelude::*;
use bevy::state::state_scoped::DespawnOnExit;
use crate::state::GameState;
use crate::config::CenturionConfig;

pub mod room;
pub use room::{GridPos, TileKind, RoomLayout, build_room, grid_to_world, world_to_grid, TILE_SIZE};

pub struct MapGenPlugin;

impl Plugin for MapGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Room), spawn_room);
    }
}

/// Spawn all tiles for the current room.
/// This system runs when entering Room state and creates a 8x8 grid of tile entities.
fn spawn_room(mut commands: Commands, config: Res<CenturionConfig>) {
    // Generate room layout for current floor
    let room_layout = build_room(config.current_floor);

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
        "Spawned 8x8 room for floor {} with exit at (4, {})",
        config.current_floor,
        4 + ((config.current_floor - 1) % 3)
    );
}
