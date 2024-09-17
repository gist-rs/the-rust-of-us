use crate::core::{
    map::{get_map_from_position, PathCost},
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
    pub fn new() -> Self {
        Self {
            paths: HashMap::new(),
        }
    }

    pub fn get_path_mut(&mut self, character_id: &str) -> Option<&mut PathCost> {
        self.paths.get_mut(character_id)
    }

    pub fn set_path(&mut self, character_id: &CharacterId, path_cost: PathCost) {
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
    const CHARACTER_SPEED: f32 = 150.0;
    const CELL_SIZE: usize = 46;
    const HALF_WIDTH: f32 = 320. / 2.;
    const HALF_HEIGHT: f32 = 320. / 2.;
    const OFFSET_X: f32 = 0.;
    const OFFSET_Y: f32 = 0.;

    for (_entity, character_id, mut transform, movement_state, mut animation) in &mut characters {
        if let Some(mut movement_state) = movement_state {
            if movement_state.is_moving {
                // Get the path for the character
                if let Some(path_cost) = character_paths.get_path_mut(&character_id.0) {
                    // Move to the next position in the path
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
                        // Move to the next position in the path
                        if let Some((next_x, next_y)) = path_cost.path.first() {
                            let next_position = Vec3::new(
                                *next_x as f32 * CELL_SIZE as f32 - HALF_WIDTH + OFFSET_X,
                                *next_y as f32 * CELL_SIZE as f32 - HALF_HEIGHT + OFFSET_Y,
                                0.0,
                            );
                            movement_state.target_position = next_position;
                            path_cost.path.remove(0);
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
                    }

                    let direction = (movement_state.target_position - transform.translation)
                        .normalize_or_zero();
                    transform.translation += direction * time.delta_seconds() * CHARACTER_SPEED;
                } else {
                    // No path found, stop moving
                    movement_state.is_moving = false;
                }
            }
        }
    }
}
