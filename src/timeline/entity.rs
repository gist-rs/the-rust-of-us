use bevy::prelude::Resource;

#[derive(Debug, Clone)]
pub struct TimelineAction {
    pub sec: f32,
    pub id: String,
    pub act: String,
    pub at: String,
    pub to: Option<String>,
}

#[derive(Resource, Default, Debug)]
pub struct TimelineActions(pub Vec<TimelineAction>);
