use crate::core::layer::SpriteLayer;
use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use serde::Deserialize;
use serde_json::from_str;
use std::fs;

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
}

#[derive(Bundle)]
struct EnemyBundle {
    sprite_bundle: SpriteBundle,
    texture_atlas: TextureAtlas,
    spritesheet_animation: SpritesheetAnimation,
    sprite_layer: SpriteLayer,
    marker: Enemy,
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
            transform: Transform::from_scale(Vec3::splat(2.0)),
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
            transform: Transform::from_scale(Vec3::splat(2.0)),
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
    }
}

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

    // Load characters from JSON file
    let characters_json =
        fs::read_to_string("assets/characters.json").expect("Unable to read file");
    let characters: Vec<Character> = from_str(&characters_json).expect("Unable to parse JSON");

    for character in characters {
        match character.r#type.as_str() {
            "player" => {
                let player_bundle =
                    build_player(&asset_server, &mut atlas_layouts, &mut library, character);
                commands.spawn(player_bundle);
            }
            "enemy" => {
                let enemy_bundle =
                    build_enemy(&asset_server, &mut atlas_layouts, &mut library, character);
                commands.spawn(enemy_bundle);
            }
            _ => (),
        }
    }
}
