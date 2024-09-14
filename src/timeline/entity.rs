use bevy::prelude::Resource;

use super::init::LookDirection;

#[derive(Debug, Clone)]
pub struct TimelineAction {
    pub sec: f32,
    pub id: String,
    pub act: String,
    pub at: String,
    pub to: Option<String>,
    pub look: Option<LookDirection>,
}

#[derive(Resource, Default, Debug)]
pub struct TimelineActions(pub Vec<TimelineAction>);
