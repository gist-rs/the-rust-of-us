mod brains;
mod characters;
mod core;
mod macros;

use bevy::{
    log::LogPlugin,
    prelude::*,
    window::{PresentMode, WindowResolution},
};
use bevy_inspector_egui::prelude::*;
// use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_spritesheet_animation::prelude::*;
use bevy_stat_bars::RegisterStatbarSubject;
use big_brain::{BigBrainPlugin, BigBrainSet};
use brains::{
    behavior::Behavior,
    fight::{fight_action_system, fight_scorer_system, fight_system, game_over_system},
    thinker::*,
};
use characters::{bar::Health, builder::init_character, update::update_character};
use core::{
    chest::{update_chest, Chest, Chests},
    damage::{
        despawn_damage_indicator, spawn_damage_indicator, update_damage, DamageEvent, Damages,
    },
    gate::{update_gate, Gates},
    grave::Grave,
    layer::{y_sort, SpriteLayer},
    menu::button_system,
    point::Exit,
    scene::MainPath,
    setup::setup_scene,
    stage::{init_stage, GameStage, Human, Monster},
    state::GameState,
};
use extol_sprite_layer::SpriteLayerPlugin;

// `InspectorOptions` are completely optional
#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct Configuration {
    behavior: Behavior,
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
                .set(LogPlugin {
                    // Use `RUST_LOG=big_brain=trace,main=trace cargo run --example main --features=trace` to see extra tracing output.
                    filter: "big_brain=debug,the_rust_of_us=debug".to_string(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        ))
        .add_plugins(BigBrainPlugin::new(PreUpdate))
        // .add_plugins(ResourceInspectorPlugin::<Configuration>::default())
        .register_type::<Health>()
        .register_type::<Behavior>()
        .add_statbar_component_observer::<Health>()
        .register_type::<Configuration>()
        .init_resource::<Configuration>()
        .init_resource::<Chests>()
        .init_resource::<Gates>()
        .init_resource::<MainPath>()
        .init_resource::<GameStage>()
        .init_resource::<Damages>()
        .init_state::<GameState>()
        .add_systems(
            Startup,
            ((
                setup_scene,
                init_stage,
                init_character::<Human>,
                init_character::<Monster>,
            )
                .chain(),),
        )
        .add_systems(
            Update,
            (
                y_sort,
                // adjust_stats,
                button_system,
                guard_system,
                update_chest,
                update_gate,
                update_character::<Human>,
                update_character::<Monster>,
                // Fight
                fight_system::<Monster, Human>,
                fight_system::<Human, Monster>,
                // Damage
                spawn_damage_indicator,
                update_damage,
                // // Die
                // despawn_fighter_on_death_system::<Human>,
                // despawn_fighter_on_death_system::<Monster>,
                despawn_damage_indicator,
                // death_system,
            )
                .run_if(in_state(GameState::Running)),
        )
        .add_systems(
            Update,
            (game_over_system,).run_if(in_state(GameState::Over)),
        )
        .add_systems(
            PreUpdate,
            (
                guard_action_system::<Chest>,
                move_to_nearest_system::<Chest>,
                move_to_nearest_system::<Grave>,
                move_to_nearest_system::<Exit>,
                // --- Monster Fight ---
                // Monster seek for Human
                fight_scorer_system::<Monster>,
                // Monster follow Human
                move_to_nearest_system::<Human>,
                // Monster fight with Human
                fight_action_system::<Monster, Human>,
                // --- Human Fight ---
                // Human seek for Monster
                fight_scorer_system::<Human>,
                // Human follow Monster
                move_to_nearest_system::<Monster>,
                // Human fight with Monster
                fight_action_system::<Human, Monster>,
            )
                .in_set(BigBrainSet::Actions)
                .run_if(in_state(GameState::Running)),
        )
        .add_systems(First, guarding_scorer_system)
        .add_event::<DamageEvent>()
        .run();
}
