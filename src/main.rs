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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut library: ResMut<AnimationLibrary>,
    assets: Res<AssetServer>,
) {
    let clip_fps = 30;
    commands.spawn(Camera2dBundle::default());

    // Create the animations
    let spritesheet = Spritesheet::new(10, 3);

    // Idle
    let idle_clip = Clip::from_frames(spritesheet.horizontal_strip(0, 0, 9))
        .with_duration(AnimationDuration::PerFrame(clip_fps));
    let idle_clip_id = library.register_clip(idle_clip);
    let idle_animation = Animation::from_clip(idle_clip_id);
    let idle_animation_id = library.register_animation(idle_animation);

    library.name_animation(idle_animation_id, "idle").unwrap();

    // Walk
    let walk_clip = Clip::from_frames(spritesheet.horizontal_strip(0, 1, 8))
        .with_duration(AnimationDuration::PerFrame(clip_fps));
    let walk_clip_id = library.register_clip(walk_clip);
    let walk_animation = Animation::from_clip(walk_clip_id);
    let walk_animation_id = library.register_animation(walk_animation);

    library.name_animation(walk_animation_id, "walk").unwrap();

    // Attack
    let attack_clip = Clip::from_frames(spritesheet.horizontal_strip(0, 2, 10))
        .with_duration(AnimationDuration::PerFrame(clip_fps));
    let attack_clip_id = library.register_clip(attack_clip);
    let attack_animation = Animation::from_clip(attack_clip_id);
    let attack_animation_id = library.register_animation(attack_animation);

    library
        .name_animation(attack_animation_id, "attack")
        .unwrap();

    // Spawn the character
    let texture = assets.load("man.png");
    let layout = atlas_layouts.add(spritesheet.atlas_layout(96, 96));

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
        SpritesheetAnimation::from_id(idle_animation_id),
        SpriteLayer::Player,
    ));

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
