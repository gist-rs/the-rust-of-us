use bevy::{prelude::*, utils::HashMap};
use bevy_spritesheet_animation::prelude::*;

use super::scene::Decor;

#[derive(Resource, Default, Debug)]
pub struct Chests(pub HashMap<String, Chest>);

#[derive(Debug, Clone)]
pub struct Chest {
    pub status: ChestState,
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
    mut chest: Query<(Entity, &ChestId, &mut SpritesheetAnimation), With<Decor>>,
    chests: Res<Chests>,
) {
    for (entity, chest_id, mut animation) in chest.iter_mut() {
        // println!("chest_id:{chest_id:?}");
        if let Some(chest) = chests.0.get(&chest_id.0) {
            if chest.status == ChestState::Open {
                if let Some(open_animation_id) = library.animation_with_name("chest_open") {
                    println!("chest:{chest:?}");
                    animation.switch(open_animation_id);
                }
            }
        }
    }
}
