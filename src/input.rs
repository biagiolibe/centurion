use bevy::prelude::*;
use bevy::ecs::message::MessageWriter;
use crate::state::GameState;

#[derive(Message)]
pub struct MoveIntent {
    pub direction: IVec2,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<MoveIntent>()
            .add_systems(Update, read_player_input);
    }
}

fn read_player_input(
    state: Res<State<GameState>>,
    input: Res<ButtonInput<KeyCode>>,
    mut move_messages: MessageWriter<MoveIntent>,
) {
    // Only process input when in Room state
    if *state.get() != GameState::Room {
        return;
    }

    let directions = [
        (KeyCode::ArrowUp, IVec2::new(0, -1)),
        (KeyCode::KeyW, IVec2::new(0, -1)),
        (KeyCode::ArrowDown, IVec2::new(0, 1)),
        (KeyCode::KeyS, IVec2::new(0, 1)),
        (KeyCode::ArrowLeft, IVec2::new(-1, 0)),
        (KeyCode::KeyA, IVec2::new(-1, 0)),
        (KeyCode::ArrowRight, IVec2::new(1, 0)),
        (KeyCode::KeyD, IVec2::new(1, 0)),
    ];

    for (key, dir) in directions {
        if input.just_pressed(key) {
            move_messages.write(MoveIntent { direction: dir });
            break; // una sola azione per frame, no diagonali
        }
    }
}
