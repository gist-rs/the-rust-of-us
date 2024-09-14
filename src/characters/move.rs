use crate::core::setup::{Enemy, Player};
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
            &mut Transform,
            Option<&mut MovementState>,
            &mut SpritesheetAnimation,
        ),
        With<Player>,
    >,
) {
    const CHARACTER_SPEED: f32 = 150.0;

    for (_entity, mut transform, movement_state, mut animation) in &mut characters {
        if let Some(mut movement_state) = movement_state {
            if movement_state.is_moving {
                let direction =
                    (movement_state.target_position - transform.translation).normalize_or_zero();
                transform.translation += direction * time.delta_seconds() * CHARACTER_SPEED;

                println!(
                    "{:?}",
                    transform
                        .translation
                        .distance(movement_state.target_position)
                );

                // Check if the character has reached the target position
                if transform
                    .translation
                    .distance(movement_state.target_position)
                    < 21.5
                {
                    movement_state.is_moving = false;
                    println!("++man_idle");
                    if let Some(idle_animation_id) =
                        library.animation_with_name(format!("man_idle"))
                    {
                        animation.switch(idle_animation_id);
                    }
                }
            }
        }
    }
}
