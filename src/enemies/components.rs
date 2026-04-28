use bevy::prelude::*;
use crate::map_gen::GridPos;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct EnemyForce(pub i32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Component, Clone, Copy, Debug)]
pub enum EnemyBehavior {
    Static,
    Patrol { axis: Axis, direction: i8 },
    Guard { alerted: bool },
}

#[derive(Clone, Copy, Debug)]
pub struct EnemyDef {
    pub pos: GridPos,
    pub force: i32,
    pub behavior: EnemyBehavior,
}
