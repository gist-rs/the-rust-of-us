use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::{
    characters::{control::*, r#move::MovementState},
    timeline::init::CharacterTimelines,
};

use super::{scene::get_position_from_map, setup::CharacterId};

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
        Option<&Attack>,
        Option<&mut MovementState>,
    )>,
    mut character_timelines: ResMut<CharacterTimelines>,
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
        _attack,
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
                    let (x, y) = convert_map_to_screen(action.at.clone()).expect("x,y");
                    let current_transform = get_position_from_map(
                        cell_size,
                        half_width,
                        half_height,
                        offset_x,
                        offset_y,
                        x,
                        y,
                    );

                    transform.translation = current_transform.translation;

                    // Target position
                    let mut target_transform = *transform;
                    if let Some(to) = &action.to {
                        let (x, y) = convert_map_to_screen(to.clone()).expect("x,y");
                        target_transform = get_position_from_map(
                            cell_size,
                            half_width,
                            half_height,
                            offset_x,
                            offset_y,
                            x,
                            y,
                        );
                    }
                    let is_flip_x =
                        current_transform.translation.x > target_transform.translation.x;

                    println!("action:{:?}", action.act.as_str());
                    match action.act.as_str() {
                        "idle" => {
                            if let Some(movement_state) = movement_state.as_mut() {
                                println!("-is_moving");
                                movement_state.is_moving = false;
                            }
                            if let Some(idle_animation_id) =
                                library.animation_with_name(format!("{subject}_idle"))
                            {
                                animation.switch(idle_animation_id);
                            }
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
                                    println!("+is_moving1");
                                    movement_state.is_moving = true;
                                } else {
                                    println!("+is_moving2");
                                    commands.entity(entity).insert(MovementState {
                                        target_position: target_transform.translation,
                                        is_moving: true,
                                    });
                                }
                            }
                            sprite.flip_x = is_flip_x;
                        }
                        "attack" => {
                            if let Some(movement_state) = movement_state.as_mut() {
                                println!("-is_moving");
                                movement_state.is_moving = false;
                            }
                            if let Some(attack_animation_id) =
                                library.animation_with_name(format!("{subject}_attack"))
                            {
                                animation.switch(attack_animation_id);
                            }
                            println!("+man_attack");
                            commands.entity(entity).insert(Attack);

                            sprite.flip_x = is_flip_x;
                        }
                        "hurt" => {
                            if let Some(movement_state) = movement_state.as_mut() {
                                println!("-is_moving");
                                movement_state.is_moving = false;
                            }
                            if let Some(hurt_animation_id) =
                                library.animation_with_name(format!("{subject}_hurt"))
                            {
                                animation.switch(hurt_animation_id);
                            }
                        }
                        "die" => {
                            if let Some(movement_state) = movement_state.as_mut() {
                                println!("-is_moving");
                                movement_state.is_moving = false;
                            }
                            if let Some(die_animation_id) =
                                library.animation_with_name(format!("{subject}_die"))
                            {
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
        }
    }

    // Remove the Attacking component when the attack animation ends
    for event in events.read() {
        match event {
            AnimationEvent::AnimationRepetitionEnd {
                entity,
                animation_id,
                ..
            } => {
                characters.iter_mut().for_each(
                    |(
                        _entity,
                        character_id,
                        mut _transform,
                        mut _sprite,
                        mut _animation,
                        _attack,
                        mut _movement_state,
                    )| {
                        let subject = &character_id.0.split('_').next().expect("subject");
                        if library.is_animation_name(*animation_id, format!("{subject}_attack")) {
                            commands.entity(*entity).remove::<Attack>();
                        }
                    },
                );
            }
            _ => (),
        }
    }
}

fn convert_map_to_screen(map_coord: String) -> Option<(usize, usize)> {
    if map_coord.len() < 2 {
        return None;
    }

    let x = match map_coord.chars().next().unwrap().to_ascii_lowercase() {
        'a'..='h' => map_coord.chars().next().unwrap().to_ascii_lowercase() as usize - 'a' as usize,
        _ => return None,
    };

    let y = match map_coord.chars().nth(1).unwrap().to_digit(10) {
        Some(digit) if (1..=8).contains(&digit) => digit as usize - 1,
        _ => return None,
    };

    Some((x, y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_map_to_screen() {
        // Test cases for the function
        assert_eq!(convert_map_to_screen("a1".to_string()), Some((0, 0)));
        assert_eq!(convert_map_to_screen("b1".to_string()), Some((1, 0)));
        assert_eq!(convert_map_to_screen("c1".to_string()), Some((2, 0)));
        assert_eq!(convert_map_to_screen("d1".to_string()), Some((3, 0)));
        assert_eq!(convert_map_to_screen("e1".to_string()), Some((4, 0)));
        assert_eq!(convert_map_to_screen("f1".to_string()), Some((5, 0)));
        assert_eq!(convert_map_to_screen("g1".to_string()), Some((6, 0)));
        assert_eq!(convert_map_to_screen("h1".to_string()), Some((7, 0)));

        assert_eq!(convert_map_to_screen("a2".to_string()), Some((0, 1)));
        assert_eq!(convert_map_to_screen("b2".to_string()), Some((1, 1)));
        assert_eq!(convert_map_to_screen("c2".to_string()), Some((2, 1)));
        assert_eq!(convert_map_to_screen("d2".to_string()), Some((3, 1)));
        assert_eq!(convert_map_to_screen("e2".to_string()), Some((4, 1)));
        assert_eq!(convert_map_to_screen("f2".to_string()), Some((5, 1)));
        assert_eq!(convert_map_to_screen("g2".to_string()), Some((6, 1)));
        assert_eq!(convert_map_to_screen("h2".to_string()), Some((7, 1)));

        assert_eq!(convert_map_to_screen("a8".to_string()), Some((0, 7)));
        assert_eq!(convert_map_to_screen("b8".to_string()), Some((1, 7)));
        assert_eq!(convert_map_to_screen("c8".to_string()), Some((2, 7)));
        assert_eq!(convert_map_to_screen("d8".to_string()), Some((3, 7)));
        assert_eq!(convert_map_to_screen("e8".to_string()), Some((4, 7)));
        assert_eq!(convert_map_to_screen("f8".to_string()), Some((5, 7)));
        assert_eq!(convert_map_to_screen("g8".to_string()), Some((6, 7)));
        assert_eq!(convert_map_to_screen("h8".to_string()), Some((7, 7)));

        // Test case for invalid input
        assert_eq!(convert_map_to_screen("i1".to_string()), None);
        assert_eq!(convert_map_to_screen("a9".to_string()), None);
        assert_eq!(convert_map_to_screen("a".to_string()), None);
        assert_eq!(convert_map_to_screen("".to_string()), None);
    }
}
