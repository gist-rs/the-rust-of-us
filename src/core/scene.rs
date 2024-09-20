use std::fs;

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use serde_json::from_str;

use super::{
    chest::{Chest, ChestId, ChestState, Chests},
    gate::{Gate, GateState, Gates},
    layer::{SpriteLayer, YSort},
    library::{build_library, Ani},
    map::{get_position_from_map, PathCost},
};

#[derive(Resource, Default, Debug)]
pub struct GameMap(pub Vec<Vec<String>>);

#[derive(Resource, Default, Debug)]
pub struct MainPath(pub PathCost);

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
    mut chests: ResMut<Chests>,
    mut gates: ResMut<Gates>,
) {
    // Spawn the background
    let transform = Transform::from_scale(Vec3::splat(11.5));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("grass.png"),
            transform: transform.with_translation(Vec3::new(
                transform.translation.x,
                transform.translation.y + 20.,
                transform.translation.z,
            )),
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
    let decor_animations: Vec<Ani> = from_str(&decor_json).expect("Unable to parse JSON");

    // Spawn obstacles based on the map
    let cell_size = 46usize;
    let half_width = 320. / 2.;
    let half_height = 320. / 2.;
    let (offset_x, offset_y) = (0., 0.);

    for (y, row) in map.0.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let transform =
                get_position_from_map(cell_size, half_width, half_height, offset_x, offset_y, x, y);
            match cell.as_str() {
                "🌳" => {
                    commands.spawn(DecorBundle {
                        sprite_bundle: SpriteBundle {
                            texture: asset_server.load("tree.png"),
                            transform: transform.with_scale(Vec3::splat(2.0)),
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
                            transform: transform.with_scale(Vec3::splat(2.0)).with_translation(
                                Vec3::new(
                                    transform.translation.x,
                                    transform.translation.y - 8.,
                                    transform.translation.z,
                                ),
                            ),
                            ..default()
                        },
                        sprite_layer: SpriteLayer::Ground,
                        marker: Decor,
                        ysort: YSort(0.0),
                    });
                }
                "🚪" => {
                    let ani = decor_animations
                        .iter()
                        .find(|ani| ani.name == "gate")
                        .expect("Expected gate");
                    let deco_bundle = build_ani_decor_bundle(
                        "gate_close".to_owned(),
                        asset_server,
                        atlas_layouts,
                        library,
                        ani,
                        transform.with_scale(Vec3::splat(2.0)),
                    );

                    let gate_id = format!("gate_{}", gates.0.len());
                    commands.spawn(deco_bundle).insert(ChestId(gate_id.clone()));

                    gates.0.insert(
                        gate_id,
                        Gate {
                            status: GateState::Close,
                            key: None,
                        },
                    );
                }
                "💰" => {
                    let ani = decor_animations
                        .iter()
                        .find(|ani| ani.name == "chest")
                        .expect("Expected chest");
                    let deco_bundle = build_ani_decor_bundle(
                        "chest_close".to_owned(),
                        asset_server,
                        atlas_layouts,
                        library,
                        ani,
                        transform
                            .with_scale(Vec3::splat(2.0))
                            .with_translation(Vec3::new(
                                transform.translation.x,
                                transform.translation.y - 8.,
                                transform.translation.z,
                            )),
                    );

                    let chest_id = format!("chest_{}", chests.0.len());
                    commands
                        .spawn(deco_bundle)
                        .insert(ChestId(chest_id.clone()));

                    chests.0.insert(
                        chest_id,
                        Chest {
                            status: ChestState::Close,
                            key: None,
                        },
                    );
                }
                "🪦" => {
                    commands.spawn(DecorBundle {
                        sprite_bundle: SpriteBundle {
                            texture: asset_server.load("grave.png"),
                            transform: transform.with_scale(Vec3::splat(2.0)).with_translation(
                                Vec3::new(
                                    transform.translation.x,
                                    transform.translation.y - 4.,
                                    transform.translation.z,
                                ),
                            ),
                            ..default()
                        },
                        sprite_layer: SpriteLayer::Ground,
                        marker: Decor,
                        ysort: YSort(0.0),
                    });
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

pub fn build_ani_decor_bundle(
    animation_name: String,
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
        spritesheet_animation: SpritesheetAnimation::from_id(
            library.animation_with_name(animation_name).unwrap(),
        ),
        sprite_layer: SpriteLayer::Ground,
        marker: Decor,
        ysort: YSort(0.0),
    }
}
