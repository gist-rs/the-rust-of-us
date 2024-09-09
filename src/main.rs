// This example shows how to create controllable character with multiple animations.
//
// - We'll create a few animations for our character (idle, walk, shoot) in a setup system
// - We'll move the character with the keyboard and switch to the appropriate animation in another system

#[path = "./common/mod.rs"]
pub mod common;

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use extol_sprite_layer::{LayerIndex, SpriteLayerOptions, SpriteLayerPlugin};

#[derive(Debug, Copy, Clone, Component, PartialEq, Eq, Hash)]
enum SpriteLayer {
    Background,
    Object,
    Enemy,
    Player,
    Ui,
}

impl LayerIndex for SpriteLayer {
    // Convert your type to an actual z-coordinate.
    fn as_z_coordinate(&self) -> f32 {
        use SpriteLayer::*;
        match *self {
            // Note that the z-coordinates must be at least 1 apart...
            Background => 0.,
            Object => 1.,
            Enemy => 2.,
            // ... but can be more than that.
            Player => 990.,
            Ui => 995.,
        }
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            SpritesheetAnimationPlugin,
            SpriteLayerPlugin::<SpriteLayer>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, control_character)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut library: ResMut<AnimationLibrary>,
    assets: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    // Create the animations

    let spritesheet = Spritesheet::new(10, 3);

    // Idle

    let idle_clip = Clip::from_frames(spritesheet.horizontal_strip(0, 0, 9));
    let idle_clip_id = library.register_clip(idle_clip);
    let idle_animation = Animation::from_clip(idle_clip_id);
    let idle_animation_id = library.register_animation(idle_animation);

    library.name_animation(idle_animation_id, "idle").unwrap();

    // Walk
    let walk_clip = Clip::from_frames(spritesheet.horizontal_strip(0, 1, 8));
    let walk_clip_id = library.register_clip(walk_clip);
    let walk_animation = Animation::from_clip(walk_clip_id);
    let walk_animation_id = library.register_animation(walk_animation);

    library.name_animation(walk_animation_id, "walk").unwrap();

    // Shoot
    let shoot_clip = Clip::from_frames(spritesheet.horizontal_strip(0, 2, 10));
    let shoot_clip_id = library.register_clip(shoot_clip);
    let shoot_animation = Animation::from_clip(shoot_clip_id);
    let shoot_animation_id = library.register_animation(shoot_animation);

    library.name_animation(shoot_animation_id, "shoot").unwrap();

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

// Component to check if a character is currently shooting
#[derive(Component)]
struct Shooting;

fn control_character(
    mut commands: Commands,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    library: Res<AnimationLibrary>,
    mut events: EventReader<AnimationEvent>,
    mut characters: Query<(
        Entity,
        &mut Transform,
        &mut Sprite,
        &mut SpritesheetAnimation,
        Option<&Shooting>,
    )>,
) {
    // Control the character with the keyboard

    const CHARACTER_SPEED: f32 = 150.0;

    for (entity, mut transform, mut sprite, mut animation, shooting) in &mut characters {
        // Except if they're shooting, in which case we wait for the animation to end

        if shooting.is_some() {
            continue;
        }

        // Shoot
        if keyboard.pressed(KeyCode::Space) {
            // Set the animation

            if let Some(shoot_animation_id) = library.animation_with_name("shoot") {
                animation.switch(shoot_animation_id);
            }

            // Add a Shooting component

            commands.entity(entity).insert(Shooting);
        }
        // Move left
        else if keyboard.pressed(KeyCode::ArrowLeft) {
            // Set the animation

            if let Some(walk_animation_id) = library.animation_with_name("walk") {
                if animation.animation_id != walk_animation_id {
                    animation.switch(walk_animation_id);
                }
            }

            // Move

            transform.translation -= Vec3::X * time.delta_seconds() * CHARACTER_SPEED;
            sprite.flip_x = true;
        }
        // Move right
        else if keyboard.pressed(KeyCode::ArrowRight) {
            // Set the animation

            if let Some(walk_animation_id) = library.animation_with_name("walk") {
                if animation.animation_id != walk_animation_id {
                    animation.switch(walk_animation_id);
                }
            }

            // Move

            transform.translation += Vec3::X * time.delta_seconds() * CHARACTER_SPEED;
            sprite.flip_x = false;
        }
        // Idle
        else {
            // Set the animation

            if let Some(idle_animation_id) = library.animation_with_name("idle") {
                if animation.animation_id != idle_animation_id {
                    animation.switch(idle_animation_id);
                }
            }
        }
    }

    // Remove the Shooting component when the shooting animation ends
    //
    // We use animation events to detect when this happens.
    // Check out the `events` examples for more details.

    for event in events.read() {
        match event {
            AnimationEvent::AnimationRepetitionEnd {
                entity,
                animation_id,
                ..
            } => {
                if library.is_animation_name(*animation_id, "shoot") {
                    commands.entity(*entity).remove::<Shooting>();
                }
            }
            _ => (),
        }
    }
}
