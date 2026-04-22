use bevy::prelude::*;
use crate::state::GameState;
use crate::player::Player;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Resource, Default)]
pub enum LastCombatOutcome {
    #[default]
    Victory,
    Defeat,
}

#[derive(Resource)]
struct ActiveFlash {
    timer: Timer,
    outcome: LastCombatOutcome,
}

pub struct FlashPlugin;

impl Plugin for FlashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::CombatEvent), start_flash)
            .add_systems(
                Update,
                update_flash.run_if(in_state(GameState::CombatEvent)),
            );
    }
}

fn start_flash(mut commands: Commands, outcome: Res<LastCombatOutcome>) {
    commands.insert_resource(ActiveFlash {
        timer: Timer::from_seconds(0.2, TimerMode::Once),
        outcome: *outcome,
    });
}

fn update_flash(
    time: Res<Time>,
    mut flash: ResMut<ActiveFlash>,
    mut player_q: Query<&mut Sprite, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    flash.timer.tick(time.delta());
    let t = flash.timer.fraction(); // 0.0 → 1.0

    if let Ok(mut sprite) = player_q.single_mut() {
        sprite.color = flash_color(flash.outcome, t);
    }

    if flash.timer.just_finished() {
        // Reset player color
        if let Ok(mut sprite) = player_q.single_mut() {
            sprite.color = Color::WHITE;
        }
        commands.remove_resource::<ActiveFlash>();
        match flash.outcome {
            LastCombatOutcome::Victory => next_state.set(GameState::Room),
            LastCombatOutcome::Defeat => next_state.set(GameState::Dead),
        }
    }
}

fn flash_color(outcome: LastCombatOutcome, t: f32) -> Color {
    match outcome {
        LastCombatOutcome::Victory => {
            // 0..0.5 WHITE→YELLOW, 0.5..1.0 YELLOW→WHITE
            let yellow = Color::srgb(1.0, 1.0, 0.0);
            if t < 0.5 {
                lerp_color(Color::WHITE, yellow, t * 2.0)
            } else {
                lerp_color(yellow, Color::WHITE, (t - 0.5) * 2.0)
            }
        }
        LastCombatOutcome::Defeat => {
            lerp_color(Color::WHITE, Color::srgb(1.0, 0.0, 0.0), t)
        }
    }
}

fn lerp_color(from: Color, to: Color, t: f32) -> Color {
    let f = from.to_srgba();
    let t2 = to.to_srgba();
    Color::srgba(
        f.red + (t2.red - f.red) * t,
        f.green + (t2.green - f.green) * t,
        f.blue + (t2.blue - f.blue) * t,
        1.0,
    )
}
