mod control;
mod core;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution},
};
use bevy_spritesheet_animation::prelude::*;
use control::character;
use core::layer::SpriteLayer;
use extol_sprite_layer::SpriteLayerPlugin;

fn main() {
    App::new()
        .add_plugins((
            SpritesheetAnimationPlugin,
            SpriteLayerPlugin::<SpriteLayer>::default(),
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "🦀 The Rust of Us".into(),
                        resolution: WindowResolution::new(320.0, 320.0),
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, character::control_character)
        .run();
}

#[derive(Debug)]
struct AnimationDetails {
    action_name: String,
    x: usize,
    y: usize,
    count: usize,
}

fn build_character(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    library: &mut ResMut<AnimationLibrary>,
    texture_path: String,
    sprite_width: u32,
    sprite_height: u32,
    animations: Vec<AnimationDetails>,
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
            SpritesheetAnimation::from_id(library.animation_with_name(anim.action_name).unwrap()),
            SpriteLayer::Player,
        ));
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut library: ResMut<AnimationLibrary>,
) {
    commands.spawn(Camera2dBundle::default());

    let animations = vec![
        AnimationDetails {
            action_name: "idle".to_string(),
            x: 0,
            y: 0,
            count: 9,
        },
        AnimationDetails {
            action_name: "walk".to_string(),
            x: 0,
            y: 1,
            count: 8,
        },
        AnimationDetails {
            action_name: "attack".to_string(),
            x: 0,
            y: 2,
            count: 10,
        },
    ];

    build_character(
        &mut commands,
        &asset_server,
        &mut atlas_layouts,
        &mut library,
        "man.png".to_owned(),
        96,
        96,
        animations,
    );

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
