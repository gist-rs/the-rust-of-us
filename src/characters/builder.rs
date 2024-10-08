use std::fs;

use crate::{
    animations::{
        build::build_library,
        entities::{Ani, AniType},
        utils::get_animation_name,
    },
    brains::{
        behavior::get_behavior,
        fight::{get_fighter, TargetAt},
        loot::get_looter,
    },
    characters::{
        actions::{Act, Action, LookDirection},
        bar::Health,
        entities::CharacterId,
    },
    core::{
        layer::{SpriteLayer, YSort},
        map::{convert_map_to_screen, get_position_from_map},
        position::Position,
        scene::ChunkMap,
        stage::{CharacterInfo, GameStage, StageInfo},
    },
    get_thinker,
};
use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use bevy_stat_bars::{Statbar, StatbarObserveEntity};
use std::fmt::Debug;

use super::{actions::AniAction, entities::CharacterKind};

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

fn build_character<T: CharacterInfo>(
    asset_server: &Res<AssetServer>,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    library: &mut ResMut<AnimationLibrary>,
    ani: Ani,
    character_info: T,
    at: (usize, usize),
) -> CharacterBundle<T> {
    let clip_fps = 30;

    let libs = build_library(atlas_layouts, library, &ani, clip_fps);

    let texture_path = ani.texture_path.clone();
    let texture = asset_server.load(texture_path);

    // let at = convert_map_to_screen(character_info.position().clone()).expect("Valid position");
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
    chunk_map: Res<ChunkMap>,
) where
    T: CharacterInfo + Clone + Debug + 'static,
{
    // let char_json = fs::read_to_string("assets/char.json").expect("Unable to read file");
    let char_json = r#"[
  {
    "ani_type": "man",
    "texture_path": "man.png",
    "width": 96,
    "height": 64,
    "animations": [
      {
        "action_name": "idle",
        "x": 0,
        "y": 0,
        "count": 9
      },
      {
        "action_name": "walk",
        "x": 0,
        "y": 1,
        "count": 8
      },
      {
        "action_name": "attack",
        "x": 0,
        "y": 2,
        "count": 10
      },
      {
        "action_name": "open",
        "x": 0,
        "y": 3,
        "count": 8
      },
      {
        "action_name": "hurt",
        "x": 0,
        "y": 4,
        "count": 8
      },
      {
        "action_name": "die",
        "x": 0,
        "y": 5,
        "count": 13
      }
    ]
  },
  {
    "ani_type": "skeleton",
    "texture_path": "skeleton.png",
    "width": 96,
    "height": 64,
    "animations": [
      {
        "action_name": "idle",
        "x": 0,
        "y": 0,
        "count": 6
      },
      {
        "action_name": "walk",
        "x": 0,
        "y": 1,
        "count": 8
      },
      {
        "action_name": "attack",
        "x": 0,
        "y": 2,
        "count": 7
      },
      {
        "action_name": "hurt",
        "x": 0,
        "y": 3,
        "count": 7
      },
      {
        "action_name": "die",
        "x": 0,
        "y": 4,
        "count": 10
      }
    ]
  }
]
"#;
    let characters: Vec<Ani> = serde_json::from_str(&char_json).expect("Unable to parse JSON");
    println!("characters:{:?}", characters);

    if let Some(character_iter) = game_stage.0.get_characters_iter_by_type::<T>() {
        for character in character_iter {
            println!("🔥 character:{:?}", *character);
            if let Some(ani) = characters
                .iter()
                .find(|&c| c.ani_type == *character.ani_type())
            {
                let character_position = match *character.ani_type() {
                    AniType::Man => {
                        get_position_from_map(chunk_map.entrance.x, chunk_map.entrance.y, None)
                    }
                    AniType::Skeleton => {
                        // TODO: more grave
                        get_position_from_map(chunk_map.graves[0].x, chunk_map.graves[0].y, None)
                    }
                    _ => {
                        todo!()
                    }
                };
                let at = (chunk_map.entrance.x, chunk_map.entrance.y);

                // let at =
                //     convert_map_to_screen(character_position.clone()).expect("Valid position");
                // let position = get_position_from_map(at.0, at.1, None);

                let character_bundle = build_character::<T>(
                    &asset_server,
                    &mut atlas_layouts,
                    &mut library,
                    ani.clone(),
                    character.clone(),
                    at,
                );

                let mut entity_commands = commands.spawn(character_bundle);

                // Statics
                entity_commands
                    .insert(*character.ani_type())
                    .insert(CharacterId(character.character_id().0.clone()))
                    .insert((
                        Action(*character.act()),
                        Position {
                            // xy: Vec2::new(position.translation.x, position.translation.y),
                            xy: Vec2::new(
                                character_position.translation.x,
                                character_position.translation.y,
                            ),
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
                    ));

                // Dynamics
                get_fighter::<T>(&mut entity_commands);
                get_looter::<T>(&mut entity_commands);
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
