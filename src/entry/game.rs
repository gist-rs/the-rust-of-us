use bevy::{
    color::palettes::basic::{BLUE, LIME},
    prelude::*,
};

use crate::{
    afterlife, brains, characters,
    core::{self, map::MapConfig},
    dialogs, entry, interactions, Configuration,
};

use super::{despawn_screen, TEXT_COLOR};

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

// This plugin will contain the game. In this case, it's just be a screen that will
// display the current settings for 5 seconds before returning to the menu
pub fn game_plugin(app: &mut App) {
    app.add_plugins((
        SpritesheetAnimationPlugin,
        SpriteLayerPlugin::<SpriteLayer>::default(),
    ))
    .add_plugins(BigBrainPlugin::new(PreUpdate))
    // .add_plugins(ResourceInspectorPlugin::<Configuration>::default())
    .register_type::<Health>()
    .register_type::<Behavior>()
    .add_statbar_component_observer::<Health>()
    .insert_resource(PkvStore::new("foo", "bar"))
    .init_resource::<Configuration>()
    .init_resource::<Chests>()
    .init_resource::<Gates>()
    .init_resource::<ChunkMap>()
    .init_resource::<MainPath>()
    .init_resource::<GameStage>()
    .init_resource::<Damages>()
    .init_resource::<MapConfig>()
    .add_systems(
        OnEnter(GameState::Game),
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
            .run_if(in_state(GameState::Game)),
    )
    .add_systems(
        Update,
        (update_ask_dialog,).run_if(in_state(GameState::Clear)),
    )
    .add_systems(
        Update,
        (game_over_system,).run_if(in_state(GameState::Game)),
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
            .run_if(in_state(GameState::Game)),
    )
    .add_systems(First, guarding_scorer_system)
    .add_event::<DamageEvent>()
    .add_event::<ToggleEvent>()
    .add_event::<AskDialogEvent>()
    // .add_systems(Update, game.run_if(in_state(GameState::Game)))
    .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
pub struct OnGameScreen;
