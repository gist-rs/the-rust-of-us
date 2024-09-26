use crate::core::layer::SpriteLayer;
use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use serde::Deserialize;

use super::{
    chest::Chests,
    gate::Gates,
    layer::YSort,
    library::{build_library, Ani},
    map::load_map_from_csv,
    scene::build_scene,
};

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    sprite_bundle: SpriteBundle,
    texture_atlas: TextureAtlas,
    spritesheet_animation: SpritesheetAnimation,
    sprite_layer: SpriteLayer,
    marker: Player,
    ysort: YSort,
}

#[derive(Component, Default, Clone, Deserialize, Debug, Eq, PartialEq)]
pub struct CharacterId(pub String);

pub fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut library: ResMut<AnimationLibrary>,
    chests: ResMut<Chests>,
    gates: ResMut<Gates>,
) {
    commands.spawn(Camera2dBundle::default());

    // Load map
    let (_walkables, start, goal, _path_cost, map) = load_map_from_csv("assets/map.csv").unwrap();

    build_scene(
        &mut commands,
        &asset_server,
        &mut atlas_layouts,
        &mut library,
        map,
        chests,
        gates,
        start,
        goal,
    );
}
