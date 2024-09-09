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
    file_uri: String,
    width: u32,
    height: u32,
    animations: Vec<AnimationDetails>,
}

fn build_character(
    asset_server: &Res<AssetServer>,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    library: &mut ResMut<AnimationLibrary>,
    texture_path: String,
    sprite_width: u32,
    sprite_height: u32,
    animations: Vec<AnimationDetails>,
) -> (
    SpriteBundle,
    TextureAtlas,
    SpritesheetAnimation,
    SpriteLayer,
) {
    let clip_fps = 30;

    // Create the spritesheet
    let spritesheet = Spritesheet::new(10, 3);

    // Register animations
    for anim in animations {
        let clip = Clip::from_frames(spritesheet.horizontal_strip(anim.x, anim.y, anim.count))
            .with_duration(AnimationDuration::PerFrame(clip_fps));
        let clip_id = library.register_clip(clip);
        let animation = Animation::from_clip(clip_id);
        let animation_id = library.register_animation(animation);
        library
            .name_animation(animation_id, &anim.action_name)
            .unwrap();
    }

    // Spawn the character with the initial idle animation
    let texture = asset_server.load(texture_path);
    let layout = atlas_layouts.add(spritesheet.atlas_layout(sprite_width, sprite_height));
    let idle_animation_id = library.animation_with_name("idle").unwrap();

    (
        SpriteBundle {
            texture,
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
        TextureAtlas {
            layout,
            ..default()
        },
        SpritesheetAnimation::from_id(idle_animation_id),
        SpriteLayer::Player,
    )
}

pub fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut library: ResMut<AnimationLibrary>,
) {
    commands.spawn(Camera2dBundle::default());

    // Load characters from JSON file
    let characters_json =
        fs::read_to_string("assets/characters.json").expect("Unable to read file");
    let characters: Vec<Character> = from_str(&characters_json).expect("Unable to parse JSON");

    for character in characters {
        let (sprite_bundle, texture_atlas, spritesheet_animation, sprite_layer) = build_character(
            &asset_server,
            &mut atlas_layouts,
            &mut library,
            character.file_uri,
            character.width,
            character.height,
            character.animations,
        );

        commands.spawn((
            sprite_bundle,
            texture_atlas,
            spritesheet_animation,
            sprite_layer,
        ));
    }

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
}
