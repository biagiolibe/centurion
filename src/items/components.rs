use bevy::prelude::*;
use crate::map_gen::GridPos;

#[derive(Component)]
pub struct Item;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemKind {
    Ration,
    Whetstone,
    Runa,
}

#[derive(Component, Clone, Copy)]
pub struct HeldItem(pub ItemKind);

#[derive(Clone, Copy, Debug)]
pub struct ItemDef {
    pub pos: GridPos,
    pub kind: ItemKind,
}
