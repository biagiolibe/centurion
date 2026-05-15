use bevy::prelude::*;
use bevy::state::state_scoped::DespawnOnExit;
use crate::state::GameState;
use crate::config::{CenturionConfig, RunSeed, RunStats};
use crate::player::{Player, CurrentSteps, Force, PlayerPersistence};
use crate::resolver::LastCombatOutcome;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeathCause {
    OutOfSteps,
    KilledByEnemy,
}

#[derive(Resource, Clone)]
struct DeadStats {
    floors_cleared: u8,
    steps_remaining: i32,
    force: i32,
    score: i32,
    cause: DeathCause,
}

#[derive(Component)]
pub struct DeadScreenRoot;

pub struct DeadScreenPlugin;

impl Plugin for DeadScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
                OnEnter(GameState::Dead),
                (populate_dead_stats, spawn_dead_screen).chain(),
            )
            .add_systems(Update, dead_input.run_if(in_state(GameState::Dead)));
    }
}

fn populate_dead_stats(
    mut commands: Commands,
    config: Res<CenturionConfig>,
    player_q: Query<(&CurrentSteps, &Force), With<Player>>,
    last_outcome: Res<LastCombatOutcome>,
    run_stats: Res<RunStats>,
) {
    let (steps_remaining, force) = player_q
        .single()
        .map(|(s, f)| (s.0, f.0))
        .unwrap_or((0, 0));

    let floors_cleared = config.current_floor.saturating_sub(1);

    let cause = if steps_remaining <= 0 {
        DeathCause::OutOfSteps
    } else {
        match *last_outcome {
            LastCombatOutcome::Defeat | LastCombatOutcome::BossVictory => DeathCause::KilledByEnemy,
            LastCombatOutcome::Victory => DeathCause::OutOfSteps,
        }
    };

    let score = run_stats.calculate_score(floors_cleared, steps_remaining.max(0), force);

    commands.insert_resource(DeadStats {
        floors_cleared,
        steps_remaining: steps_remaining.max(0),
        force,
        score,
        cause,
    });
}

fn spawn_dead_screen(mut commands: Commands, stats: Res<DeadStats>, run_seed: Res<RunSeed>) {
    let cause_text = match stats.cause {
        DeathCause::OutOfSteps => "Cause: Out of steps",
        DeathCause::KilledByEnemy => "Cause: Killed by enemy",
    };

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
            DeadScreenRoot,
            DespawnOnExit(GameState::Dead),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("CENTURION FALLS"),
                TextFont { font_size: 36.0, ..default() },
                TextColor(Color::srgb(1.0, 0.0, 0.0)),
            ));
            parent.spawn((
                Text::new(format!("Floors cleared: {}", stats.floors_cleared)),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
            ));
            parent.spawn((
                Text::new(format!("Final Force: {}", stats.force)),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
            ));
            parent.spawn((
                Text::new(format!("Steps remaining: {}", stats.steps_remaining)),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
            ));
            parent.spawn((
                Text::new(cause_text),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
            ));
            parent.spawn((
                Text::new(format!("Score: {}", stats.score)),
                TextFont { font_size: 28.0, ..default() },
                TextColor(Color::WHITE),
            ));
            parent.spawn((
                Text::new(format!("Seed: 0x{:016x}", run_seed.0)),
                TextFont { font_size: 18.0, ..default() },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
            ));
            parent.spawn((
                Text::new("Press R to restart"),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
        });
}

fn dead_input(
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
