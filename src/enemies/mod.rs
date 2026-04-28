use bevy::prelude::*;
use bevy::state::state_scoped::DespawnOnExit;
use crate::state::GameState;
use crate::config::CenturionConfig;
use crate::map_gen::{GridPos, TileKind, RoomLayout, grid_to_world, TILE_SIZE};

pub mod components;
pub mod movement;

pub use components::{Enemy, EnemyForce, EnemyBehavior, EnemyDef, Axis};
pub use movement::advance_enemies;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnemySpawn;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnemyTurnSet;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        use crate::resolver::apply_victory_movement;

        app.add_systems(OnEnter(GameState::Room), spawn_enemies)
            .add_systems(OnEnter(GameState::Rest), cleanup_enemies)
            .add_systems(Update, advance_enemies
                .after(apply_victory_movement)
                .run_if(in_state(GameState::Room)));
    }
}

fn enemy_positions(floor: u8) -> Vec<EnemyDef> {
    match floor {
        1 => vec![
            EnemyDef { pos: GridPos { x: 3, y: 3 }, force: 3, behavior: EnemyBehavior::Guard { alerted: false } },
            EnemyDef { pos: GridPos { x: 5, y: 5 }, force: 7, behavior: EnemyBehavior::Static },
            EnemyDef { pos: GridPos { x: 6, y: 3 }, force: 4, behavior: EnemyBehavior::Static },
        ],
        2 => vec![
            EnemyDef { pos: GridPos { x: 2, y: 2 }, force: 4, behavior: EnemyBehavior::Patrol { axis: Axis::Horizontal, direction: 1 } },
            EnemyDef { pos: GridPos { x: 5, y: 5 }, force: 9, behavior: EnemyBehavior::Guard { alerted: false } },
            EnemyDef { pos: GridPos { x: 6, y: 3 }, force: 6, behavior: EnemyBehavior::Static },
            EnemyDef { pos: GridPos { x: 3, y: 6 }, force: 5, behavior: EnemyBehavior::Static },
        ],
        _ => {
            let base = 2 + floor as i32;
            vec![
                EnemyDef { pos: GridPos { x: 2, y: 2 }, force: base + 1, behavior: EnemyBehavior::Patrol { axis: Axis::Vertical, direction: 1 } },
                EnemyDef { pos: GridPos { x: 5, y: 5 }, force: base + 4, behavior: EnemyBehavior::Guard { alerted: false } },
                EnemyDef { pos: GridPos { x: 6, y: 3 }, force: base + 2, behavior: EnemyBehavior::Static },
            ]
        }
    }
}

fn spawn_enemies(
    mut commands: Commands,
    config: Res<CenturionConfig>,
    layout: Option<Res<RoomLayout>>,
    existing: Query<(), With<Enemy>>,
) {
    if !existing.is_empty() {
        return;
    }

    // If RoomLayout not yet inserted, build it ourselves
    let layout_ref = if let Some(layout) = layout {
        layout.into_inner().clone()
    } else {
        crate::map_gen::build_room(config.current_floor)
    };

    let player_spawn = GridPos { x: 1, y: 1 };

    for def in enemy_positions(config.current_floor) {
        // Validation: skip wall, skip player spawn, skip exit
        let tile = layout_ref.get(def.pos.x, def.pos.y);
        if tile == TileKind::Wall || tile == TileKind::Exit {
            warn!("Enemy at ({},{}) skipped: invalid tile", def.pos.x, def.pos.y);
            continue;
        }
        if def.pos.x == player_spawn.x && def.pos.y == player_spawn.y {
            warn!("Enemy at ({},{}) skipped: player spawn", def.pos.x, def.pos.y);
            continue;
        }

        let world_pos = grid_to_world(def.pos);
        let behavior_name = match def.behavior {
            EnemyBehavior::Static => "Static",
            EnemyBehavior::Patrol { .. } => "Patrol",
            EnemyBehavior::Guard { .. } => "Guard",
        };

        commands.spawn((
            Enemy,
            EnemyForce(def.force),
            def.behavior,
            def.pos,
            Sprite {
                color: Color::srgb(0.7, 0.3, 0.3),
                ..default()
            },
            Transform::from_translation(world_pos.extend(0.5))
                .with_scale(Vec3::splat(TILE_SIZE * 0.875)),
            DespawnOnExit(GameState::Dead),
        ));

        info!("Enemy F{} ({}) spawned at ({},{})", def.force, behavior_name, def.pos.x, def.pos.y);
    }
}

fn cleanup_enemies(mut commands: Commands, enemies: Query<Entity, With<Enemy>>) {
    for entity in enemies.iter() {
        commands.entity(entity).despawn();
    }
}
