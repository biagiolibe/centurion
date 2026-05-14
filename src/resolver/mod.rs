use bevy::prelude::*;
use bevy::ecs::message::MessageReader;
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
pub struct PendingPlayerVictory {
    pub target_pos: Option<GridPos>,
}

#[derive(Resource, Default)]
pub struct CurrentPlayerForce(pub i32);

pub struct ResolverPlugin;

impl Plugin for ResolverPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LastCombatOutcome>()
            .init_resource::<PendingPlayerVictory>()
            .init_resource::<CurrentPlayerForce>()
            .add_plugins(FlashPlugin)
            .add_systems(Update, sync_player_force.after(MovementSet))
            .add_systems(Update, (resolve_combat, apply_victory_movement)
                .chain()
                .after(MovementSet));
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

fn resolve_combat(
    mut commands: Commands,
    mut combat_reader: MessageReader<CombatIntent>,
    mut player_q: Query<(Entity, &mut Force), With<Player>>,
    enemy_force_q: Query<&EnemyForce, With<Enemy>>,
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

        // Phase 2: Determine who's attacking and get enemy force directly from component
        let attacker_is_player = intent.attacker == player_entity;
        let enemy_entity = if attacker_is_player { intent.defender } else { intent.attacker };
        let enemy_force_val = enemy_force_q.get(enemy_entity).map(|f| f.0).unwrap_or(0);

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
                    let new_force = player_force_val - enemy_force_val;
                    let mut iter = player_q.iter_mut();
                    if let Some((_, mut pf)) = iter.next() {
                        pf.0 = new_force;
                    }
                    commands.entity(intent.attacker).despawn();
                    info!("Combat: Player defeats advancing enemy! Force {} -> {}", player_force_val, new_force);
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
