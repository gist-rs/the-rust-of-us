use super::init::LookDirection;
use bevy::prelude::Resource;

#[derive(Debug, Clone)]
pub struct TimelineAction {
    #[allow(dead_code)]
    pub id: String,
    pub act: String,
    pub at: String,
    pub to: Option<String>,
    pub look: Option<LookDirection>,
}

#[derive(Resource, Default, Debug)]
pub struct TimelineActions(pub Vec<TimelineAction>);

use std::collections::HashMap;

#[derive(Resource, Default, Debug)]
pub struct TimelineClock(HashMap<String, f32>);

impl TimelineClock {
    pub fn get_time(&self, character_id: &str) -> f32 {
        *self.0.get(character_id).unwrap_or(&0.0)
    }

    pub fn set_time(&mut self, character_id: &str, time: f32) {
        self.0.insert(character_id.to_string(), time);
    }

    pub fn increment_time(&mut self, character_id: &str, increment: f32) {
        let current_time = self.get_time(character_id);
        self.set_time(character_id, current_time + increment);
    }
}
