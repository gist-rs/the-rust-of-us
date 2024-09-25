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

#[derive(Component, Clone, Deserialize, Debug, Eq, PartialEq)]
pub struct CharacterId(pub String);

#[allow(unused)]
pub fn build_player(
    asset_server: &Res<AssetServer>,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    library: &mut ResMut<AnimationLibrary>,
    ani: Ani,
) -> PlayerBundle {
    let clip_fps = 30;

    let libs = build_library(atlas_layouts, library, &ani, clip_fps);

    let texture_path = ani.texture_path.clone();
    let texture = asset_server.load(texture_path);

    PlayerBundle {
        sprite_bundle: SpriteBundle {
            texture,
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(2.0)),
            ..default()
        },
        texture_atlas: TextureAtlas {
            layout: libs[0].1.clone(),
            ..default()
        },
        spritesheet_animation: SpritesheetAnimation::from_id(
            library.animation_with_name("man_idle").unwrap(),
        ),
        sprite_layer: SpriteLayer::Ground,
        marker: Player,
        ysort: YSort(0.0),
    }
}

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
