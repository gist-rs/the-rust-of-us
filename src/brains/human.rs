use std::fs;

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use bevy_stat_bars::{Statbar, StatbarObserveEntity};

#[derive(Bundle)]
struct HumanBundle {
    sprite_bundle: SpriteBundle,
    texture_atlas: TextureAtlas,
    spritesheet_animation: SpritesheetAnimation,
    sprite_layer: SpriteLayer,
    marker: Human,
    ysort: YSort,
}

use crate::{
    brains::actions::Action,
    characters::bar::Health,
    core::{
        layer::{SpriteLayer, YSort},
        library::{build_library, Ani},
        map::{convert_map_to_screen, get_position_from_map},
        position::Position,
        setup::CharacterId,
        stage::{GameStage, Human},
    },
    get_thinker,
    timeline::init::LookDirection,
    Guard,
};

fn build_human(
    asset_server: &Res<AssetServer>,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    library: &mut ResMut<AnimationLibrary>,
    ani: Ani,
    human_stage_info: &Human,
) -> HumanBundle {
    let clip_fps = 30;

    let libs = build_library(atlas_layouts, library, &ani, clip_fps);

    let texture_path = ani.texture_path.clone();
    let texture = asset_server.load(texture_path);

    let at = convert_map_to_screen(human_stage_info.position.clone()).expect("Valid position");
    let position = get_position_from_map(at.0, at.1, None);

    let is_flip_x = match human_stage_info.look_direction {
        LookDirection::Left => true,
        LookDirection::Right => false,
    };

    HumanBundle {
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
            library.animation_with_name("man_idle").unwrap(),
        ),
        sprite_layer: SpriteLayer::Ground,
        marker: human_stage_info.clone(),
        ysort: YSort(0.0),
    }
}

pub fn init_human(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut library: ResMut<AnimationLibrary>,
    game_stage: Res<GameStage>,
) {
    let char_json = fs::read_to_string("assets/char.json").expect("Unable to read file");
    let characters: Vec<Ani> = serde_json::from_str(&char_json).expect("Unable to parse JSON");
    println!("characters:{:?}", characters);
    let stage = &game_stage.0;
    // println!("stage.humans:{:#?}", stage.humans);

    for human in stage.humans.iter() {
        println!("🔥 human:{:?}", human);
        if let Some(ani) = characters.iter().find(|&c| c.r#type == human.r#type) {
            let at = convert_map_to_screen(human.position.clone()).expect("Valid position");
            let position = get_position_from_map(at.0, at.1, None);

            let human_bundle = build_human(
                &asset_server,
                &mut atlas_layouts,
                &mut library,
                ani.clone(),
                human,
            );

            let human_id = commands
                .spawn(human_bundle)
                .insert(CharacterId(human.character_id.0.clone()))
                .insert((
                    Action(human.act),
                    Guard::new(75.0, 10.0),
                    Position {
                        position: Vec2::new(position.translation.x, position.translation.y),
                    },
                    get_thinker(),
                    Health::new_full(100.0),
                    Statbar::<Health> {
                        color: Color::from(bevy::color::palettes::css::YELLOW),
                        empty_color: Color::from(bevy::color::palettes::css::BLACK),
                        length: 32.0,
                        thickness: 6.0,
                        displacement: 40. * Vec2::Y,
                        ..Default::default()
                    },
                ))
                .id();

            commands
                .spawn((
                    Statbar::<Health> {
                        color: Color::WHITE,
                        empty_color: Color::BLACK,
                        length: 500.0,
                        thickness: 50.0,
                        ..Default::default()
                    },
                    StatbarObserveEntity(human_id),
                ))
                .insert(SpatialBundle {
                    transform: Transform::from_translation(-200. * Vec3::Y),
                    ..Default::default()
                });
        }
    }
}

#[allow(clippy::complexity)]
pub fn update_human(
    game_stage: Res<GameStage>,
    mut humans: Query<
        (
            &CharacterId,
            &mut Position,
            &mut Transform,
            &mut Sprite,
            &mut SpritesheetAnimation,
            &mut Action,
        ),
        With<Human>,
    >,
    library: Res<AnimationLibrary>,
) {
    for human in game_stage.0.humans.iter() {
        for (
            character_id,
            human_position,
            mut human_transform,
            mut sprite,
            mut animation,
            action,
        ) in humans.iter_mut()
        {
            if human.character_id == *character_id {
                // Look direction
                sprite.flip_x = human_transform.translation.x > human_position.position.x;

                // Position
                human_transform.translation.x = human_position.position.x;
                human_transform.translation.y = human_position.position.y;

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
