use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use std::str::FromStr;
use strum_macros::{Display, EnumString};

use crate::{
    characters::r#move::{CharacterPath, MovementState},
    core::{
        chest::ChestState,
        map::{convert_map_to_screen, find_path},
    },
    timeline::{
        entity::{TimelineAction, TimelineClock},
        init::{CharacterTimelines, LookDirection},
    },
};

use super::{
    chest::Chests,
    gate::{GateState, Gates},
    map::get_position_from_map,
    setup::{CharacterId, Walkable},
};

#[derive(Component)]
#[allow(dead_code)]
pub struct Action(Act);

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, Display)]
pub enum Act {
    #[strum(serialize = "idle")]
    Idle,
    #[strum(serialize = "walk")]
    Walk,
    #[strum(serialize = "attack")]
    Attack,
    #[strum(serialize = "open")]
    Open,
    #[strum(serialize = "hurt")]
    Hurt,
    #[strum(serialize = "die")]
    Die,
}

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
pub fn schedule_timeline_actions(
    mut commands: Commands,
    library: Res<AnimationLibrary>,
    mut events: EventReader<AnimationEvent>,
    mut characters: Query<(
        Entity,
        &CharacterId,
        &mut Transform,
        &mut Sprite,
        &mut SpritesheetAnimation,
        Option<&Action>,
        Option<&mut MovementState>,
    )>,
    mut character_timelines: ResMut<CharacterTimelines>,
    mut chests: ResMut<Chests>,
    mut gates: ResMut<Gates>,
    mut character_path: ResMut<CharacterPath>,
    current_walkables: ResMut<Walkable>,
    mut timeline_clock: ResMut<TimelineClock>,
) {
    for (
        entity,
        character_id,
        mut transform,
        mut sprite,
        mut animation,
        _action,
        mut movement_state,
    ) in characters.iter_mut()
    {
        if let Some(timeline_actions) = character_timelines.0.get_mut(&character_id.0) {
            let mut actions_to_remove = Vec::new();

            for (i, action) in timeline_actions.0.iter().enumerate() {
                process_action(
                    &mut commands,
                    entity,
                    &mut transform,
                    &mut sprite,
                    &mut animation,
                    &mut movement_state,
                    &library,
                    character_id,
                    action,
                    &current_walkables,
                    &mut character_path,
                    &mut timeline_clock,
                );
                actions_to_remove.push(i);
            }

            // Remove processed actions
            for i in actions_to_remove.iter().rev() {
                timeline_actions.0.remove(*i);
            }
        }
    }

    // Remove the Action component when the corresponding animation ends
    for event in events.read() {
        if let AnimationEvent::AnimationRepetitionEnd {
            entity,
            animation_id,
            ..
        } = event
        {
            characters
                .iter_mut()
                .for_each(|(_entity, character_id, ..)| {
                    let subject = &character_id.0.split('_').next().expect("subject");
                    if library.is_animation_name(*animation_id, format!("{subject}_attack")) {
                        commands.entity(*entity).remove::<Action>();
                    }
                    if library.is_animation_name(*animation_id, format!("{subject}_hurt")) {
                        commands.entity(*entity).remove::<Action>();
                    }
                    if library.is_animation_name(*animation_id, format!("{subject}_die")) {
                        commands.entity(*entity).remove::<Action>();
                        commands.entity(*entity).despawn();
                    }
                    if library.is_animation_name(*animation_id, format!("{subject}_open")) {
                        commands.entity(*entity).remove::<Action>();

                        // Update the chest state after the player's "open" animation ends
                        if let Some(chest) = chests.0.get_mut("chest_0") {
                            chest.status = ChestState::Open;
                        }

                        // Update the chest state after the player's "open" animation ends
                        if let Some(gate) = gates.0.get_mut("gate_0") {
                            gate.status = GateState::Open;
                        }
                    }
                });
        }
    }
}

fn set_animation_and_action(
    library: &Res<AnimationLibrary>,
    animation: &mut SpritesheetAnimation,
    act: Act,
    subject: String,
    commands: &mut Commands,
    entity: Entity,
) {
    if let Some(animation_id) = library.animation_with_name(format!("{subject}_{act}")) {
        animation.switch(animation_id);
    }
    commands.entity(entity).insert(Action(act));
}

fn handle_movement_state(movement_state: &mut Option<Mut<MovementState>>, is_moving: bool) {
    if let Some(movement_state) = movement_state.as_mut() {
        movement_state.is_moving = is_moving;
    }
}

#[allow(clippy::too_many_arguments)]
fn process_action(
    commands: &mut Commands,
    entity: Entity,
    transform: &mut Transform,
    sprite: &mut Sprite,
    animation: &mut SpritesheetAnimation,
    movement_state: &mut Option<Mut<MovementState>>,
    library: &Res<AnimationLibrary>,
    character_id: &CharacterId,
    action: &TimelineAction,
    current_walkables: &ResMut<Walkable>,
    character_path: &mut ResMut<CharacterPath>,
    timeline_clock: &mut ResMut<TimelineClock>,
) {
    let cell_size = 46usize;
    let half_width = 320. / 2.;
    let half_height = 320. / 2.;
    let (offset_x, offset_y) = (0., 0.);

    let subject = character_id.0.split('_').next().expect("subject");

    // Subject position
    let at = convert_map_to_screen(action.at.clone()).expect("x,y");
    let current_transform = get_position_from_map(
        cell_size,
        half_width,
        half_height,
        offset_x,
        offset_y,
        at.0,
        at.1,
    );

    // Target position
    let mut to = at;
    let mut target_transform = *transform;
    if let Some(to_string) = &action.to {
        to = convert_map_to_screen(to_string.clone()).expect("x,y");
        target_transform = get_position_from_map(
            cell_size,
            half_width,
            half_height,
            offset_x,
            offset_y,
            to.0,
            to.1,
        );
    }

    let is_flip_x = match action.look {
        Some(LookDirection::Left) => true,
        Some(LookDirection::Right) => false,
        None => current_transform.translation.x > target_transform.translation.x,
    };
    sprite.flip_x = is_flip_x;

    println!("{:?} at {:?}", action.act.as_str(), action.at.clone());
    let act = action.act.as_str();
    match act {
        "walk" => {
            if let Some(walk_animation_id) = library.animation_with_name(format!("{subject}_walk"))
            {
                animation.switch(walk_animation_id);
            }
            if let Some(_to) = &action.to {
                if let Some(movement_state) = movement_state.as_mut() {
                    movement_state.target_position = target_transform.translation;
                    movement_state.is_moving = true;
                } else {
                    commands.entity(entity).insert(MovementState {
                        target_position: target_transform.translation,
                        is_moving: true,
                    });
                }

                // Find path
                println!("{:?} → {:?}", at, to);
                match find_path(&current_walkables.0, at, to) {
                    Ok(path_cost) => {
                        println!("set_path: {:?}→{:?}", character_id.0, path_cost);
                        character_path.set_path(character_id, path_cost)
                    }
                    Err(error) => println!("{error:?}"),
                };
            }
            commands.entity(entity).insert(Action(Act::Walk));
        }
        "idle" | "attack" | "open" | "hurt" | "die" => {
            handle_movement_state(movement_state, false);
            set_animation_and_action(
                library,
                animation,
                Act::from_str(act).expect("act"),
                subject.to_owned(),
                commands,
                entity,
            );
        }
        _ => (),
    }

    // Increment the timeline clock for this character
    timeline_clock.increment_time(&character_id.0, 1.0);
}
