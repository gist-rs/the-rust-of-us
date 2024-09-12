use std::fs;

use anyhow::*;
use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use csv::Reader;

use crate::characters::control::*;

use super::{scene::get_position_from_map, setup::Player};

#[derive(Debug)]
struct TimelineAction {
    sec: f32,
    id: String,
    act: String,
    at: String,
    to: Option<String>,
}

#[derive(Resource, Default, Debug)]
pub struct TimelineActions(Vec<TimelineAction>);

pub fn load_timeline_from_csv(file_path: &str) -> Result<TimelineActions> {
    let file_content = fs::read_to_string(file_path)?;
    let mut rdr = Reader::from_reader(file_content.as_bytes());

    let mut actions = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let action = TimelineAction {
            sec: record[0].parse()?,
            id: record[1].to_string(),
            act: record[2].to_string(),
            at: record[3].to_string(),
            to: record.get(4).map(|s| s.to_string()),
        };
        actions.push(action);
    }

    println!("🔥 {:#?}", actions);

    Ok(TimelineActions(actions))
}

pub fn schedule_timeline_actions(
    mut commands: Commands,
    time: Res<Time>,
    library: Res<AnimationLibrary>,
    mut events: EventReader<AnimationEvent>,
    mut characters: Query<
        (
            Entity,
            &mut Transform,
            &mut Sprite,
            &mut SpritesheetAnimation,
            Option<&Attack>,
        ),
        With<Player>,
    >,
    mut timeline_actions: ResMut<TimelineActions>,
) {
    let mut actions_to_remove = Vec::new();

    // println!("🔥 {:#?}", timeline_actions);

    for (i, action) in timeline_actions.0.iter().enumerate() {
        if time.elapsed_seconds() >= action.sec {
            actions_to_remove.push(i);
            let (entity, mut transform, mut sprite, mut animation, attack) = characters
                .iter_mut()
                .find(|(_, _, _, _, _)| {
                    // Assuming the character ID is stored in a component or some other way
                    // Here we just use the ID directly for simplicity
                    action.id == "man_0" || action.id == "skeleton_0"
                })
                .unwrap();

            let (x, y) = convert_map_to_screen(action.at.clone());
            let new_transform = get_position_from_map(32, 400.0, 300.0, 0.0, 0.0, x, y);
            transform.translation = new_transform.translation;

            println!("🔥 {}", action.act.as_str());
            match action.act.as_str() {
                "idle" => {
                    if let Some(idle_animation_id) = library.animation_with_name("man_idle") {
                        animation.switch(idle_animation_id);
                    }
                }
                "walk" => {
                    if let Some(walk_animation_id) = library.animation_with_name("man_walk") {
                        animation.switch(walk_animation_id);
                    }
                    if let Some(to) = &action.to {
                        let (x, y) = convert_map_to_screen(to.clone());
                        let target_transform =
                            get_position_from_map(32, 400.0, 300.0, 0.0, 0.0, x, y);
                        // Schedule a move to the target position
                        // This can be done using a timer or a tweening system
                    }
                }
                "attack" => {
                    if let Some(attack_animation_id) = library.animation_with_name("man_attack") {
                        animation.switch(attack_animation_id);
                    }
                    commands.entity(entity).insert(Attack);
                }
                "hurt" => {
                    if let Some(hurt_animation_id) = library.animation_with_name("man_hurt") {
                        animation.switch(hurt_animation_id);
                    }
                }
                "die" => {
                    if let Some(die_animation_id) = library.animation_with_name("man_die") {
                        animation.switch(die_animation_id);
                    }
                }
                _ => (),
            }
        }
    }

    // Remove processed actions
    for i in actions_to_remove.iter().rev() {
        timeline_actions.0.remove(*i);
    }

    // Remove the Attacking component when the attack animation ends
    for event in events.read() {
        match event {
            AnimationEvent::AnimationRepetitionEnd {
                entity,
                animation_id,
                ..
            } => {
                if library.is_animation_name(*animation_id, "man_attack") {
                    commands.entity(*entity).remove::<Attack>();
                }
            }
            _ => (),
        }
    }
}

fn convert_map_to_screen(map_coord: String) -> (usize, usize) {
    // Assuming map coordinates are in the format "a1", "b2", etc.
    let x = map_coord.chars().nth(0).unwrap() as usize - 'a' as usize;
    let y = map_coord.chars().nth(1).unwrap().to_digit(10).unwrap() as usize - 1;
    (x, y)
}
