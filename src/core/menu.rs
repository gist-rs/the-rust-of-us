use bevy::prelude::*;

use super::{
    layer::SpriteLayer,
    play::{load_timeline_from_csv, TimelineActions},
};

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                left: Val::Px(70.0),
                right: Val::Px(70.0),
                bottom: Val::Px(25.0),
                ..default()
            },
            ..default()
        })
        .with_children(|b| {
            b.spawn(
                TextBundle::from_section(
                    "RUN",
                    TextStyle {
                        font_size: 30.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_text_justify(JustifyText::Center),
            );
        });
}

pub fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut timeline_actions: ResMut<TimelineActions>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                println!("Pressed");
                *color = Color::srgb(0.35, 0.75, 0.35).into();
                if let Ok(actions) = load_timeline_from_csv("assets/timeline.csv") {
                    *timeline_actions = actions
                }
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}
