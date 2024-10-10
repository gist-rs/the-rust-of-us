mod afterlife;
mod animations;
mod brains;
mod characters;
mod core;
mod dialogs;
mod entry;
mod interactions;
mod macros;
mod maps;

#[cfg(target_arch = "wasm32")]
mod web;

use afterlife::over::game_over_system;
use bevy::{
    asset::AssetMetaCheck,
    log::LogPlugin,
    prelude::*,
    window::{PresentMode, WindowResolution},
};
use bevy_pkv::PkvStore;
use bevy_spritesheet_animation::prelude::*;
use bevy_stat_bars::RegisterStatbarSubject;
use big_brain::{BigBrainPlugin, BigBrainSet};
use brains::{
    behavior::Behavior,
    fight::{fight_action_system, fight_scorer_system, fight_system},
    loot::{loot_action_system, loot_scorer_system, loot_system},
    thinker::*,
};
use characters::{bar::Health, builder::init_character, update::update_character};
use core::{
    chest::{update_chest, Chest, Chests},
    gate::{update_gate, Gates},
    grave::Grave,
    layer::{y_sort, SpriteLayer},
    menu::button_system,
    point::Exit,
    scene::{ChunkMap, MainPath},
    setup::setup_scene,
    stage::{init_stage, GameStage, Human, Monster},
    state::GameState,
};
use dialogs::ask::{update_ask_dialog, AskDialogEvent};
use entry::{game, menu, splash, DisplayQuality, Volume};
use extol_sprite_layer::SpriteLayerPlugin;
use interactions::{
    damage::{
        despawn_damage_indicator, spawn_damage_indicator, update_damage, DamageEvent, Damages,
    },
    toggle::{update_toggle_chest, ToggleEvent},
};

#[cfg(target_arch = "wasm32")]
use web::local_storage::get_local_storage_value;

#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
struct Configuration {
    behavior: Behavior,
}

pub fn entry() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "🦀 The Rust of Us".into(),
                        resolution: WindowResolution::new(320.0, 320.0),
                        present_mode: PresentMode::AutoNoVsync,
                        canvas: Some("#game".into()),
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    // Use `RUST_LOG=big_brain=trace,main=trace cargo run --example main --features=trace` to see extra tracing output.
                    filter: "big_brain=debug,the_rust_of_us=debug".to_string(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        )
        // Insert as resource the initial value for the settings resources
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        // Declare the game state, whose starting value is determined by the `Default` trait
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        // Adds the plugins for each state
        .add_plugins((splash::splash_plugin, menu::menu_plugin, game::game_plugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[cfg(target_arch = "wasm32")]
pub fn get_public_key() -> Option<String> {
    let public_key = get_local_storage_value("public_key");
    debug!("public_key: {:?}", public_key);
    public_key
}
