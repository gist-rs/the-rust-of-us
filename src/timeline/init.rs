use bevy::prelude::*;

use std::fs;

use anyhow::*;
use csv::Reader;

use super::entity::{TimelineAction, TimelineActions};

use std::collections::HashMap;

#[derive(Resource, Default, Debug)]
pub struct CharacterTimelines(pub HashMap<String, TimelineActions>);

pub fn load_timeline_from_csv(file_path: &str) -> Result<CharacterTimelines> {
    let file_content = fs::read_to_string(file_path).expect("Expected timeline.csv");
    let mut rdr = Reader::from_reader(file_content.as_bytes());

    let mut timelines = HashMap::new();

    for result in rdr.records() {
        let record = result.expect("a CSV record");
        let action = TimelineAction {
            sec: record[0].parse()?,
            id: record[1].to_string(),
            act: record[2].to_string(),
            at: record[3].to_string(),
            to: record.get(4).map(|s| s.to_string()),
        };

        timelines
            .entry(action.id.clone())
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

pub fn init_timeline(_commands: Commands, mut character_timelines: ResMut<CharacterTimelines>) {
    let timelines = load_timeline_from_csv("assets/timeline.csv").expect("timeline.csv");
    *character_timelines = timelines;
}
