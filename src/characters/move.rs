use crate::core::{
    map::{convert_map_to_screen, get_map_from_position, get_position_from_map},
    setup::{CharacterId, Enemy, Player},
};
use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::{AnimationLibrary, SpritesheetAnimation};

#[derive(Component)]
pub struct MovementState {
    pub target_position: Vec3,
    pub is_moving: bool,
}

pub fn move_character(
    mut _commands: Commands,
    library: Res<AnimationLibrary>,
    time: Res<Time>,
    mut characters: Query<
        (
            Entity,
            &CharacterId,
            &mut Transform,
            Option<&mut MovementState>,
            &mut SpritesheetAnimation,
        ),
        With<Player>,
    >,
) {
    const CHARACTER_SPEED: f32 = 150.0;
    const CELL_SIZE: usize = 46;
    const HALF_WIDTH: f32 = 320. / 2.;
    const HALF_HEIGHT: f32 = 320. / 2.;
    const OFFSET_X: f32 = 0.;
    const OFFSET_Y: f32 = 0.;

    for (_entity, character_id, mut transform, movement_state, mut animation) in &mut characters {
        if let Some(mut movement_state) = movement_state {
            if movement_state.is_moving {
                let direction =
                    (movement_state.target_position - transform.translation).normalize_or_zero();
                transform.translation += direction * time.delta_seconds() * CHARACTER_SPEED;

                // Convert current and target positions to map coordinates
                let current_map_position = get_map_from_position(
                    CELL_SIZE,
                    HALF_WIDTH,
                    HALF_HEIGHT,
                    OFFSET_X,
                    OFFSET_Y,
                    Transform {
                        translation: transform.translation,
                        ..Default::default()
                    },
                );
                let target_map_position = get_map_from_position(
                    CELL_SIZE,
                    HALF_WIDTH,
                    HALF_HEIGHT,
                    OFFSET_X,
                    OFFSET_Y,
                    Transform {
                        translation: movement_state.target_position,
                        ..Default::default()
                    },
                );

                // Check if the character has reached the target position
                if current_map_position == target_map_position {
                    transform.translation = movement_state.target_position;

                    movement_state.is_moving = false;
                    let subject = &character_id.0.split('_').next().expect("subject");
                    if let Some(idle_animation_id) =
                        library.animation_with_name(format!("{subject}_idle"))
                    {
                        animation.switch(idle_animation_id);
                    }
                }
            }
        }
    }
}
