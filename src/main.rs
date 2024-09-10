mod control;
mod core;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution},
};
use bevy_spritesheet_animation::prelude::*;
use bevy_stat_bars::RegisterStatbarSubject;
use control::{
    bar::{adjust_stats, Health, Magic, PlayerCharacter},
    character,
};
use core::{
    layer::{y_sort, SpriteLayer},
    setup::setup_scene,
};
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
        .register_type::<Health>()
        .register_type::<Magic>()
        .register_type::<PlayerCharacter>()
        .add_statbar_component_observer::<Health>()
        .add_statbar_component_observer::<Magic>()
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (character::control_character, y_sort, adjust_stats))
        .run();
}
