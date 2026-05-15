use bevy::prelude::*;
use bevy::state::state_scoped::DespawnOnExit;
use crate::state::GameState;
use crate::config::{CenturionConfig, RunSeed};
use crate::player::PlayerPersistence;
use crate::map_gen::{GridPos, RoomLayout, grid_to_world, TILE_SIZE, MapGenSet, CurrentExitPos, build_room_proc, generate_enemy_defs};

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

        app.configure_sets(OnEnter(GameState::Room), EnemySpawn.after(MapGenSet))
            .add_systems(OnEnter(GameState::Room), spawn_enemies.in_set(EnemySpawn))
            .add_systems(OnEnter(GameState::Rest), cleanup_enemies)
            .add_systems(Update, advance_enemies
                .after(apply_victory_movement)
                .run_if(in_state(GameState::Room)));
    }
}

fn spawn_enemies(
    mut commands: Commands,
    config: Res<CenturionConfig>,
    run_seed: Res<RunSeed>,
    layout: Option<Res<RoomLayout>>,
    exit_pos_res: Option<Res<CurrentExitPos>>,
    existing: Query<(), With<Enemy>>,
    persistence: Option<Res<PlayerPersistence>>,
) {
    if !existing.is_empty() {
        return;
    }

    // Floor 10: single boss at center, force = entry_force * 2
    if config.current_floor == 10 {
        let boss_force = persistence.map(|p| p.force * 2).unwrap_or(20);
        let boss_pos = GridPos { x: 4, y: 4 };
        commands.spawn((
            Enemy,
            EnemyForce(boss_force),
            EnemyBehavior::Static,
            boss_pos,
            Sprite {
                color: Color::srgb(0.9, 0.1, 0.1),
                ..default()
            },
            Transform::from_translation(grid_to_world(boss_pos).extend(0.5))
                .with_scale(Vec3::splat(TILE_SIZE * 0.875)),
            DespawnOnExit(GameState::Dead),
        ));
        info!("Boss spawned at (4,4) with force {}", boss_force);
        return;
    }

    // If RoomLayout not yet inserted, build it ourselves
    let layout_ref = if let Some(layout) = layout {
        layout.into_inner().clone()
    } else {
        let (layout, _) = build_room_proc(config.current_floor, run_seed.0);
        layout
    };

    // Get exit position; fallback to re-derive if somehow missing (defensive)
    let exit_pos = match exit_pos_res {
        Some(r) => r.0,
        None => {
            let (_, ep) = build_room_proc(config.current_floor, run_seed.0);
            ep
        }
    };

    // Generate enemies procedurally
    let defs = generate_enemy_defs(config.current_floor, run_seed.0, &layout_ref, exit_pos);

    for def in defs {
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
