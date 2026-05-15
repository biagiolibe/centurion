use bevy::prelude::*;
use bevy::state::state_scoped::DespawnOnExit;
use crate::state::GameState;
use crate::config::{CenturionConfig, RunSeed};
use crate::map_gen::{GridPos, RoomLayout, grid_to_world, TILE_SIZE, CurrentExitPos, generate_item_defs};
use crate::enemies::{Enemy, EnemySpawn};

pub mod components;
pub use components::{Item, ItemKind, HeldItem, ItemDef};

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Room),
            spawn_items.after(EnemySpawn),
        )
        .add_systems(OnEnter(GameState::Rest), cleanup_items);
    }
}

fn spawn_items(
    mut commands: Commands,
    config: Res<CenturionConfig>,
    run_seed: Res<RunSeed>,
    layout: Option<Res<RoomLayout>>,
    exit_pos_res: Option<Res<CurrentExitPos>>,
    enemy_q: Query<&GridPos, With<Enemy>>,
    existing: Query<(), With<Item>>,
) {
    if !existing.is_empty() || config.current_floor == 10 {
        return;
    }

    let layout_ref = match layout {
        Some(l) => l.into_inner().clone(),
        None => return,
    };

    let exit_pos = match exit_pos_res {
        Some(e) => e.0,
        None => return,
    };

    let enemy_positions: Vec<GridPos> = enemy_q.iter().copied().collect();
    let defs = generate_item_defs(config.current_floor, run_seed.0, &layout_ref, exit_pos, &enemy_positions);

    for def in defs {
        let world_pos = grid_to_world(def.pos);
        let color = match def.kind {
            ItemKind::Ration => Color::srgb(0.0, 0.9, 0.9),
            ItemKind::Whetstone => Color::srgb(1.0, 0.6, 0.0),
        };

        commands.spawn((
            Item,
            def.kind,
            def.pos,
            Sprite { color, ..default() },
            Transform::from_translation(world_pos.extend(0.3))
                .with_scale(Vec3::splat(TILE_SIZE * 0.5)),
            DespawnOnExit(GameState::Dead),
        ));

        info!("Item {:?} spawned at ({},{})", def.kind, def.pos.x, def.pos.y);
    }
}

fn cleanup_items(mut commands: Commands, items: Query<Entity, With<Item>>) {
    for entity in items.iter() {
        commands.entity(entity).despawn();
    }
}
