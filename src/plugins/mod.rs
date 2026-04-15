use bevy::prelude::*;
use bevy::app::PluginGroupBuilder;
use crate::rendering::CenturionRenderPlugin;
use crate::config::CenturionConfig;

pub struct CenturionPlugins;

impl PluginGroup for CenturionPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CenturionRenderPlugin)
    }
}

pub fn setup_config(mut commands: Commands) {
    commands.insert_resource(CenturionConfig::default());
}
