use bevy::prelude::*;
use bevy::state::state_scoped::DespawnOnExit;
use crate::state::GameState;
use crate::config::CenturionConfig;
use crate::map_gen::{GridPos, TileKind, RoomLayout, grid_to_world, TILE_SIZE};

pub mod components;
pub use components::{Enemy, EnemyForce, EnemyBehavior};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnemySpawn;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Room), spawn_enemies)
            .add_systems(OnEnter(GameState::Rest), cleanup_enemies);
    }
}

fn enemy_positions(floor: u8) -> Vec<(GridPos, i32)> {
    match floor {
        1 => vec![
            (GridPos { x: 3, y: 3 }, 3),
            (GridPos { x: 5, y: 5 }, 7),
            (GridPos { x: 6, y: 3 }, 4),
        ],
        2 => vec![
            (GridPos { x: 2, y: 2 }, 4),
            (GridPos { x: 5, y: 5 }, 9),
            (GridPos { x: 6, y: 3 }, 6),
            (GridPos { x: 3, y: 6 }, 5),
        ],
        _ => {
            let base = 2 + floor as i32;
            vec![
                (GridPos { x: 2, y: 2 }, base + 1),
                (GridPos { x: 5, y: 5 }, base + 4),
                (GridPos { x: 6, y: 3 }, base + 2),
            ]
        }
    }
}

fn spawn_enemies(
    mut commands: Commands,
    config: Res<CenturionConfig>,
    layout: Option<Res<RoomLayout>>,
) {
    // If RoomLayout not yet inserted, build it ourselves
    let layout_ref = if let Some(layout) = layout {
        layout.into_inner().clone()
    } else {
        crate::map_gen::build_room(config.current_floor)
    };

    let player_spawn = GridPos { x: 1, y: 1 };

    for (pos, force) in enemy_positions(config.current_floor) {
        // Validation: skip wall, skip player spawn, skip exit
        let tile = layout_ref.get(pos.x, pos.y);
        if tile == TileKind::Wall || tile == TileKind::Exit {
            warn!("Enemy at ({},{}) skipped: invalid tile", pos.x, pos.y);
            continue;
        }
        if pos.x == player_spawn.x && pos.y == player_spawn.y {
            warn!("Enemy at ({},{}) skipped: player spawn", pos.x, pos.y);
            continue;
        }

        let world_pos = grid_to_world(pos);
        commands.spawn((
            Enemy,
            EnemyForce(force),
            EnemyBehavior::Static,
            pos,
            Sprite {
                color: Color::srgb(0.7, 0.3, 0.3),
                ..default()
            },
            Transform::from_translation(world_pos.extend(0.5))
                .with_scale(Vec3::splat(TILE_SIZE * 0.875)),
            DespawnOnExit(GameState::Dead),
        ));

        info!("Enemy F{} spawned at ({},{})", force, pos.x, pos.y);
    }
}

fn cleanup_enemies(mut commands: Commands, enemies: Query<Entity, With<Enemy>>) {
    for entity in enemies.iter() {
        commands.entity(entity).despawn();
    }
}
