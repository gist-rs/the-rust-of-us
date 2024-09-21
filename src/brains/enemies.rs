use bevy::{prelude::*, utils::HashMap};
use bevy_spritesheet_animation::prelude::*;

use crate::{characters::bar::EnemyCharacter, Position};

#[allow(clippy::type_complexity)]
pub fn update_enemy(
    mut enemy_positions: Query<(&mut Position, &mut Transform), With<EnemyCharacter>>,
) {
    for (enemy_position, mut enemy_transform) in enemy_positions.iter_mut() {
        enemy_transform.translation.x = enemy_position.position.x;
        enemy_transform.translation.y = enemy_position.position.y;
    }
}
