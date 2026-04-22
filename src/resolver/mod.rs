use bevy::prelude::*;
use bevy::ecs::message::MessageReader;
use crate::state::GameState;
use crate::player::{Player, Force};
use crate::enemies::{Enemy, EnemyForce};
use crate::tactics::{CombatIntent, MovementSet};
use crate::map_gen::{GridPos, grid_to_world};

pub mod combat;
pub mod flash;
pub use combat::{CombatResult, resolve};
pub use flash::{FlashPlugin, LastCombatOutcome};

pub struct ResolverPlugin;

impl Plugin for ResolverPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LastCombatOutcome>()
            .add_plugins(FlashPlugin)
            .add_systems(Update, resolve_combat.after(MovementSet));
    }
}

fn resolve_combat(
    mut commands: Commands,
    mut combat_reader: MessageReader<CombatIntent>,
    mut player_q: Query<(&mut Force, &mut GridPos, &mut Transform), (With<Player>, Without<Enemy>)>,
    enemy_q: Query<&EnemyForce, (With<Enemy>, Without<Player>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut last_outcome: ResMut<LastCombatOutcome>,
) {
    for intent in combat_reader.read() {
        let Ok(enemy_force) = enemy_q.get(intent.defender) else { continue; };
        let enemy_force_val = enemy_force.0;

        let mut iter = player_q.iter_mut();
        let Some((mut player_force, mut player_pos, mut transform)) = iter.next() else { continue; };
        drop(iter);

        match resolve(player_force.0, enemy_force_val) {
            CombatResult::PlayerWins { new_force } => {
                player_force.0 = new_force;
                *player_pos = intent.target_pos;
                transform.translation = grid_to_world(intent.target_pos).extend(1.0);
                commands.entity(intent.defender).despawn();
                info!("Combat: Player wins! Force {} -> {}", player_force.0 + enemy_force_val, new_force);
                *last_outcome = LastCombatOutcome::Victory;
                next_state.set(GameState::CombatEvent);
            }
            CombatResult::PlayerDies => {
                info!("Combat: Player dies (force {} vs {})!", player_force.0, enemy_force_val);
                *last_outcome = LastCombatOutcome::Defeat;
                next_state.set(GameState::CombatEvent);
            }
        }
    }
}
