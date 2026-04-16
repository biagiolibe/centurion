use bevy::prelude::*;
use bevy::app::PluginGroupBuilder;
use crate::rendering::CenturionRenderPlugin;
use crate::config::CenturionConfig;
use crate::map_gen::MapGenPlugin;
use crate::player::PlayerPlugin;

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
    }
}

pub fn setup_config(mut commands: Commands) {
    commands.insert_resource(CenturionConfig::default());
}
