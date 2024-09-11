use crate::core::layer::SpriteLayer;
use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use serde::Deserialize;
use serde_json::from_str;
use std::fs;

use super::{layer::YSort, map::load_map_from_csv, scene::build_scene};

#[derive(Deserialize)]
struct AnimationDetails {
    action_name: String,
    x: usize,
    y: usize,
    count: usize,
}

#[derive(Deserialize)]
struct Character {
    name: String,
    r#type: String,
    texture_path: String,
    width: u32,
    height: u32,
    animations: Vec<AnimationDetails>,
}

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

fn build_library(
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    library: &mut ResMut<AnimationLibrary>,
    character: &Character,
    clip_fps: u32,
) -> Vec<Handle<TextureAtlasLayout>> {
    // Create the spritesheet
    let spritesheet = Spritesheet::new(10, character.animations.len());

    // Register animations
    let animations = &character.animations;
    let sprite_width = character.width;
    let sprite_height = character.height;

    animations
        .iter()
        .map(|anim| {
            let clip = Clip::from_frames(spritesheet.horizontal_strip(anim.x, anim.y, anim.count))
                .with_duration(AnimationDuration::PerFrame(clip_fps));
            let clip_id = library.register_clip(clip);
            let animation = Animation::from_clip(clip_id);
            let animation_id = library.register_animation(animation);
            let animation_name = format!("{}_{}", character.name, &anim.action_name);
            println!("{animation_name}");
            library
                .name_animation(animation_id, animation_name.clone())
                .unwrap();

            atlas_layouts.add(spritesheet.atlas_layout(sprite_width, sprite_height))
        })
        .collect::<Vec<_>>()
}

fn build_player(
    asset_server: &Res<AssetServer>,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    library: &mut ResMut<AnimationLibrary>,
    character: Character,
) -> PlayerBundle {
    let clip_fps = 30;

    let libs = build_library(atlas_layouts, library, &character, clip_fps);

    let texture_path = character.texture_path.clone();
    let texture = asset_server.load(texture_path);

    PlayerBundle {
        sprite_bundle: SpriteBundle {
            texture,
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(2.0)),
            ..default()
        },
        texture_atlas: TextureAtlas {
            layout: libs[0].clone(),
            ..default()
        },
        spritesheet_animation: SpritesheetAnimation::from_id(
            library.animation_with_name("man_idle").unwrap(),
        ),
        sprite_layer: SpriteLayer::Character,
        marker: Player,
        ysort: YSort(0.0),
    }
}

fn build_enemy(
    asset_server: &Res<AssetServer>,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    library: &mut ResMut<AnimationLibrary>,
    character: Character,
) -> EnemyBundle {
    let clip_fps = 30;

    let libs = build_library(atlas_layouts, library, &character, clip_fps);

    let texture_path = character.texture_path.clone();
    let texture = asset_server.load(texture_path);

    EnemyBundle {
        sprite_bundle: SpriteBundle {
            texture,
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(2.0)),
            ..default()
        },
        texture_atlas: TextureAtlas {
            layout: libs[0].clone(),
            ..default()
        },
        spritesheet_animation: SpritesheetAnimation::from_id(
            library.animation_with_name("skeleton_idle").unwrap(),
        ),
        sprite_layer: SpriteLayer::Character,
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
    build_scene(&mut commands, &asset_server, map);

    // Load characters from JSON file
    let characters_json =
        fs::read_to_string("assets/characters.json").expect("Unable to read file");
    let characters: Vec<Character> = from_str(&characters_json).expect("Unable to parse JSON");

    for character in characters {
        match character.r#type.as_str() {
            "player" => {
                let player_bundle =
                    build_player(&asset_server, &mut atlas_layouts, &mut library, character);

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
                    build_enemy(&asset_server, &mut atlas_layouts, &mut library, character);
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
