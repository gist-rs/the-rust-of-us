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
    texture_path: String,
    width: u32,
    height: u32,
    animations: Vec<AnimationDetails>,
}

#[derive(Component)]
pub struct ManCharacter;

#[derive(Component)]
pub struct SkeletonCharacter;

fn build_character(
    mut commands: &Commands,
    asset_server: &Res<AssetServer>,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    library: &mut ResMut<AnimationLibrary>,
    character: Character,
) {
    let clip_fps = 30;

    // Create the spritesheet
    let spritesheet = Spritesheet::new(10, 3);

    // Register animations
    let animations = character.animations;
    let texture_path = character.texture_path;
    let sprite_width = character.width;
    let sprite_height = character.height;

    animations.iter().for_each(|anim: &AnimationDetails| {
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

        // Spawn the character
        let texture = asset_server.load(texture_path.clone());
        let layout = atlas_layouts.add(spritesheet.atlas_layout(sprite_width, sprite_height));

        commands.spawn((
            SpriteBundle {
                texture,
                transform: Transform::from_scale(Vec3::splat(2.0)),
                ..default()
            },
            TextureAtlas {
                layout,
                ..default()
            },
            SpritesheetAnimation::from_id(library.animation_with_name(animation_name).unwrap()),
            SpriteLayer::Player,
            match character.name.as_str() {
                "man" => ManCharacter,
                "skeleton" => SkeletonCharacter,
                _ => panic!("Unknown character type"),
            },
        ));
    })
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
        build_character(
            &commands,
            &asset_server,
            &mut atlas_layouts,
            &mut library,
            character,
        );
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
