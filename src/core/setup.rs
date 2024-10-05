use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use crate::{
    core::scene::GameMap,
    maps::gen::{gen_map_from_public_key, refine_walkable_map},
};

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
    // let (walkables, start, goal, map) = load_map_from_csv("assets/map.csv").unwrap();
    // *chunk_map = ChunkMap { walkables };

    let pubkey = "gistmeAhMG7AcKSPCHis8JikGmKT9tRRyZpyMLNNULq";
    let (walkables, start, goal, map, graves) = gen_map_from_public_key(pubkey).unwrap();
    let mut walkables = walkables;
    let mut map = map;

    let (refined_game_map, refined_walkables) =
        refine_walkable_map(&mut walkables, &mut map, &start, &goal);

    *chunk_map = ChunkMap {
        walkables: refined_walkables,
        entrance: start.clone(),
        exit: goal.clone(),
        graves,
    };

    build_scene(
        &mut commands,
        &asset_server,
        &mut atlas_layouts,
        &mut library,
        refined_game_map,
        chests,
        gates,
        start,
        goal,
    );
}
