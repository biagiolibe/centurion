use bevy::prelude::*;
use bevy::state::state_scoped::DespawnOnExit;
use crate::state::GameState;
use crate::config::CenturionConfig;
use crate::player::PlayerPersistence;

#[derive(Component)]
pub struct RestScreenRoot;

pub struct RestScreenPlugin;

impl Plugin for RestScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_rest_screen,
                rest_input,
            )
                .run_if(in_state(GameState::Rest)),
        );
    }
}

fn spawn_rest_screen(
    mut commands: Commands,
    config: Res<CenturionConfig>,
    persistence: Option<Res<PlayerPersistence>>,
    existing: Query<(), With<RestScreenRoot>>,
) {
    if !existing.is_empty() {
        return;
    }

    let floor_cleared = config.current_floor.saturating_sub(1);
    let (steps, force) = persistence
        .as_ref()
        .map(|p| (p.steps, p.force))
        .unwrap_or((0, 0));

    let force_after_rest = ((force / 5) + 1) * 5;

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
            RestScreenRoot,
            DespawnOnExit(GameState::Rest),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(format!("FLOOR {} CLEARED", floor_cleared)),
                TextFont { font_size: 32.0, ..default() },
                TextColor(Color::WHITE),
            ));
            parent.spawn((
                Text::new(format!("STEPS REMAINING: {}", steps)),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
            ));
            parent.spawn((
                Text::new(format!("FORCE: {}", force)),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::WHITE),
            ));
            parent.spawn((
                Text::new(format!("FORCE after rest: {}", force_after_rest)),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::srgb(1.0, 0.9, 0.0)),
            ));
            parent.spawn((
                Text::new("Press SPACE to descend"),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
        });
}

fn rest_input(
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Room);
    }
}
