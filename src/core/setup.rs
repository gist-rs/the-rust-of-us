use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use super::{
    chest::Chests,
    gate::Gates,
    map::load_map_from_csv,
    scene::{build_scene, ChunkMap},
};

pub fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut library: ResMut<AnimationLibrary>,
    chests: ResMut<Chests>,
    gates: ResMut<Gates>,
    mut chunk_map: ResMut<ChunkMap>,
) {
    println!("🔥 setup_scene");
    commands.spawn(Camera2dBundle::default());

    // Load map
    let (walkables, start, goal, map) = load_map_from_csv("assets/map.csv").unwrap();
    *chunk_map = ChunkMap { walkables };

    println!("🔥 chunk_map:{:?}", chunk_map);

    build_scene(
        &mut commands,
        &asset_server,
        &mut atlas_layouts,
        &mut library,
        map,
        chests,
        gates,
        start,
        goal,
    );
}
