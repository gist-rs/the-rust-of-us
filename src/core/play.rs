use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::{
    characters::r#move::{CharacterPath, MovementState},
    core::{
        chest::ChestState,
        map::{convert_map_to_screen, find_path},
    },
    timeline::init::{CharacterTimelines, LookDirection},
};

use super::{
    chest::Chests,
    map::get_position_from_map,
    setup::{CharacterId, Walkable},
};

#[derive(Component)]
#[allow(dead_code)]
pub struct Action(Act);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Act {
    Idle,
    Walk,
    Attack,
    Open,
    Hurt,
    Die,
}

#[allow(clippy::type_complexity)]
pub fn schedule_timeline_actions(
    mut commands: Commands,
    time: Res<Time>,
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
    mut character_path: ResMut<CharacterPath>,
    current_walkables: ResMut<Walkable>,
) {
    let cell_size = 46usize;
    let half_width = 320. / 2.;
    let half_height = 320. / 2.;
    let (offset_x, offset_y) = (0., 0.);

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

            let subject = &character_id.0.split('_').next().expect("subject");
            for (i, action) in timeline_actions.0.iter().enumerate() {
                if time.elapsed_seconds() >= action.sec {
                    actions_to_remove.push(i);

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

                    transform.translation = current_transform.translation;

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

                    println!("{:?} at {:?}", action.act.as_str(), action.at.clone());
                    match action.act.as_str() {
                        "idle" => {
                            if let Some(movement_state) = movement_state.as_mut() {
                                movement_state.is_moving = false;
                            }
                            if let Some(idle_animation_id) =
                                library.animation_with_name(format!("{subject}_idle"))
                            {
                                animation.switch(idle_animation_id);
                            }
                            commands.entity(entity).insert(Action(Act::Idle));
                            sprite.flip_x = is_flip_x;
                        }
                        "walk" => {
                            if let Some(walk_animation_id) =
                                library.animation_with_name(format!("{subject}_walk"))
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
                            sprite.flip_x = is_flip_x;
                        }
                        "attack" => {
                            if let Some(movement_state) = movement_state.as_mut() {
                                movement_state.is_moving = false;
                            }
                            if let Some(attack_animation_id) =
                                library.animation_with_name(format!("{subject}_attack"))
                            {
                                animation.switch(attack_animation_id);
                            }
                            commands.entity(entity).insert(Action(Act::Attack));
                            sprite.flip_x = is_flip_x;
                        }
                        "open" => {
                            if let Some(movement_state) = movement_state.as_mut() {
                                movement_state.is_moving = false;
                            }
                            if let Some(attack_animation_id) =
                                library.animation_with_name(format!("{subject}_open"))
                            {
                                animation.switch(attack_animation_id);
                            }
                            commands.entity(entity).insert(Action(Act::Open));
                            sprite.flip_x = is_flip_x;
                        }
                        "hurt" => {
                            if let Some(movement_state) = movement_state.as_mut() {
                                movement_state.is_moving = false;
                            }
                            if let Some(hurt_animation_id) =
                                library.animation_with_name(format!("{subject}_hurt"))
                            {
                                animation.switch(hurt_animation_id);
                            }
                            commands.entity(entity).insert(Action(Act::Hurt));
                        }
                        "die" => {
                            if let Some(movement_state) = movement_state.as_mut() {
                                movement_state.is_moving = false;
                            }
                            if let Some(die_animation_id) =
                                library.animation_with_name(format!("{subject}_die"))
                            {
                                animation.switch(die_animation_id);
                            }
                            commands.entity(entity).insert(Action(Act::Die));
                        }
                        _ => (),
                    }
                }
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
            characters.iter_mut().for_each(
                |(
                    _entity,
                    character_id,
                    mut _transform,
                    mut _sprite,
                    mut _animation,
                    _action,
                    mut _movement_state,
                )| {
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
                    }
                },
            );
        }
    }
}
