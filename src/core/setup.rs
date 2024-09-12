use crate::core::layer::SpriteLayer;
use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use serde_json::from_str;
use std::fs;

use super::{
    layer::YSort,
    library::{build_library, Ani},
    map::load_map_from_csv,
    scene::{build_decor_bundle, build_scene},
};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Bundle)]
struct PlayerBundle {
    sprite_bundle: SpriteBundle,
    texture_atlas: TextureAtlas,
    spritesheet_animation: SpritesheetAnimation,
    sprite_layer: SpriteLayer,
    marker: Player,
    ysort: YSort,
}

#[derive(Bundle)]
struct EnemyBundle {
    sprite_bundle: SpriteBundle,
    texture_atlas: TextureAtlas,
    spritesheet_animation: SpritesheetAnimation,
    sprite_layer: SpriteLayer,
    marker: Enemy,
    ysort: YSort,
}

fn build_player(
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

fn build_enemy(
    asset_server: &Res<AssetServer>,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    library: &mut ResMut<AnimationLibrary>,
    ani: Ani,
) -> EnemyBundle {
    let clip_fps = 30;

    let libs = build_library(atlas_layouts, library, &ani, clip_fps);

    let texture_path = ani.texture_path.clone();
    let texture = asset_server.load(texture_path);

    EnemyBundle {
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
            library.animation_with_name("skeleton_idle").unwrap(),
        ),
        sprite_layer: SpriteLayer::Ground,
        marker: Enemy,
        ysort: YSort(0.0),
    }
}

use crate::characters::bar::*;
use bevy_stat_bars::*;

pub fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut library: ResMut<AnimationLibrary>,
) {
    commands.spawn(Camera2dBundle::default());

    // Background
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

    // Load map
    let (mut _grid, map) = load_map_from_csv("assets/map.csv").unwrap();
    build_scene(
        &mut commands,
        &asset_server,
        &mut atlas_layouts,
        &mut library,
        map,
    );

    // Load characters from JSON file
    let char_json = fs::read_to_string("assets/char.json").expect("Unable to read file");
    let characters: Vec<Ani> = from_str(&char_json).expect("Unable to parse JSON");

    for ani in characters {
        match ani.r#type.as_str() {
            "player" => {
                let player_bundle =
                    build_player(&asset_server, &mut atlas_layouts, &mut library, ani);

                let player_id = commands
                    .spawn(player_bundle)
                    .insert((
                        PlayerCharacter,
                        Health::new_full(100.0),
                        Statbar::<Health> {
                            color: Color::from(bevy::color::palettes::css::RED),
                            empty_color: Color::from(bevy::color::palettes::css::BLACK),
                            length: 32.0,
                            thickness: 6.0,
                            displacement: 40. * Vec2::Y,
                            ..Default::default()
                        },
                    ))
                    .id();

                commands
                    .spawn((
                        Statbar::<Health> {
                            color: Color::WHITE,
                            empty_color: Color::BLACK,
                            length: 500.0,
                            thickness: 50.0,
                            ..Default::default()
                        },
                        StatbarObserveEntity(player_id),
                    ))
                    .insert(SpatialBundle {
                        transform: Transform::from_translation(-200. * Vec3::Y),
                        ..Default::default()
                    });
            }
            "enemy" => {
                let enemy_bundle =
                    build_enemy(&asset_server, &mut atlas_layouts, &mut library, ani);
                let enemy_id = commands
                    .spawn(enemy_bundle)
                    .insert((
                        EnemyCharacter,
                        Health::new_full(100.0),
                        Statbar::<Health> {
                            color: Color::from(bevy::color::palettes::css::YELLOW),
                            empty_color: Color::from(bevy::color::palettes::css::BLACK),
                            length: 32.0,
                            thickness: 6.0,
                            displacement: 40. * Vec2::Y,
                            ..Default::default()
                        },
                    ))
                    .id();

                commands
                    .spawn((
                        Statbar::<Health> {
                            color: Color::WHITE,
                            empty_color: Color::BLACK,
                            length: 500.0,
                            thickness: 50.0,
                            ..Default::default()
                        },
                        StatbarObserveEntity(enemy_id),
                    ))
                    .insert(SpatialBundle {
                        transform: Transform::from_translation(-200. * Vec3::Y),
                        ..Default::default()
                    });
            }
            _ => (),
        }
    }
}
