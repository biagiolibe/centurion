use bevy::prelude::*;
use bevy::state::state_scoped::DespawnOnExit;
use crate::state::GameState;
use crate::config::{CenturionConfig, RunSeed, RunStats};
use crate::player::{Player, CurrentSteps, Force, PlayerPersistence};

#[derive(Component)]
struct WinScreenRoot;

pub struct WinScreenPlugin;

impl Plugin for WinScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::WinScreen), spawn_win_screen)
            .add_systems(Update, win_input.run_if(in_state(GameState::WinScreen)));
    }
}

fn spawn_win_screen(
    mut commands: Commands,
    config: Res<CenturionConfig>,
    stats: Res<RunStats>,
    run_seed: Res<RunSeed>,
    player_q: Query<(&CurrentSteps, &Force), With<Player>>,
) {
    let (steps_remaining, force) = player_q
        .single()
        .map(|(s, f)| (s.0, f.0))
        .unwrap_or((0, 0));

    let floors_cleared = config.current_floor;
    let score = stats.calculate_score(floors_cleared, steps_remaining, force);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(12.0),
                ..default()
            },
            WinScreenRoot,
            DespawnOnExit(GameState::WinScreen),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("CENTURION ASCENDS"),
                TextFont { font_size: 48.0, ..default() },
                TextColor(Color::srgb(1.0, 0.85, 0.0)),
            ));
            parent.spawn((
                Text::new(format!("Floors Cleared: {}", floors_cleared)),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
            ));
            parent.spawn((
                Text::new(format!("Final Force: {}", force)),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
            ));
            parent.spawn((
                Text::new(format!("Steps Remaining: {}", steps_remaining)),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
            ));
            parent.spawn((
                Text::new(format!("Score: {}", score)),
                TextFont { font_size: 32.0, ..default() },
                TextColor(Color::srgb(1.0, 0.85, 0.0)),
            ));
            parent.spawn((
                Text::new(format!("Seed: 0x{:016x}", run_seed.0)),
                TextFont { font_size: 16.0, ..default() },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
            ));
            parent.spawn((
                Text::new("Press R to restart"),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
        });
}

fn win_input(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut config: ResMut<CenturionConfig>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        config.current_floor = 1;
        commands.remove_resource::<PlayerPersistence>();
        commands.remove_resource::<RunSeed>();
        next_state.set(GameState::Loading);
    }
}
