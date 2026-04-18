use bevy::prelude::*;
use bevy::state::state_scoped::DespawnOnExit;
use crate::state::GameState;
use crate::player::PlayerStats;
use crate::config::CenturionConfig;

#[derive(Component)]
pub struct HudRoot;

#[derive(Component)]
pub struct HudStep;

#[derive(Component)]
pub struct HudForce;

#[derive(Component)]
pub struct HudFloor;

pub fn spawn_hud(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(16.0),
            left: Val::Px(16.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(4.0),
            ..default()
        },
        HudRoot,
        DespawnOnExit(GameState::Room),
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("STEPS: 100"),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::WHITE),
            HudStep,
        ));
        parent.spawn((
            Text::new("FORCE: 5"),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::WHITE),
            HudForce,
        ));
        parent.spawn((
            Text::new("FLOOR: 1"),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::WHITE),
            HudFloor,
        ));
    });
}

pub fn update_steps_display(
    stats: Res<PlayerStats>,
    config: Res<CenturionConfig>,
    mut queries: ParamSet<(
        Query<&mut Text, With<HudStep>>,
        Query<&mut Text, With<HudForce>>,
        Query<&mut Text, With<HudFloor>>,
    )>,
) {
    if let Ok(mut text) = queries.p0().single_mut() {
        **text = format!("STEPS: {}", stats.steps);
    }
    if let Ok(mut text) = queries.p1().single_mut() {
        **text = format!("FORCE: {}", stats.force);
    }
    if let Ok(mut text) = queries.p2().single_mut() {
        **text = format!("FLOOR: {}", config.current_floor);
    }
}
