use std::fs;

use anyhow::*;
use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use csv::Reader;

use crate::{
    characters::{control::*, position::MovementState},
    timeline::entity::TimelineActions,
};

use super::{scene::get_position_from_map, setup::Player};

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
            Option<&mut MovementState>,
        ),
        With<Player>,
    >,
    mut timeline_actions: ResMut<TimelineActions>,
) {
    let mut actions_to_remove = Vec::new();

    let cell_size = 46usize;
    let half_width = 320. / 2.;
    let half_height = 320. / 2.;
    let (offset_x, offset_y) = (0., 0.);

    for (i, action) in timeline_actions.0.iter().enumerate() {
        if time.elapsed_seconds() >= action.sec {
            actions_to_remove.push(i);
            let (entity, mut transform, mut sprite, mut animation, attack, mut movement_state) =
                characters
                    .iter_mut()
                    .find(|(_, _, _, _, _, _)| action.id == "man_0" || action.id == "skeleton_0")
                    .unwrap();

            let (x, y) = convert_map_to_screen(action.at.clone()).expect("x,y");
            let current_transform =
                get_position_from_map(cell_size, half_width, half_height, offset_x, offset_y, x, y);

            transform.translation = current_transform.translation;

            match action.act.as_str() {
                "idle" => {
                    if let Some(idle_animation_id) = library.animation_with_name("man_idle") {
                        animation.switch(idle_animation_id);
                    }
                    if let Some(mut movement_state) = movement_state {
                        movement_state.is_moving = false;
                    }
                }
                "walk" => {
                    if let Some(walk_animation_id) = library.animation_with_name("man_walk") {
                        animation.switch(walk_animation_id);
                    }
                    if let Some(to) = &action.to {
                        let (x, y) = convert_map_to_screen(to.clone()).expect("x,y");
                        let target_transform = get_position_from_map(
                            cell_size,
                            half_width,
                            half_height,
                            offset_x,
                            offset_y,
                            x,
                            y,
                        );
                        println!("+ move: {:#?}", target_transform.translation);
                        if let Some(mut movement_state) = movement_state {
                            movement_state.target_position = target_transform.translation;
                            movement_state.is_moving = true;
                        } else {
                            commands.entity(entity).insert(MovementState {
                                target_position: target_transform.translation,
                                is_moving: true,
                            });
                        }
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
fn convert_map_to_screen(map_coord: String) -> Option<(usize, usize)> {
    if map_coord.len() < 2 {
        return None;
    }

    let x = match map_coord.chars().nth(0).unwrap().to_ascii_lowercase() {
        'a'..='h' => map_coord.chars().nth(0).unwrap().to_ascii_lowercase() as usize - 'a' as usize,
        _ => return None,
    };

    let y = match map_coord.chars().nth(1).unwrap().to_digit(10) {
        Some(digit) if digit >= 1 && digit <= 8 => digit as usize - 1,
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
