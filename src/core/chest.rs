use bevy::{prelude::*, utils::HashMap};
use bevy_spritesheet_animation::prelude::*;

use crate::brains::loot::{Loot, Looted};

use super::scene::Decor;

#[derive(Resource, Default, Debug)]
pub struct Chests(pub HashMap<String, Chest>);

#[derive(Component, Debug, Clone)]
pub struct Chest {
    pub status: ChestState,
    // TODO
    #[allow(dead_code)]
    pub key: Option<String>,
}

#[derive(Component, Debug, Default, Clone, PartialEq, Eq)]
pub struct ChestId(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChestState {
    Close,
    Open,
}

#[allow(clippy::type_complexity)]
pub fn update_chest(
    mut commands: Commands,
    library: Res<AnimationLibrary>,
    mut chest_query: Query<
        (&ChestId, &mut SpritesheetAnimation, Entity),
        (With<Chest>, Without<Looted>),
    >,
    chests: Res<Chests>,
) {
    for (chest_id, mut animation, entity) in chest_query.iter_mut() {
        if let Some(chest) = chests.0.get(&chest_id.0) {
            if chest.status == ChestState::Open {
                if let Some(open_animation_id) = library.animation_with_name("chest_open") {
                    animation.switch(open_animation_id);
                }

                println!("😱 Looted!!!!");
                commands.entity(entity).insert(Looted);
            }
        }
    }
}
