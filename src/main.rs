mod brains;
mod characters;
mod core;
mod timeline;

use bevy::{
    log::LogPlugin,
    prelude::*,
    window::{PresentMode, WindowResolution},
};
use bevy_spritesheet_animation::prelude::*;
use bevy_stat_bars::RegisterStatbarSubject;
use big_brain::{BigBrainPlugin, BigBrainSet};
use brains::skeleton::*;
use characters::{
    bar::{adjust_stats, Health, PlayerCharacter},
    r#move::{move_character, CharacterPath},
};
use core::{
    chest::{update_chest, Chests},
    gate::{update_gate, Gates},
    layer::{y_sort, SpriteLayer},
    menu::button_system,
    play::schedule_timeline_actions,
    scene::MainPath,
    setup::{setup_scene, Walkable},
};
use extol_sprite_layer::SpriteLayerPlugin;
use timeline::{
    entity::{TimelineActions, TimelineClock},
    init::{init_timeline, CharacterTimelines},
};

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
                .set(LogPlugin {
                    // Use `RUST_LOG=big_brain=trace,sequence=trace cargo run --example sequence --features=trace` to see extra tracing output.
                    filter: "big_brain=debug,sequence=debug".to_string(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        ))
        .add_plugins(BigBrainPlugin::new(PreUpdate))
        .register_type::<Health>()
        .register_type::<PlayerCharacter>()
        .add_statbar_component_observer::<Health>()
        .init_resource::<TimelineClock>()
        .init_resource::<Chests>()
        .init_resource::<Gates>()
        .init_resource::<TimelineActions>()
        .init_resource::<CharacterTimelines>()
        .init_resource::<MainPath>()
        .init_resource::<CharacterPath>()
        .init_resource::<Walkable>()
        .add_systems(Startup, (setup_scene, init_entities, init_timeline))
        .add_systems(
            Update,
            (
                y_sort,
                adjust_stats,
                button_system,
                thirst_system,
                schedule_timeline_actions,
                move_character,
                update_chest,
                update_gate,
            ),
        )
        .add_systems(
            PreUpdate,
            (drink_action_system, move_to_water_source_action_system).in_set(BigBrainSet::Actions),
        )
        .add_systems(First, thirsty_scorer_system)
        .run();
}
