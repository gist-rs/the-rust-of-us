use crate::core::setup::{Enemy, Player};
use bevy::prelude::*;

#[derive(Component)]
pub struct MovementState {
    pub target_position: Vec3,
    pub is_moving: bool,
}

pub fn move_character(
    mut _commands: Commands,
    time: Res<Time>,
    mut characters: Query<
        (Entity, &mut Transform, Option<&mut MovementState>),
        Or<(With<Player>, With<Enemy>)>,
    >,
) {
    const CHARACTER_SPEED: f32 = 150.0;

    for (_entity, mut transform, movement_state) in &mut characters {
        if let Some(mut movement_state) = movement_state {
            if movement_state.is_moving {
                let direction =
                    (movement_state.target_position - transform.translation).normalize_or_zero();
                transform.translation += direction * time.delta_seconds() * CHARACTER_SPEED;

                // Check if the character has reached the target position
                if transform
                    .translation
                    .distance(movement_state.target_position)
                    < 5.0
                {
                    println!("-is_moving");
                    movement_state.is_moving = false;
                }
            }
        }
    }
}
