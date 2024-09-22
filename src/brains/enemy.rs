use std::fs;

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;
use bevy_stat_bars::{Statbar, StatbarObserveEntity};

#[derive(Bundle)]
struct EnemyBundle {
    sprite_bundle: SpriteBundle,
    texture_atlas: TextureAtlas,
    spritesheet_animation: SpritesheetAnimation,
    sprite_layer: SpriteLayer,
    marker: Enemy,
    ysort: YSort,
}

use crate::{
    characters::bar::Health,
    core::{
        layer::{SpriteLayer, YSort},
        library::{build_library, Ani},
        position::Position,
        setup::{CharacterId, Enemy},
        stage::GameStage,
    },
    get_thinker, Guard,
};

fn build_enemy(
    asset_server: &Res<AssetServer>,
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    library: &mut ResMut<AnimationLibrary>,
    ani: Ani,
) -> EnemyBundle {
    let clip_fps = 30;

    let libs = build_library(atlas_layouts, library, &ani, clip_fps);

    let texture_path = ani.texture_path.clone();
    let texture = asset_server.load(texture_path);

    EnemyBundle {
        sprite_bundle: SpriteBundle {
            texture,
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(2.0)),
            ..default()
        },
        texture_atlas: TextureAtlas {
            layout: libs[0].1.clone(),
            ..default()
        },
        spritesheet_animation: SpritesheetAnimation::from_id(
            library.animation_with_name("skeleton_idle").unwrap(),
        ),
        sprite_layer: SpriteLayer::Ground,
        marker: Enemy,
        ysort: YSort(0.0),
    }
}

pub fn init_enemy(
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
    // println!("stage.enemies:{:#?}", stage.enemies);

    for enemy in stage.enemies.iter() {
        println!("🔥 enemy:{:?}", enemy);
        if let Some(ani) = characters.iter().find(|&c| c.r#type == enemy.r#type) {
            let enemy_bundle =
                build_enemy(&asset_server, &mut atlas_layouts, &mut library, ani.clone());

            let enemy_id = commands
                .spawn(enemy_bundle)
                .insert(CharacterId(enemy.character_id.0.clone()))
                .insert((
                    Guard::new(75.0, 2.0),
                    Position {
                        position: Vec2::new(0.0, 0.0),
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
                    StatbarObserveEntity(enemy_id),
                ))
                .insert(SpatialBundle {
                    transform: Transform::from_translation(-200. * Vec3::Y),
                    ..Default::default()
                });
        }
    }
}

pub fn update_enemy(
    game_stage: Res<GameStage>,
    mut enemy_positions: Query<(&CharacterId, &mut Position, &mut Transform), With<Enemy>>,
) {
    for enemy in game_stage.0.enemies.iter() {
        for (character_id, enemy_position, mut enemy_transform) in enemy_positions.iter_mut() {
            if enemy.character_id == *character_id {
                enemy_transform.translation.x = enemy_position.position.x;
                enemy_transform.translation.y = enemy_position.position.y;
            }
        }
    }
}
