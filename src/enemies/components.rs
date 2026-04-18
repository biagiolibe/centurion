use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct EnemyForce(pub i32);

#[derive(Component)]
pub enum EnemyBehavior {
    Static,
}
