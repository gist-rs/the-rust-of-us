use anyhow::*;
use bevy::prelude::*;
use serde::Deserialize;
use serde_yaml::from_str;
use std::collections::HashMap;
use std::fs;

use super::entity::{TimelineAction, TimelineActions, TimelineClock};

#[derive(Resource, Default, Debug)]
pub struct CharacterTimelines(pub HashMap<String, TimelineActions>);

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LookDirection {
    Left,
    Right,
}

#[derive(Deserialize, Debug)]
pub struct YamlTimelineAction {
    pub id: String,
    pub act: String,
    pub at: String,
    pub to: Option<String>,
    pub look: Option<LookDirection>,
}

pub fn load_timeline_from_yaml(file_path: &str) -> Result<CharacterTimelines> {
    let file_content = fs::read_to_string(file_path).expect("Expected timeline.yml");
    let yaml_actions: Vec<YamlTimelineAction> = from_str(&file_content)?;

    let mut timelines = HashMap::new();

    for yaml_action in yaml_actions {
        let action = TimelineAction {
            id: yaml_action.id.clone(),
            act: yaml_action.act,
            at: yaml_action.at,
            to: yaml_action.to,
            look: yaml_action.look,
        };

        timelines
            .entry(yaml_action.id)
            .or_insert_with(Vec::new)
            .push(action);
    }

    // Wrap each Vec<TimelineAction> in TimelineActions
    let wrapped_timelines: HashMap<String, TimelineActions> = timelines
        .into_iter()
        .map(|(id, actions)| (id, TimelineActions(actions)))
        .collect();

    Ok(CharacterTimelines(wrapped_timelines))
}

pub fn init_timeline(mut commands: Commands, mut character_timelines: ResMut<CharacterTimelines>) {
    let timelines = load_timeline_from_yaml("assets/timeline.yml").expect("timeline.yml");
    *character_timelines = timelines;

    // Initialize the TimelineClock
    let mut timeline_clock = TimelineClock::default();
    for character_id in character_timelines.0.keys() {
        timeline_clock.set_time(character_id, 0.0);
    }
    commands.insert_resource(timeline_clock);
}
