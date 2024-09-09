use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowResolution},
};
use bevy_spritesheet_animation::prelude::*;

use extol_sprite_layer::{LayerIndex, SpriteLayerPlugin};

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
    let clip_fps = 60;
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

// Component to check if a character is currently attack
#[derive(Component)]
struct Attack;

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
        Option<&Attack>,
    )>,
) {
    // Control the character with the keyboard
    const CHARACTER_SPEED: f32 = 150.0;

    for (entity, mut transform, mut sprite, mut animation, attack) in &mut characters {
        // Except if they're attack, in which case we wait for the animation to end
        if attack.is_some() {
            continue;
        }

        // Attack
        if keyboard.pressed(KeyCode::Space) {
            // Set the animation
            if let Some(attack_animation_id) = library.animation_with_name("attack") {
                animation.switch(attack_animation_id);
            }

            // Add a Attacking component
            commands.entity(entity).insert(Attack);
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

    // Remove the Attacking component when the attack animation ends
    // We use animation events to detect when this happens.
    for event in events.read() {
        match event {
            AnimationEvent::AnimationRepetitionEnd {
                entity,
                animation_id,
                ..
            } => {
                if library.is_animation_name(*animation_id, "attack") {
                    commands.entity(*entity).remove::<Attack>();
                }
            }
            _ => (),
        }
    }
}
