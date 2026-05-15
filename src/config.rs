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

#[derive(Resource, Clone, Copy, Debug)]
pub struct RunSeed(pub u64);

#[derive(Resource, Default, Debug, Clone, Copy)]
pub struct RunStats {
    pub floors_cleared: u8,
    pub total_steps_taken: i32,
    pub enemies_defeated: u32,
    pub items_collected: u32,
}

impl RunStats {
    pub fn calculate_score(&self, floors_cleared: u8, steps_remaining: i32, force: i32) -> i32 {
        (floors_cleared as i32 * 100)
            + (steps_remaining * 2)
            + (force * 10)
            - (self.total_steps_taken / 2)
    }
}
