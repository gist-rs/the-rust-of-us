use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::core::setup::Player;

#[derive(Component)]
pub struct MovementState {
    pub target_position: Vec3,
    pub is_moving: bool,
}

pub fn move_character(
    mut commands: Commands,
    time: Res<Time>,
    library: Res<AnimationLibrary>,
    mut characters: Query<
        (
            Entity,
            &mut Transform,
            &mut Sprite,
            &mut SpritesheetAnimation,
            Option<&mut MovementState>,
        ),
        With<Player>,
    >,
) {
    const CHARACTER_SPEED: f32 = 60.0;

    for (entity, mut transform, mut sprite, mut animation, movement_state) in &mut characters {
        if let Some(mut movement_state) = movement_state {
            if movement_state.is_moving {
                let direction =
                    (movement_state.target_position - transform.translation).normalize_or_zero();
                transform.translation += direction * time.delta_seconds() * CHARACTER_SPEED;

                // Check if the character has reached the target position
                if transform
                    .translation
                    .distance(movement_state.target_position)
                    < 1.0
                {
                    movement_state.is_moving = false;
                    if let Some(idle_animation_id) = library.animation_with_name("man_idle") {
                        animation.switch(idle_animation_id);
                    }
                } else {
                    // Flip the sprite based on the movement direction
                    sprite.flip_x = direction.x < 0.0;
                }
            }
        }
    }
}
