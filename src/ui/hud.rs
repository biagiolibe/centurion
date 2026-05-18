use bevy::prelude::*;
use bevy::state::state_scoped::DespawnOnExit;
use crate::state::GameState;
use crate::player::PlayerStats;
use crate::config::{CenturionConfig, RunStats};

#[derive(Component)]
pub struct HudRoot;

#[derive(Component)]
pub struct HudStep;

#[derive(Component)]
pub struct HudForce;

#[derive(Component)]
pub struct HudFloor;

#[derive(Component)]
pub struct HudEnemies;

#[derive(Component)]
pub struct HudRunes;

#[derive(Component)]
pub struct HudBoss;

pub fn spawn_hud(mut commands: Commands, existing: Query<(), With<HudRoot>>) {
    if !existing.is_empty() {
        return;
    }

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
        parent.spawn((
            Text::new("─────────────"),
            TextFont { font_size: 16.0, ..default() },
            TextColor(Color::srgb(0.3, 0.3, 0.3)),
        ));
        parent.spawn((
            Text::new("NEMICI: 0"),
            TextFont { font_size: 20.0, ..default() },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            HudEnemies,
        ));
        parent.spawn((
            Text::new("RUNE:   0"),
            TextFont { font_size: 20.0, ..default() },
            TextColor(Color::srgb(0.7, 0.3, 1.0)),
            HudRunes,
        ));
        parent.spawn((
            Text::new("BOSS~:  F10"),
            TextFont { font_size: 20.0, ..default() },
            TextColor(Color::srgb(0.9, 0.3, 0.3)),
            HudBoss,
        ));
    });
}

pub fn update_steps_display(
    stats: Res<PlayerStats>,
    config: Res<CenturionConfig>,
    run_stats: Res<RunStats>,
    mut queries: ParamSet<(
        Query<&mut Text, With<HudStep>>,
        Query<&mut Text, With<HudForce>>,
        Query<&mut Text, With<HudFloor>>,
        Query<&mut Text, With<HudEnemies>>,
        Query<&mut Text, With<HudRunes>>,
        Query<&mut Text, With<HudBoss>>,
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
    if let Ok(mut text) = queries.p3().single_mut() {
        **text = format!("NEMICI: {}", run_stats.enemies_defeated);
    }
    if let Ok(mut text) = queries.p4().single_mut() {
        **text = format!("RUNE:   {}", run_stats.runes_collected);
    }
    let boss_est = (10 + run_stats.enemies_defeated as i32 * 2
        - run_stats.runes_collected as i32 * 5)
        .max(1);
    if let Ok(mut text) = queries.p5().single_mut() {
        **text = format!("BOSS~:  F{}", boss_est);
    }
}
