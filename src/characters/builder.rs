use std::fs;

use crate::{
    brains::behavior::get_behavior,
    characters::{actions::Action, bar::Health},
    core::{
        layer::{SpriteLayer, YSort},
        library::{build_library, Ani},
        map::{convert_map_to_screen, get_position_from_map},
        position::Position,
        setup::CharacterId,
        stage::{CharacterInfo, GameStage, StageInfo},
    },
    get_thinker,
};
use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use bevy_stat_bars::{Statbar, StatbarObserveEntity};
use std::fmt::Debug;

use super::actions::{Act, LookDirection};

#[derive(Bundle)]
struct CharacterBundle<T: Component> {
    sprite_bundle: SpriteBundle,
    texture_atlas: TextureAtlas,
    spritesheet_animation: SpritesheetAnimation,
    sprite_layer: SpriteLayer,
    marker: T,
    ysort: YSort,
}

#[derive(Component)]
pub struct CharacterMarker;

fn get_animation_name(character_id: CharacterId, act: Act) -> String {
    let subject = character_id.0.split('_').next().expect("subject");

    format!("{subject}_{act}")
}

fn build_character<T: CharacterInfo>(
    asset_server: &Res<AssetServer>,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    library: &mut ResMut<AnimationLibrary>,
    ani: Ani,
    character_info: T,
) -> CharacterBundle<T> {
    let clip_fps = 30;

    let libs = build_library(atlas_layouts, library, &ani, clip_fps);

    let texture_path = ani.texture_path.clone();
    let texture = asset_server.load(texture_path);

    let at = convert_map_to_screen(character_info.position().clone()).expect("Valid position");
    let position = get_position_from_map(at.0, at.1, None);

    let is_flip_x = match character_info.look_direction() {
        LookDirection::Left => true,
        LookDirection::Right => false,
    };

    let animation_name = get_animation_name(character_info.character_id().to_owned(), Act::Idle);

    CharacterBundle {
        sprite_bundle: SpriteBundle {
            texture,
            sprite: Sprite {
                flip_x: is_flip_x,
                ..default()
            },
            transform: Transform::from_xyz(position.translation.x, position.translation.y, 0.0)
                .with_scale(Vec3::splat(2.0)),
            ..default()
        },
        texture_atlas: TextureAtlas {
            layout: libs[0].1.clone(),
            ..default()
        },
        spritesheet_animation: SpritesheetAnimation::from_id(
            library.animation_with_name(animation_name).unwrap(),
        ),
        sprite_layer: SpriteLayer::Ground,
        marker: character_info.get_clone(),
        ysort: YSort(0.0),
    }
}

pub fn init_character<T>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut library: ResMut<AnimationLibrary>,
    game_stage: Res<GameStage>,
) where
    T: CharacterInfo + Clone + Debug + 'static,
{
    let char_json = fs::read_to_string("assets/char.json").expect("Unable to read file");
    let characters: Vec<Ani> = serde_json::from_str(&char_json).expect("Unable to parse JSON");
    println!("characters:{:?}", characters);

    if let Some(character_iter) = game_stage.0.get_characters_iter_by_type::<T>() {
        for character in character_iter {
            println!("🔥 character:{:?}", *character);
            if let Some(ani) = characters.iter().find(|&c| c.r#type == *character.r#type()) {
                let at =
                    convert_map_to_screen(character.position().clone()).expect("Valid position");
                let position = get_position_from_map(at.0, at.1, None);

                let character_bundle = build_character::<T>(
                    &asset_server,
                    &mut atlas_layouts,
                    &mut library,
                    ani.clone(),
                    character.clone(),
                );

                let mut entity_commands = commands.spawn(character_bundle);

                // Statics
                entity_commands
                    .insert(CharacterId(character.character_id().0.clone()))
                    .insert((
                        Action(*character.act()),
                        Position {
                            position: Vec2::new(position.translation.x, position.translation.y),
                        },
                        Health::new_full(100.0),
                        Statbar::<Health> {
                            color: Color::from(bevy::color::palettes::css::RED),
                            empty_color: Color::from(bevy::color::palettes::css::BLACK),
                            length: 32.0,
                            thickness: 6.0,
                            displacement: 32. * Vec2::Y,
                            ..Default::default()
                        },
                        CharacterMarker,
                    ));

                // Dynamics
                entity_commands.insert((get_thinker::<T>(), get_behavior::<T>()));

                let character_id = entity_commands.id();

                commands
                    .spawn((
                        Statbar::<Health> {
                            color: Color::WHITE,
                            empty_color: Color::BLACK,
                            length: 500.0,
                            thickness: 50.0,
                            ..Default::default()
                        },
                        StatbarObserveEntity(character_id),
                    ))
                    .insert(SpatialBundle {
                        transform: Transform::from_translation(-200. * Vec3::Y),
                        ..Default::default()
                    });
            }
        }
    }
}

#[allow(clippy::complexity)]
pub fn update_character<T>(
    game_stage: Res<GameStage>,
    mut characters: Query<
        (
            &CharacterId,
            &mut Position,
            &mut Transform,
            &mut Sprite,
            &mut SpritesheetAnimation,
            &mut Action,
        ),
        With<CharacterMarker>,
    >,
    library: Res<AnimationLibrary>,
) where
    T: CharacterInfo + 'static,
{
    if let Some(character_iter) = game_stage.0.get_characters_iter_by_type::<T>() {
        for character in character_iter {
            for (
                character_id,
                character_position,
                mut character_transform,
                mut sprite,
                mut animation,
                action,
            ) in characters.iter_mut()
            {
                if character.character_id() == character_id {
                    // Look direction
                    sprite.flip_x =
                        character_transform.translation.x > character_position.position.x;

                    // Position
                    character_transform.translation.x = character_position.position.x;
                    character_transform.translation.y = character_position.position.y;

                    // Action
                    let subject = character_id.0.split('_').next().expect("subject");
                    let act = action.0;
                    let animation_name = format!("{subject}_{act}");

                    if let Some(animation_id) = library.animation_with_name(animation_name) {
                        if animation.animation_id != animation_id {
                            animation.switch(animation_id);
                        }
                    }
                }
            }
        }
    }
}