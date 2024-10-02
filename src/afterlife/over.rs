use crate::core::{layer::SpriteLayer, state::GameState};
use bevy::prelude::*;

#[derive(Component)]
pub struct GameOverText;

pub fn game_over_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_state: Res<State<GameState>>,
    query: Query<(), With<GameOverText>>,
) {
    // println!("game_state:{:?}", game_state.get());
    match game_state.get() {
        GameState::Running => {
            // TODO
        }
        GameState::Clear => {
            // TODO
        }
        GameState::Over => {
            if query.is_empty() {
                show_game_over(&mut commands, &asset_server);
            }
        }
    }
}

#[derive(Bundle)]
struct TextOverBundle {
    text_bundle: TextBundle,
    sprite_layer: SpriteLayer,
    game_over_text: GameOverText,
}

fn show_game_over(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    println!("show_game_over");
    let bundle = TextOverBundle {
        text_bundle: TextBundle::from_section(
            "GAME OVER!",
            TextStyle {
                font: asset_server.load("PixelOperator-Bold.ttf"),
                font_size: 64.0,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            align_self: AlignSelf::Center,

            ..default()
        }),
        sprite_layer: SpriteLayer::Ui,
        game_over_text: GameOverText,
    };
    commands.spawn((bundle,));
}
