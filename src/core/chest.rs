use bevy::{prelude::*, utils::HashMap};
use bevy_spritesheet_animation::prelude::*;

use super::scene::Decor;
use crate::core::position::Position;

#[derive(Resource, Default, Debug)]
pub struct Chests(pub HashMap<String, Chest>);

#[derive(Component, Debug, Clone)]
pub struct Chest {
    pub status: ChestState,
    // TODO
    #[allow(dead_code)]
    pub key: Option<String>,
}

#[derive(Component, Debug)]
pub struct ChestId(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChestState {
    Close,
    Open,
}

#[allow(clippy::type_complexity)]
pub fn update_chest(
    library: Res<AnimationLibrary>,
    mut chest: Query<(&ChestId, &mut SpritesheetAnimation), With<Decor>>,
    chests: Res<Chests>,
) {
    for (chest_id, mut animation) in chest.iter_mut() {
        if let Some(chest) = chests.0.get(&chest_id.0) {
            if chest.status == ChestState::Open {
                if let Some(open_animation_id) = library.animation_with_name("chest_open") {
                    animation.switch(open_animation_id);
                }
            }
        }
    }
}
