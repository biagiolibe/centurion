use bevy::prelude::*;
use bevy::app::PluginGroupBuilder;
use crate::enemies::EnemiesPlugin;
use crate::input::InputPlugin;
use crate::rendering::CenturionRenderPlugin;
use crate::resolver::ResolverPlugin;
use crate::config::CenturionConfig;
use crate::map_gen::MapGenPlugin;
use crate::player::PlayerPlugin;
use crate::tactics::TacticsPlugin;
use crate::ui::HudPlugin;

pub mod state_plugin;
pub use state_plugin::StatePlugin;

pub struct CenturionPlugins;

impl PluginGroup for CenturionPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CenturionRenderPlugin)
            .add(StatePlugin)
            .add(MapGenPlugin)
            .add(PlayerPlugin)
            .add(InputPlugin)
            .add(TacticsPlugin)
            .add(EnemiesPlugin)
            .add(ResolverPlugin)
            .add(HudPlugin)
    }
}

pub fn setup_config(mut commands: Commands) {
    commands.insert_resource(CenturionConfig::default());
}
