use bevy::prelude::*;

use std::fs;

use anyhow::*;
use csv::Reader;

use super::entity::{TimelineAction, TimelineActions};

pub fn load_timeline_from_csv(file_path: &str) -> Result<TimelineActions> {
    // Read the CSV file
    let file_content = fs::read_to_string(file_path).expect("Expected timeline.csv");
    let mut rdr = Reader::from_reader(file_content.as_bytes());

    let mut actions = Vec::new();

    // Parse the CSV data
    for result in rdr.records() {
        let record = result.expect("a CSV record");
        let action = TimelineAction {
            sec: record[0].parse()?,
            id: record[1].to_string(),
            act: record[2].to_string(),
            at: record[3].to_string(),
            to: record.get(4).map(|s| s.to_string()),
        };
        actions.push(action);
    }

    Ok(TimelineActions(actions))
}

pub fn init_timeline(mut commands: Commands, mut timeline_actions: ResMut<TimelineActions>) {
    let actions = load_timeline_from_csv("assets/timeline.csv").expect("timeline.csv");
    *timeline_actions = actions
}
