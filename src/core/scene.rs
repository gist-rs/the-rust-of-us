use bevy::prelude::*;

use super::layer::{SpriteLayer, YSort};

#[derive(Resource)]
pub struct GameMap(pub Vec<Vec<String>>);

#[derive(Component)]
pub struct Decoration;

#[derive(Bundle)]
struct DecorationBundle {
    sprite_bundle: SpriteBundle,
    sprite_layer: SpriteLayer,
    marker: Decoration,
    ysort: YSort,
}

pub fn build_scene(commands: &mut Commands, asset_server: &Res<AssetServer>, map: GameMap) {
    // Spawn the background
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(10.0)),
            texture: asset_server.load("grass.png"),
            ..default()
        },
        SpriteLayer::Background,
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: true,
            stretch_value: 0.25,
        },
    ));

    // Spawn obstacles based on the map
    let cell_size = 46.;
    let half_width = 320. / 2.;
    let half_height = 320. / 2.;
    for (y, row) in map.0.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            match cell.as_str() {
                "🌳" => {
                    println!("+🌳");
                    commands.spawn(DecorationBundle {
                        sprite_bundle: SpriteBundle {
                            texture: asset_server.load("tree.png"),
                            transform: Transform::from_xyz(
                                cell_size * x as f32 - half_width,
                                cell_size * y as f32 - half_height + 30.,
                                0.0,
                            )
                            .with_scale(Vec3::splat(2.0)),
                            ..default()
                        },
                        sprite_layer: SpriteLayer::Character,
                        marker: Decoration,
                        ysort: YSort(0.0),
                    });
                }
                "🚪" => (),
                _ => (),
            }
        }
    }
}
