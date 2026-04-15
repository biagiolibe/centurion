use bevy::prelude::*;

#[derive(Resource, Clone, Copy)]
pub struct CenturionConfig {
    pub current_floor: u8,
    pub window_width: f32,
    pub window_height: f32,
}

impl Default for CenturionConfig {
    fn default() -> Self {
        Self {
            current_floor: 1,
            window_width: 800.0,
            window_height: 800.0,
        }
    }
}
