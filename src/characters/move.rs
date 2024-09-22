use crate::core::{
    map::{get_position_from_map, PathCost},
    setup::{CharacterId, Player},
};
use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::{AnimationLibrary, SpritesheetAnimation};
use std::collections::HashMap;

#[derive(Component)]
pub struct MovementState {
    pub target_position: Vec3,
    pub is_moving: bool,
}

#[derive(Resource, Default, Debug)]
pub struct CharacterPath {
    paths: HashMap<String, PathCost>,
}

impl CharacterPath {
    pub fn get_path_mut(&mut self, character_id: &str) -> Option<&mut PathCost> {
        self.paths.get_mut(character_id)
    }

    pub fn set_path(&mut self, character_id: &CharacterId, path_cost: PathCost) {
        self.paths = HashMap::new();
        self.paths.insert(character_id.0.clone(), path_cost);
    }

    pub fn remove_path(&mut self, character_id: &str) {
        self.paths.remove(character_id);
    }
}

#[allow(clippy::type_complexity)]
pub fn move_character(
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
    mut character_paths: ResMut<CharacterPath>,
) {
    const CHARACTER_SPEED: f32 = 320.;

    for (_entity, character_id, mut transform, movement_state, mut animation) in &mut characters {
        if let Some(mut movement_state) = movement_state {
            if movement_state.is_moving {
                // Get the path for the character
                if let Some(path_cost) = character_paths.get_path_mut(&character_id.0) {
                    // Move to the next position in the path
                    if let Some((next_x, next_y)) = path_cost.path.first() {
                        let next_position = get_position_from_map(*next_x, *next_y, None);

                        // Distance
                        let delta = next_position.translation - transform.translation;
                        let distance = delta.xy().length();

                        // If the character is close enough to the target position, consider it reached
                        if distance < 2.0 {
                            // Adjust this threshold as needed
                            transform.translation = next_position.translation;
                            path_cost.path.remove(0);
                        } else {
                            // Move smoothly towards the next position
                            let direction = (next_position.translation - transform.translation)
                                .normalize_or_zero();
                            transform.translation +=
                                direction * time.delta_seconds() * CHARACTER_SPEED;
                        }
                    } else {
                        // No more positions in the path, stop moving
                        movement_state.is_moving = false;
                        character_paths.remove_path(&character_id.0);
                        let subject = &character_id.0.split('_').next().expect("subject");
                        if let Some(idle_animation_id) =
                            library.animation_with_name(format!("{subject}_idle"))
                        {
                            animation.switch(idle_animation_id);
                        }
                    }
                } else {
                    // No path found, stop moving
                    movement_state.is_moving = false;
                }
            }
        }
    }
}
