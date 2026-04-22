use bevy::prelude::*;
use bevy::ecs::message::{MessageReader, MessageWriter};
use crate::state::GameState;
use crate::map_gen::{GridPos, TileKind, RoomLayout, grid_to_world};
use crate::player::{Player, CurrentSteps};
use crate::input::MoveIntent;
use crate::enemies::Enemy;
use crate::tactics::CombatIntent;
use crate::config::CenturionConfig;

pub fn can_move_to(pos: GridPos, layout: &RoomLayout) -> bool {
    layout.get(pos.x, pos.y) != TileKind::Wall
}

pub fn apply_movement(
    state: Res<State<GameState>>,
    mut move_reader: MessageReader<MoveIntent>,
    mut player_q: Query<(Entity, &mut GridPos, &mut Transform, &mut CurrentSteps), (With<Player>, Without<Enemy>)>,
    layout: Res<RoomLayout>,
    enemy_q: Query<(Entity, &GridPos), (With<Enemy>, Without<Player>)>,
    mut combat_writer: MessageWriter<CombatIntent>,
    mut next_state: ResMut<NextState<GameState>>,
    mut config: ResMut<CenturionConfig>,
) {
    if *state.get() != GameState::Room {
        return;
    }

    let mut iter = player_q.iter_mut();
    let Some((player_entity, mut grid_pos, mut transform, mut steps)) = iter.next() else {
        return;
    };
    drop(iter); // Release the iterator borrow so we can use player_q again if needed

    for intent in move_reader.read() {
        // Calcola nuova posizione con saturating add
        let new_x = (grid_pos.x as i32) + intent.direction.x;
        let new_y = (grid_pos.y as i32) + intent.direction.y;

        // Bounds check 0..8
        if new_x < 0 || new_x >= 8 || new_y < 0 || new_y >= 8 {
            continue;
        }

        let new_pos = GridPos { x: new_x as u8, y: new_y as u8 };

        // Wall check — nessun consumo di passi
        if !can_move_to(new_pos, &layout) {
            continue;
        }

        // Enemy check
        let maybe_enemy = enemy_q.iter()
            .find(|(_, ep)| ep.x == new_pos.x && ep.y == new_pos.y);

        if let Some((enemy_entity, _)) = maybe_enemy {
            combat_writer.write(CombatIntent {
                attacker: player_entity,
                defender: enemy_entity,
                target_pos: new_pos,
            });
            // Combat non consuma passi — continue
            continue;
        }

        // Muovi player, decrementa passi
        *grid_pos = new_pos;
        transform.translation = grid_to_world(new_pos).extend(1.0);
        steps.0 -= 1;

        info!("Player moved to ({}, {}), steps remaining: {}", new_pos.x, new_pos.y, steps.0);

        // Morte se passi esauriti
        if steps.0 <= 0 {
            next_state.set(GameState::Dead);
        }

        // Exit detection
        if layout.get(new_pos.x, new_pos.y) == TileKind::Exit {
            config.current_floor += 1;
            info!("Player reached exit! Advancing to floor {}", config.current_floor);
            next_state.set(GameState::Rest);
        }
    }
}
