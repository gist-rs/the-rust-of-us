use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_spritesheet_animation::prelude::{AnimationLibrary, SpritesheetAnimation};
use bevy_stat_bars::Statbar;

use crate::{
    brains::{fight::Fighter, loot::Looted},
    characters::{
        actions::{Act, Action},
        bar::Health,
        entities::CharacterKind,
    },
    core::{
        chest::{Chest, ChestId, ChestState, Chests},
        layer::SpriteLayer,
        scene::Decor,
    },
};
use std::fmt::Debug;

#[allow(unused)]
#[derive(Resource, Default, Debug)]
pub struct Toggles(pub Vec<Toggle>);

#[allow(unused)]
#[derive(Clone, Default, Debug)]
pub struct Toggle {
    pub position: Vec2,
    pub by: CharacterKind,
    // TODO: more generic with switch, door, ...
    pub target: ChestId,
}

#[derive(Component)]
pub struct ToggleIndicator {
    pub duration: f32,
}

#[derive(Event)]
pub struct ToggleEvent(pub Toggle);

pub fn spawn_toggle_indicator(
    mut commands: Commands,
    mut toggle_events: EventReader<ToggleEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for ToggleEvent(interaction) in toggle_events.read() {
        let shape = meshes.add(Circle { radius: 32. });
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(shape),
                material: materials.add(Color::srgba(0.0, 1.0, 0.0, 0.5)),
                transform: Transform::from_xyz(interaction.position.x, interaction.position.y, 0.0)
                    .with_scale(Vec3::new(32., 32., 1.0)),
                ..default()
            },
            SpriteLayer::Foreground,
            ToggleIndicator { duration: 3. },
        ));
    }
}

pub fn despawn_toggle_indicator(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ToggleIndicator)>,
) {
    for (entity, mut indicator) in query.iter_mut() {
        indicator.duration -= time.delta_seconds();
        if indicator.duration <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

// TODO: more generic with switch, door, ...
#[allow(clippy::type_complexity)]
pub fn update_toggle_chest(
    mut toggle_events: EventReader<ToggleEvent>,
    mut chests: ResMut<Chests>,
) {
    for ToggleEvent(toggle) in toggle_events.read() {
        if let Some(chest) = chests.0.get_mut(&toggle.target.0) {
            if chest.status == ChestState::Close {
                // Update the state
                chest.status = ChestState::Open;
            }
        }
    }
}
