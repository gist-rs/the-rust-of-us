use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use super::{chest::Chests, gate::Gates, map::load_map_from_csv, scene::build_scene};

pub fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut library: ResMut<AnimationLibrary>,
    chests: ResMut<Chests>,
    gates: ResMut<Gates>,
) {
    commands.spawn(Camera2dBundle::default());

    // Load map
    let (_walkables, start, goal, _path_cost, map) = load_map_from_csv("assets/map.csv").unwrap();

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
