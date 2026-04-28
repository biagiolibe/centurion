use bevy::prelude::*;
use bevy::ecs::message::MessageReader;
use std::collections::HashMap;
use crate::state::GameState;
use crate::player::{Player, Force};
use crate::enemies::{Enemy, EnemyForce};
use crate::tactics::MovementSet;
use crate::tactics::CombatIntent;
use crate::map_gen::{GridPos, grid_to_world};

pub mod combat;
pub mod flash;
pub use combat::{CombatResult, resolve};
pub use flash::{FlashPlugin, LastCombatOutcome};

#[derive(Resource, Default)]
pub struct EnemyForcesMap(pub HashMap<Entity, i32>);

#[derive(Resource, Default)]
pub struct PendingPlayerVictory {
    pub target_pos: Option<GridPos>,
}

#[derive(Resource, Default)]
pub struct CurrentPlayerForce(pub i32);

#[derive(Resource, Copy, Clone)]
pub struct CurrentPlayerPos(pub GridPos);

impl Default for CurrentPlayerPos {
    fn default() -> Self {
        Self(GridPos { x: 0, y: 0 })
    }
}

pub struct ResolverPlugin;

impl Plugin for ResolverPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LastCombatOutcome>()
            .init_resource::<EnemyForcesMap>()
            .init_resource::<PendingPlayerVictory>()
            .init_resource::<CurrentPlayerForce>()
            .init_resource::<CurrentPlayerPos>()
            .add_plugins(FlashPlugin)
            .add_systems(OnEnter(GameState::Room), cache_enemy_forces)
            .add_systems(Update, (sync_player_force, sync_player_pos).after(MovementSet))
            .add_systems(Update, (resolve_combat, apply_victory_movement)
                .chain()
                .after(MovementSet));
    }
}

fn cache_enemy_forces(
    enemy_q: Query<(Entity, &EnemyForce), With<Enemy>>,
    mut forces_map: ResMut<EnemyForcesMap>,
) {
    forces_map.0.clear();
    for (entity, force) in &enemy_q {
        forces_map.0.insert(entity, force.0);
    }
}

fn sync_player_force(
    player_q: Query<&Force, With<Player>>,
    mut current_force: ResMut<CurrentPlayerForce>,
) {
    if let Ok(force) = player_q.single() {
        current_force.0 = force.0;
    }
}

fn sync_player_pos(
    player_q: Query<&GridPos, With<Player>>,
    mut current_pos: ResMut<CurrentPlayerPos>,
) {
    if let Ok(pos) = player_q.single() {
        current_pos.0 = *pos;
    }
}

fn resolve_combat(
    mut commands: Commands,
    mut combat_reader: MessageReader<CombatIntent>,
    mut player_q: Query<(Entity, &mut Force), With<Player>>,
    forces_map: Res<EnemyForcesMap>,
    current_force: Res<CurrentPlayerForce>,
    mut victory: ResMut<PendingPlayerVictory>,
    mut next_state: ResMut<NextState<GameState>>,
    mut last_outcome: ResMut<LastCombatOutcome>,
) {
    for intent in combat_reader.read() {
        // Phase 1: Get player entity and extract force
        let player_entity = {
            let mut iter = player_q.iter();
            if let Some((e, _)) = iter.next() { e } else { continue; }
        };

        let player_force_val = current_force.0;

        // Phase 2: Determine who's attacking and get enemy force
        let attacker_is_player = intent.attacker == player_entity;
        let enemy_force_val = if attacker_is_player {
            forces_map.0.get(&intent.defender).copied().unwrap_or(0)
        } else {
            forces_map.0.get(&intent.attacker).copied().unwrap_or(0)
        };

        // Phase 3: Resolve combat and determine outcome
        let outcome = if attacker_is_player {
            resolve(player_force_val, enemy_force_val)
        } else {
            resolve(enemy_force_val, player_force_val)
        };

        // Phase 4: Apply outcome
        if attacker_is_player {
            match outcome {
                CombatResult::PlayerWins { new_force } => {
                    let mut iter = player_q.iter_mut();
                    if let Some((_, mut pf)) = iter.next() {
                        pf.0 = new_force;
                    }
                    victory.target_pos = Some(intent.target_pos);
                    commands.entity(intent.defender).despawn();
                    info!("Combat: Player wins! Force {} -> {}", player_force_val + enemy_force_val, new_force);
                    *last_outcome = LastCombatOutcome::Victory;
                    next_state.set(GameState::CombatEvent);
                }
                CombatResult::PlayerDies => {
                    info!("Combat: Player dies (force {} vs {})!", player_force_val, enemy_force_val);
                    *last_outcome = LastCombatOutcome::Defeat;
                    next_state.set(GameState::CombatEvent);
                }
            }
        } else {
            match outcome {
                CombatResult::PlayerWins { .. } => {
                    info!("Combat: Enemy wins (force {} vs {})!", enemy_force_val, player_force_val);
                    *last_outcome = LastCombatOutcome::Defeat;
                    next_state.set(GameState::CombatEvent);
                }
                CombatResult::PlayerDies => {
                    commands.entity(intent.attacker).despawn();
                    info!("Combat: Player defeats advancing enemy! Force preserved at {}", player_force_val);
                    *last_outcome = LastCombatOutcome::Victory;
                    next_state.set(GameState::CombatEvent);
                }
            }
        }
    }
}

pub fn apply_victory_movement(
    mut player_q: Query<(&mut GridPos, &mut Transform), With<Player>>,
    mut victory: ResMut<PendingPlayerVictory>,
) {
    if let Some(target_pos) = victory.target_pos.take() {
        if let Ok((mut pos, mut transform)) = player_q.single_mut() {
            *pos = target_pos;
            transform.translation = grid_to_world(target_pos).extend(1.0);
        }
    }
}
