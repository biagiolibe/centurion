use bevy::prelude::*;
use bevy::ecs::message::MessageWriter;
use crate::map_gen::{GridPos, RoomLayout, TileKind, grid_to_world};
use crate::player::Player;
use crate::tactics::CombatIntent;
use super::components::{Enemy, EnemyBehavior, Axis};

pub fn advance_enemies(
    player_q: Query<Entity, With<Player>>,
    mut enemy_q: Query<(Entity, &mut GridPos, &mut EnemyBehavior, &mut Transform), With<Enemy>>,
    layout: Res<RoomLayout>,
    player_pos: Res<crate::resolver::CurrentPlayerPos>,
    mut combat_writer: MessageWriter<CombatIntent>,
    mut turn_pending: ResMut<crate::tactics::TurnPending>,
) {
    if !turn_pending.0 {
        return;
    }
    turn_pending.0 = false;

    let Ok(player_entity) = player_q.single() else {
        return;
    };

    let player_grid_pos = player_pos.0;

    for (enemy_entity, mut enemy_pos, mut behavior, mut transform) in &mut enemy_q {
        let (target, new_behavior) = compute_enemy_move(&enemy_pos, *behavior, &player_grid_pos, &layout);
        *behavior = new_behavior;

        if target == player_grid_pos {
            combat_writer.write(CombatIntent {
                attacker: enemy_entity,
                defender: player_entity,
                target_pos: target,
            });
        } else if is_walkable(target, &layout) {
            *enemy_pos = target;
            transform.translation = grid_to_world(target).extend(0.5);
        }
    }
}

fn compute_enemy_move(
    enemy_pos: &GridPos,
    behavior: EnemyBehavior,
    player_pos: &GridPos,
    layout: &RoomLayout,
) -> (GridPos, EnemyBehavior) {
    match behavior {
        EnemyBehavior::Static => (*enemy_pos, EnemyBehavior::Static),
        EnemyBehavior::Patrol { axis, direction } => {
            let candidate = match axis {
                Axis::Horizontal => GridPos {
                    x: (enemy_pos.x as i32 + direction as i32) as u8,
                    y: enemy_pos.y,
                },
                Axis::Vertical => GridPos {
                    x: enemy_pos.x,
                    y: (enemy_pos.y as i32 + direction as i32) as u8,
                },
            };

            if is_walkable(candidate, layout) {
                (candidate, EnemyBehavior::Patrol { axis, direction })
            } else {
                // Rimbalza: inverti direzione e rimani fermo questo turno
                (*enemy_pos, EnemyBehavior::Patrol { axis, direction: -direction })
            }
        }
        EnemyBehavior::Guard { alerted } => {
            let new_alerted = alerted || player_in_los(enemy_pos, player_pos, layout);

            let target = if new_alerted {
                step_toward(enemy_pos, player_pos)
            } else {
                *enemy_pos
            };

            (target, EnemyBehavior::Guard { alerted: new_alerted })
        }
    }
}

fn player_in_los(enemy: &GridPos, player: &GridPos, layout: &RoomLayout) -> bool {
    // Ray cast ortogonale a 3 tile da nemico verso player
    // Controlla se player è visibile in una delle 4 direzioni

    let dx = player.x as i32 - enemy.x as i32;
    let dy = player.y as i32 - enemy.y as i32;

    // Orizzontale
    if dy == 0 && dx.abs() <= 3 && dx.abs() > 0 {
        let step = if dx > 0 { 1 } else { -1 };
        for i in 1..=dx.abs().min(3) {
            let check_x = (enemy.x as i32 + step * i) as u8;
            if layout.get(check_x, enemy.y) == TileKind::Wall {
                return false;
            }
        }
        return true;
    }

    // Verticale
    if dx == 0 && dy.abs() <= 3 && dy.abs() > 0 {
        let step = if dy > 0 { 1 } else { -1 };
        for i in 1..=dy.abs().min(3) {
            let check_y = (enemy.y as i32 + step * i) as u8;
            if layout.get(enemy.x, check_y) == TileKind::Wall {
                return false;
            }
        }
        return true;
    }

    false
}

fn step_toward(enemy: &GridPos, player: &GridPos) -> GridPos {
    let dx = player.x as i32 - enemy.x as i32;
    let dy = player.y as i32 - enemy.y as i32;

    // Manhattan: preferisci chiudere l'asse con delta maggiore
    if dx.abs() > dy.abs() {
        if dx > 0 {
            GridPos { x: enemy.x + 1, y: enemy.y }
        } else {
            GridPos { x: enemy.x - 1, y: enemy.y }
        }
    } else if dy.abs() > 0 {
        if dy > 0 {
            GridPos { x: enemy.x, y: enemy.y + 1 }
        } else {
            GridPos { x: enemy.x, y: enemy.y - 1 }
        }
    } else {
        // Su nemico
        *enemy
    }
}

fn is_walkable(pos: GridPos, layout: &RoomLayout) -> bool {
    pos.x < 8 && pos.y < 8 && layout.get(pos.x, pos.y) != TileKind::Wall
}
