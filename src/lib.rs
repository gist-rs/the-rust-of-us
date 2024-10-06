mod afterlife;
mod animations;
mod brains;
mod characters;
mod core;
mod dialogs;
mod interactions;
mod macros;
mod maps;

use afterlife::over::game_over_system;
use bevy::{
    asset::AssetMetaCheck,
    log::LogPlugin,
    prelude::*,
    window::{PresentMode, WindowResolution},
};
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
use extol_sprite_layer::SpriteLayerPlugin;
use interactions::{
    damage::{
        despawn_damage_indicator, spawn_damage_indicator, update_damage, DamageEvent, Damages,
    },
    toggle::{update_toggle_chest, ToggleEvent},
};

#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
struct Configuration {
    behavior: Behavior,
}

pub fn entry() {
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
        .init_resource::<ChunkMap>()
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
                // Chest
                update_chest,
                update_toggle_chest,
                update_gate,
                // Character
                update_character::<Human>,
                update_character::<Monster>,
                // Loot
                loot_system::<Human, Chest>,
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
                update_ask_dialog,
            )
                .run_if(in_state(GameState::Running)),
        )
        .add_systems(
            Update,
            (update_ask_dialog,).run_if(in_state(GameState::Clear)),
        )
        .add_systems(
            Update,
            (game_over_system,).run_if(in_state(GameState::Running)),
        )
        .add_systems(
            PreUpdate,
            (
                guard_action_system::<Chest>,
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
                // --- Human Loot ---
                loot_scorer_system::<Human>,
                move_to_nearest_system::<Chest>,
                loot_action_system::<Human, Chest>,
            )
                .in_set(BigBrainSet::Actions)
                .run_if(in_state(GameState::Running)),
        )
        .add_systems(First, guarding_scorer_system)
        .add_event::<DamageEvent>()
        .add_event::<ToggleEvent>()
        .add_event::<AskDialogEvent>()
        .run();
}

// ---------------------------------------------

use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
        material: materials.add(Color::srgb(1., 0., 0.)),
        ..default()
    });
}

pub fn entry2() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "🦀 The Rust of Us".into(),
                resolution: WindowResolution::new(320.0, 320.0),
                present_mode: PresentMode::AutoNoVsync,
                canvas: Some("#game".into()),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}
