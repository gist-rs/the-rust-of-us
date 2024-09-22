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
use brains::{
    enemy::{init_enemy, update_enemy},
    skeleton::*,
};
use characters::{
    bar::{adjust_stats, Health},
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
    stage::{init_stage, GameStage},
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
                    // Use `RUST_LOG=big_brain=trace,main=trace cargo run --example main --features=trace` to see extra tracing output.
                    filter: "big_brain=debug,the_rust_of_us=debug".to_string(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        ))
        .add_plugins(BigBrainPlugin::new(PreUpdate))
        .register_type::<Health>()
        .add_statbar_component_observer::<Health>()
        .init_resource::<TimelineClock>()
        .init_resource::<Chests>()
        .init_resource::<Gates>()
        .init_resource::<TimelineActions>()
        .init_resource::<CharacterTimelines>()
        .init_resource::<MainPath>()
        .init_resource::<CharacterPath>()
        .init_resource::<Walkable>()
        .init_resource::<GameStage>()
        .add_systems(
            Startup,
            ((
                setup_scene,
                init_stage,
                init_enemy,
                // init_entities,
                init_timeline,
            )
                .chain(),),
        )
        .add_systems(
            Update,
            (
                y_sort,
                adjust_stats,
                button_system,
                guard_system,
                schedule_timeline_actions,
                move_character,
                update_chest,
                update_gate,
                update_enemy,
            ),
        )
        .add_systems(
            PreUpdate,
            (guard_action_system, move_to_chest_action_system).in_set(BigBrainSet::Actions),
        )
        .add_systems(First, guarding_scorer_system)
        .run();
}
