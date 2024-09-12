use std::fs;

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use serde_json::from_str;

use super::{
    layer::{SpriteLayer, YSort},
    library::{build_library, Ani},
};

#[derive(Resource)]
pub struct GameMap(pub Vec<Vec<String>>);

#[derive(Component)]
pub struct Decor;

#[derive(Bundle)]
struct DecorBundle {
    sprite_bundle: SpriteBundle,
    sprite_layer: SpriteLayer,
    marker: Decor,
    ysort: YSort,
}

pub fn build_scene(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    library: &mut ResMut<AnimationLibrary>,
    map: GameMap,
) {
    // Spawn the background
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(10.0)),
            texture: asset_server.load("grass.png"),
            ..default()
        },
        SpriteLayer::Background,
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: true,
            stretch_value: 0.25,
        },
    ));

    // Load decor from JSON file
    let decor_json = fs::read_to_string("assets/decor.json").expect("Unable to read file");
    let object_animations: Vec<Ani> = from_str(&decor_json).expect("Unable to parse JSON");

    // Spawn obstacles based on the map
    let cell_size = 46.;
    let half_width = 320. / 2.;
    let half_height = 320. / 2.;
    for (y, row) in map.0.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            match cell.as_str() {
                "🌳" => {
                    commands.spawn(DecorBundle {
                        sprite_bundle: SpriteBundle {
                            texture: asset_server.load("tree.png"),
                            transform: Transform::from_xyz(
                                cell_size * x as f32 - half_width,
                                cell_size * y as f32 - half_height + 30.,
                                0.0,
                            )
                            .with_scale(Vec3::splat(2.0)),
                            ..default()
                        },
                        sprite_layer: SpriteLayer::Ground,
                        marker: Decor,
                        ysort: YSort(0.0),
                    });
                }
                "🦀" => {
                    commands.spawn(DecorBundle {
                        sprite_bundle: SpriteBundle {
                            texture: asset_server.load("crab.png"),
                            transform: Transform::from_xyz(
                                cell_size * x as f32 - half_width,
                                cell_size * y as f32 - half_height + 30.,
                                0.0,
                            )
                            .with_scale(Vec3::splat(1.0)),
                            ..default()
                        },
                        sprite_layer: SpriteLayer::Ground,
                        marker: Decor,
                        ysort: YSort(0.0),
                    });
                }
                "💰" => {
                    let ani = object_animations
                        .iter()
                        .find(|ani| ani.name == "chest")
                        .expect("Expected chest");
                    let deco_bundle = build_decor_bundle(
                        asset_server,
                        atlas_layouts,
                        library,
                        ani,
                        Transform::from_xyz(
                            cell_size * x as f32 - half_width,
                            cell_size * y as f32 - half_height + 30.,
                            0.0,
                        )
                        .with_scale(Vec3::splat(2.0)),
                    );

                    commands.spawn(deco_bundle);
                }
                _ => (),
            }
        }
    }
}

#[derive(Bundle)]
pub struct AniDecorBundle {
    sprite_bundle: SpriteBundle,
    texture_atlas: TextureAtlas,
    spritesheet_animation: SpritesheetAnimation,
    sprite_layer: SpriteLayer,
    marker: Decor,
    ysort: YSort,
}

pub fn build_decor_bundle(
    asset_server: &Res<AssetServer>,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    library: &mut ResMut<AnimationLibrary>,
    ani: &Ani,
    transform: Transform,
) -> AniDecorBundle {
    let clip_fps = 30;

    let libs = build_library(atlas_layouts, library, ani, clip_fps);

    let texture_path = ani.texture_path.clone();
    let texture = asset_server.load(texture_path);

    AniDecorBundle {
        sprite_bundle: SpriteBundle {
            texture,
            transform,
            ..default()
        },
        texture_atlas: TextureAtlas {
            layout: libs[0].1.clone(),
            ..default()
        },
        spritesheet_animation: SpritesheetAnimation::from_id(libs[0].0),
        sprite_layer: SpriteLayer::Ground,
        marker: Decor,
        ysort: YSort(0.0),
    }
}
