use std::fs;

use bevy::{color::palettes::css::RED, prelude::*};
use bevy_spritesheet_animation::prelude::*;
use serde_json::from_str;

use crate::{
    animations::{
        build::build_library,
        entities::{self, Ani, AniType},
    },
    entry::game::OnGameScreen,
};

use super::{
    chest::{Chest, ChestId, ChestState, Chests},
    gate::{Gate, GateState, Gates},
    grave::Grave,
    layer::{SpriteLayer, YSort},
    map::{get_position_from_map, MapConfig, MapPosition, PathCost},
    point::{Entrance, Exit},
    position::Position,
};

#[derive(Resource, Default, Debug, Clone)]
pub struct GameMap(pub Vec<Vec<String>>);

#[derive(Resource, Default, Debug)]
pub struct ChunkMap {
    pub walkables: Vec<Vec<bool>>,
    pub entrance: MapPosition,
    pub exit: MapPosition,
    pub graves: Vec<MapPosition>,
}

#[allow(unused)]
#[derive(Resource, Default, Debug)]
pub struct MainPath(pub PathCost);

#[derive(Component, Clone, Debug, Default)]
pub struct Decor;

#[derive(Bundle)]
struct DecorBundle {
    sprite_bundle: SpriteBundle,
    sprite_layer: SpriteLayer,
    marker: Decor,
    ysort: YSort,
}

#[allow(clippy::too_many_arguments)]
pub fn build_scene(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    library: &mut ResMut<AnimationLibrary>,
    map: GameMap,
    mut chests: ResMut<Chests>,
    mut gates: ResMut<Gates>,
    entrance: MapPosition,
    exit: MapPosition,
    map_config: Res<MapConfig>,
) -> Vec<(Entity, Chest)> {
    // TODO: Didn't help
    // child.spawn(NodeBundle {
    //     background_color: Color::srgba(1., 0., 0., 0.1).into(),
    //     style: Style {
    //         // position_type: PositionType::Absolute,
    //         left: Val::Px(0.0),
    //         bottom: Val::Px(0.0),
    //         width: Val::Px(320.0),
    //         height: Val::Px(320.0),
    //         ..default()
    //     },
    //     ..default()
    // });

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
    // let decor_json = fs::read_to_string("assets/decor.json").expect("Unable to read file");
    let decor_json = r#"[
  {
    "ani_type": "chest",
    "texture_path": "chest.png",
    "width": 18,
    "height": 29,
    "animations": [
      {
        "action_name": "close",
        "x": 0,
        "y": 0,
        "count": 1
      },
      {
        "action_name": "open",
        "x": 0,
        "y": 1,
        "count": 1
      }
    ]
  },
  {
    "ani_type": "gate",
    "texture_path": "gate.png",
    "width": 42,
    "height": 18,
    "animations": [
      {
        "action_name": "close",
        "x": 0,
        "y": 0,
        "count": 1
      },
      {
        "action_name": "open",
        "x": 0,
        "y": 1,
        "count": 1
      }
    ]
  }
]
"#;
    let decor_animations: Vec<Ani> = from_str(&decor_json).expect("Unable to parse JSON");
    let mut chest_entities = vec![];

    for (y, row) in map.0.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let transform = get_position_from_map(x, y, None);

            let transform = transform.with_translation(Vec3::new(
                transform.translation.x,
                transform.translation.y,
                transform.translation.z,
            ));
            match cell.as_str() {
                "🌳" => {
                    commands.spawn(DecorBundle {
                        sprite_bundle: SpriteBundle {
                            texture: asset_server.load("tree.png"),
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
                        .find(|ani| ani.ani_type == AniType::Gate)
                        .expect("Expected gate");
                    let deco_bundle = build_ani_decor_bundle(
                        "gate_close".to_owned(),
                        asset_server,
                        atlas_layouts,
                        library,
                        ani,
                        transform
                            .with_scale(Vec3::splat(2.0))
                            .with_translation(Vec3::new(
                                transform.translation.x,
                                transform.translation.y - 16.,
                                transform.translation.z,
                            )),
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
                        .find(|ani| ani.ani_type == AniType::Chest)
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
                    let entity = commands
                        .spawn((
                            deco_bundle,
                            ChestId(chest_id.clone()),
                            Position {
                                xy: Vec2::new(transform.translation.x, transform.translation.y),
                            },
                        ))
                        .id();

                    let chest = Chest {
                        status: ChestState::Close,
                        key: None,
                    };

                    // commands.entity(entity).insert(chest.clone());
                    chest_entities.push((entity, chest.clone()));
                    chests.0.insert(chest_id, chest);
                }
                "💀" => {
                    commands.spawn((
                        DecorBundle {
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
                        },
                        Grave,
                        Position {
                            xy: Vec2::new(transform.translation.x, transform.translation.y),
                        },
                    ));
                }
                _ => (),
            }
        }
    }

    // Add entrance, exit
    let position = get_position_from_map(entrance.x, entrance.y, None);
    commands.spawn((
        Entrance,
        Position {
            xy: Vec2::new(position.translation.x, position.translation.y),
        },
    ));
    let position = get_position_from_map(exit.x, exit.y, None);
    commands.spawn((
        Exit,
        Position {
            xy: Vec2::new(position.translation.x, position.translation.y),
        },
    ));

    chest_entities
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
