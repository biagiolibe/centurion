use bevy::prelude::*;
use bevy::state::state_scoped::DespawnOnExit;
use crate::state::GameState;
use crate::config::CenturionConfig;
use crate::player::PlayerPersistence;
use crate::items::ItemKind;

#[derive(Component)]
pub struct RestScreenRoot;

pub struct RestScreenPlugin;

impl Plugin for RestScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_rest_screen, handle_rest_choice).run_if(in_state(GameState::Rest)),
        );
    }
}

fn tier_recovery(force: i32) -> i32 {
    ((force / 5) + 1) * 5
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

    let Some(p) = persistence else { return; };

    let floor_cleared = config.current_floor.saturating_sub(1);
    let steps = p.steps;
    let force = p.force;
    let held = p.held_item;

    let adjusted_force = if held == Some(ItemKind::Whetstone) { force + 1 } else { force };

    let opt1_force = tier_recovery(adjusted_force);
    let opt3_force = tier_recovery(tier_recovery(adjusted_force));

    let held_line = match held {
        Some(ItemKind::Whetstone) => format!("Holding: Whetstone  (+1 force tier bonus)"),
        Some(ItemKind::Ration) => "Holding: Ration".to_string(),
        None => String::new(),
    };

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
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
                Text::new(format!("Steps: {}   Force: {}", steps, force)),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));
            if !held_line.is_empty() {
                parent.spawn((
                    Text::new(held_line),
                    TextFont { font_size: 18.0, ..default() },
                    TextColor(Color::srgb(1.0, 0.6, 0.0)),
                ));
            }
            parent.spawn((
                Text::new(format!(
                    "1. Discesa Veloce        +20 steps   Force {} → {}",
                    force, opt1_force
                )),
                TextFont { font_size: 22.0, ..default() },
                TextColor(Color::srgb(0.3, 1.0, 0.3)),
            ));
            parent.spawn((
                Text::new(format!(
                    "2. Riposo Prolungato     +40 steps   Force {} (invariata)",
                    force
                )),
                TextFont { font_size: 22.0, ..default() },
                TextColor(Color::srgb(0.3, 0.7, 1.0)),
            ));
            parent.spawn((
                Text::new(format!(
                    "3. Allenamento Intensivo  +0 steps   Force {} → {}",
                    force, opt3_force
                )),
                TextFont { font_size: 22.0, ..default() },
                TextColor(Color::srgb(1.0, 0.7, 0.3)),
            ));
            parent.spawn((
                Text::new("Premi 1 / 2 / 3 per scegliere"),
                TextFont { font_size: 18.0, ..default() },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
            ));
        });
}

fn handle_rest_choice(
    input: Res<ButtonInput<KeyCode>>,
    mut persistence: Option<ResMut<PlayerPersistence>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let choice = if input.just_pressed(KeyCode::Digit1) {
        1
    } else if input.just_pressed(KeyCode::Digit2) {
        2
    } else if input.just_pressed(KeyCode::Digit3) {
        3
    } else {
        return;
    };

    let Some(p) = persistence.as_mut() else { return; };

    let adjusted_force = if p.held_item == Some(ItemKind::Whetstone) {
        p.force + 1
    } else {
        p.force
    };

    match choice {
        1 => {
            p.steps += 20;
            p.force = tier_recovery(adjusted_force);
        }
        2 => {
            p.steps += 40;
            // force invariata
        }
        3 => {
            p.force = tier_recovery(tier_recovery(adjusted_force));
        }
        _ => unreachable!(),
    }

    p.held_item = None;

    info!(
        "Rest choice {}: steps → {}, force → {}",
        choice, p.steps, p.force
    );

    next_state.set(GameState::Room);
}
