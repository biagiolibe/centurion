use bevy::prelude::*;
use crate::map_gen::GridPos;

pub mod movement;
pub use movement::apply_movement;

#[derive(Message)]
pub struct CombatIntent {
    pub attacker: Entity,
    pub defender: Entity,
    pub target_pos: GridPos,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MovementSet;

#[derive(Resource, Default)]
pub struct TurnPending(pub bool);

pub struct TacticsPlugin;

impl Plugin for TacticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<CombatIntent>()
            .init_resource::<TurnPending>()
            .add_systems(Update, apply_movement.in_set(MovementSet));
    }
}
