use bevy::prelude::*;
use bevy::window::Window;
use centurion::plugins::{CenturionPlugins, setup_config};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Centurion: 100 Steps".into(),
                resolution: (800, 800).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(CenturionPlugins)
        .add_systems(Startup, (setup_config, setup))
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d::default());

    info!("Centurion initialized with Bevy 0.18.1");
}
