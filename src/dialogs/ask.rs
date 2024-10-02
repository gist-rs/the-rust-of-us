use bevy::{asset::AssetServer, prelude::*};

use crate::{characters::entities::CharacterId, core::layer::SpriteLayer};

#[allow(unused)]
#[derive(Clone, Default, Debug, PartialEq)]
pub struct AskDialogContent {
    pub position: Vec2,
    pub by: CharacterId,
    pub content: String,
}

#[allow(unused)]
#[derive(Event)]
pub struct AskDialogEvent(pub AskDialogContent);

#[derive(Component)]
pub struct AskDialog {
    pub by: CharacterId,
    pub content: String,
}

#[derive(Bundle)]
struct AskDialogBundle {
    node_bundle: NodeBundle,
    sprite_layer: SpriteLayer,
    dialog: AskDialog,
}

#[allow(clippy::type_complexity)]
pub fn update_ask_dialog(
    mut commands: Commands,
    mut ask_dialog_events: EventReader<AskDialogEvent>,
    query: Query<&AskDialog>,
    asset_server: Res<AssetServer>,
) {
    for AskDialogEvent(ask_dialog_content) in ask_dialog_events.read() {
        if query.is_empty() {
            // No dialog, show some
            show_ask_dialog(&mut commands, &asset_server, ask_dialog_content.clone());
        } else {
            // Has dialog, check if content matches
            let mut content_matches = false;
            for existing_content in query.iter() {
                if existing_content.by == ask_dialog_content.by
                    && existing_content.content == ask_dialog_content.content
                {
                    content_matches = true;
                    break;
                }
            }

            if !content_matches {
                // Content does not match, update the dialog
                show_ask_dialog(&mut commands, &asset_server, ask_dialog_content.clone());
            }
        }
    }
}

fn show_ask_dialog(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    ask_dialog_content: AskDialogContent,
) {
    println!("🎉 show_ask_dialog:{:?}", ask_dialog_content);
    let image = asset_server.load("panel-border-010.png");

    let slicer = TextureSlicer {
        border: BorderRect::square(22.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };

    let node_bundle = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    };

    commands.spawn(AskDialogBundle {
        node_bundle,
        sprite_layer: SpriteLayer::Ui,
        dialog: AskDialog {
            by: ask_dialog_content.by,
            content: ask_dialog_content.content,
        },
    });

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            {
                let [w, h] = [320.0, 64.0];
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(w),
                                height: Val::Px(h),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                margin: UiRect::all(Val::Px(20.0)),
                                ..default()
                            },
                            image: image.clone().into(),
                            ..default()
                        },
                        ImageScaleMode::Sliced(slicer.clone()),
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "STAGE CLEAR!",
                            TextStyle {
                                font: asset_server.load("PixelOperator-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::srgb(0.9, 0.9, 0.9),
                            },
                        ));
                    });
            }
        });
}
