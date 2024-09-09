mod control;
mod core;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution},
};
use bevy_spritesheet_animation::prelude::*;
use control::character;
use core::{layer::SpriteLayer, setup::setup_scene};
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
        .add_systems(Startup, setup_scene)
        .add_systems(Update, character::control_character)
        .run();
}
