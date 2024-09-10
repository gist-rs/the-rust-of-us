use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::core::setup::Player;

// Component to check if a character is currently attack
#[derive(Component)]
pub struct Attack;

pub fn control_character(
    mut commands: Commands,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
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
) {
    // Control the character with the keyboard
    const CHARACTER_SPEED: f32 = 60.0;

    for (entity, mut transform, mut sprite, mut animation, attack) in &mut characters {
        // Except if they're attack, in which case we wait for the animation to end
        if attack.is_some() {
            continue;
        }

        // Attack
        if keyboard.pressed(KeyCode::Space) {
            println!("Space");
            // Set the animation
            if let Some(attack_animation_id) = library.animation_with_name("man_attack") {
                println!("+man_attack");
                animation.switch(attack_animation_id);
            }

            // Add a Attacking component
            println!("++man_attack");
            commands.entity(entity).insert(Attack);
        }
        // Move left
        else if keyboard.pressed(KeyCode::ArrowLeft) {
            // Set the animation
            if let Some(walk_animation_id) = library.animation_with_name("man_walk") {
                if animation.animation_id != walk_animation_id {
                    animation.switch(walk_animation_id);
                }
            }

            // Move
            transform.translation -= Vec3::X * time.delta_seconds() * CHARACTER_SPEED;
            sprite.flip_x = true;
        }
        // Move right
        else if keyboard.pressed(KeyCode::ArrowRight) {
            // Set the animation
            if let Some(walk_animation_id) = library.animation_with_name("man_walk") {
                if animation.animation_id != walk_animation_id {
                    animation.switch(walk_animation_id);
                }
            }

            // Move
            transform.translation += Vec3::X * time.delta_seconds() * CHARACTER_SPEED;
            sprite.flip_x = false;
        }
        // Move up
        else if keyboard.pressed(KeyCode::ArrowUp) {
            // Set the animation
            if let Some(walk_animation_id) = library.animation_with_name("man_walk") {
                if animation.animation_id != walk_animation_id {
                    animation.switch(walk_animation_id);
                }
            }

            // Move
            transform.translation += Vec3::Y * time.delta_seconds() * CHARACTER_SPEED;
        }
        // Move down
        else if keyboard.pressed(KeyCode::ArrowDown) {
            // Set the animation
            if let Some(walk_animation_id) = library.animation_with_name("man_walk") {
                if animation.animation_id != walk_animation_id {
                    animation.switch(walk_animation_id);
                }
            }

            // Move
            transform.translation -= Vec3::Y * time.delta_seconds() * CHARACTER_SPEED;
        }
        // Idle
        else {
            // Set the animation
            if let Some(idle_animation_id) = library.animation_with_name("man_idle") {
                if animation.animation_id != idle_animation_id {
                    animation.switch(idle_animation_id);
                }
            }
        }
    }

    // Remove the Attacking component when the attack animation ends
    // We use animation events to detect when this happens.
    for event in events.read() {
        match event {
            AnimationEvent::AnimationRepetitionEnd {
                entity,
                animation_id,
                ..
            } => {
                if library.is_animation_name(*animation_id, "man_attack") {
                    println!("-man_attack");
                    // commands.entity(*entity).remove::<Attack>();
                }
            }
            _ => (),
        }
    }
}
