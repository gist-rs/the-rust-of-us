use std::fs;

use crate::{
    brains::{
        behavior::get_behavior,
        fight::{get_fighter, TargetAt},
    },
    characters::{
        actions::{Act, Action, LookDirection},
        bar::Health,
        entities::CharacterId,
    },
    core::{
        damage::{Damage, DamageEvent},
        layer::{SpriteLayer, YSort},
        library::{build_library, Ani},
        map::{convert_map_to_screen, get_position_from_map},
        position::Position,
        stage::{CharacterInfo, GameStage, StageInfo},
    },
    get_thinker,
};
use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use bevy_stat_bars::{Statbar, StatbarObserveEntity};
use std::fmt::Debug;

use super::{
    actions::AniAction,
    entities::{AniType, CharacterKind},
};

#[derive(Bundle)]
struct CharacterBundle<T: Component> {
    sprite_bundle: SpriteBundle,
    texture_atlas: TextureAtlas,
    spritesheet_animation: SpritesheetAnimation,
    sprite_layer: SpriteLayer,
    marker: T,
    ysort: YSort,
    target_at: TargetAt,
    kind: CharacterKind,
    ani_action: AniAction,
}

#[derive(Component)]
pub struct CharacterMarker;

pub fn get_animation_name(ani_type: &AniType, act: Act) -> String {
    format!("{ani_type}_{act}")
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

    let animation_name = get_animation_name(character_info.ani_type(), Act::Idle);

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
        target_at: TargetAt::default(),
        kind: *character_info.kind(),
        ani_action: AniAction {
            act: Act::default(),
        },
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
            if let Some(ani) = characters
                .iter()
                .find(|&c| c.ani_type == *character.ani_type())
            {
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
                    .insert(*character.ani_type())
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
                get_fighter::<T>(&mut entity_commands);
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

// #[allow(clippy::complexity)]
// pub fn update_animation<T>(mut characters: Query<(&mut SpritesheetAnimation, &mut AniAction)>) {
//     // TODO: update AniAction state
// }

#[allow(clippy::complexity)]
pub fn update_character<T>(
    game_stage: Res<GameStage>,
    mut characters: Query<
        (
            &CharacterId,
            &AniType,
            &mut Position,
            &mut Transform,
            &mut Sprite,
            &mut SpritesheetAnimation,
            &mut Action,
            &mut AniAction,
            &mut TargetAt,
            &T,
        ),
        With<CharacterMarker>,
    >,
    library: Res<AnimationLibrary>,
    mut damage_events: EventWriter<DamageEvent>,
) where
    T: CharacterInfo + 'static,
{
    if let Some(character_iter) = game_stage.0.get_characters_iter_by_type::<T>() {
        for character in character_iter {
            for (
                character_id,
                ani_type,
                character_position,
                mut character_transform,
                mut sprite,
                mut animation,
                action,
                mut ani_action,
                actor_target_at,
                character_info,
            ) in characters.iter_mut()
            {
                if character.character_id() == character_id {
                    // Position
                    match action.0 {
                        Act::Walk => {
                            // Look direction
                            sprite.flip_x =
                                character_transform.translation.x > character_position.position.x;

                            // Snap
                            character_transform.translation.x = character_position.position.x;
                            character_transform.translation.y = character_position.position.y;

                            *ani_action = AniAction { act: action.0 };
                        }
                        Act::Attack => {
                            // Look direction
                            if let Some(actor_target_at_position) = actor_target_at.position {
                                sprite.flip_x = character_transform.translation.x
                                    > actor_target_at_position.position.x;

                                // TODO: use total frame /2
                                // TOFIX: damage position max to radius
                                if animation.progress.frame == 3 && ani_action.act != Act::Attack {
                                    // Damage
                                    let actor_position = character_position;
                                    let delta =
                                        actor_target_at_position.position - actor_position.position;
                                    let _distance = delta.length();
                                    let direction = delta.normalize_or_zero();
                                    let damage_position = actor_position.position + delta;

                                    let damage = Damage {
                                        by: *character_info.kind(),
                                        position: damage_position,
                                        power: character_info.attack() as f32,
                                        radius: 48.,
                                        direction,
                                        duration: 0.5,
                                    };

                                    println!("💥 DamageEvent:{:?}", damage);
                                    damage_events.send(DamageEvent(damage));
                                    *ani_action = AniAction { act: action.0 };
                                };

                                // This weird
                                if animation.progress.frame > 5 {
                                    *ani_action = AniAction { act: Act::Idle };
                                }
                            };
                        }
                        Act::Die => {
                            // TODO: play once by value from char.json
                            let total_frame = match character_info.kind() {
                                CharacterKind::Human => 12,
                                CharacterKind::Monster => 9,
                                CharacterKind::Animal => todo!(),
                            };

                            if animation.progress.repetition >= 1
                                && animation.progress.frame == total_frame
                            {
                                animation.playing = false;
                            } else {
                                *ani_action = AniAction { act: action.0 };
                            }
                        }
                        _ => (),
                    }

                    // Action
                    let animation_name = get_animation_name(ani_type, action.0);

                    if let Some(animation_id) = library.animation_with_name(animation_name) {
                        if animation.animation_id != animation_id {
                            debug!("🥹 ACT: {}", action.0);
                            animation.switch(animation_id);
                        }
                    }
                }
            }
        }
    }
}
