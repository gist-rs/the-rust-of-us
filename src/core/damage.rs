use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use super::{layer::SpriteLayer, stage::CharacterInfo, state::GameState};
use crate::{
    brains::fight::Fighter,
    characters::{
        actions::{Act, Action},
        bar::Health,
        entities::CharacterKind,
    },
};
use std::fmt::Debug;

#[allow(unused)]
#[derive(Resource, Default, Debug)]
pub struct Damages(pub Vec<Damage>);

#[allow(unused)]
#[derive(Clone, Default, Debug)]
pub struct Damage {
    pub by: CharacterKind,
    pub position: Vec2,
    pub power: f32,
    pub radius: f32,
    pub direction: Vec2,
    pub duration: f32,
}

#[derive(Component)]
pub struct DamageIndicator {
    pub duration: f32,
}

#[derive(Event)]
pub struct DamageEvent(pub Damage);

pub fn spawn_damage_indicator(
    mut commands: Commands,
    mut damage_events: EventReader<DamageEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for DamageEvent(damage) in damage_events.read() {
        let shape = meshes.add(Circle {
            radius: damage.radius,
        });
        println!("💥 damage.power: {:?}", damage.power);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(shape),
                material: materials.add(Color::srgba(1.0, 0.0, 0.0, damage.power / 10.)),
                transform: Transform::from_xyz(damage.position.x, damage.position.y, 0.0)
                    .with_scale(Vec3::new(damage.radius / 100.0, damage.radius / 100.0, 1.0)),
                ..default()
            },
            SpriteLayer::Foreground,
            DamageIndicator {
                duration: damage.duration,
            },
        ));
    }
}

pub fn despawn_damage_indicator(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DamageIndicator)>,
) {
    for (entity, mut indicator) in query.iter_mut() {
        indicator.duration -= time.delta_seconds();
        if indicator.duration <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn update_damage(
    mut targets: Query<(&CharacterKind, &mut Health, &mut Action)>,
    mut damage_events: EventReader<DamageEvent>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for DamageEvent(damage) in damage_events.read() {
        targets
            .iter_mut()
            .for_each(|(kind, mut hp, mut actor_action)| {
                if actor_action.0 != Act::Die && *kind != damage.by {
                    // TODO: some bounce from damage
                    // let player_position = Vec2::new(
                    //     player_transform.translation.x,
                    //     player_transform.translation.y,
                    // );
                    // let direction_to_damage = (damage.position - player_position).normalize();
                    // let move_direction = direction_to_damage * damage.power * damage.radius;
                    // player_transform.translation += Vec3::new(move_direction.x, move_direction.y, 0.0);

                    *hp -= damage.power;

                    // Action
                    if hp.value > 0. {
                        *actor_action = Action(Act::Hurt);
                    } else {
                        *actor_action = Action(Act::Die);

                        if *kind == CharacterKind::Human {
                            println!("GameState::Over");
                            game_state.set(GameState::Over);
                        } else {
                            // do something?
                        }
                    }
                }
            });
    }
}

pub fn despawn_fighter_on_death_system<T>(
    mut commands: Commands,
    mut characters: Query<(Entity, &Action), With<T>>,
) where
    T: CharacterInfo + 'static,
{
    for (entity, action) in characters.iter_mut() {
        if action.0 == Act::Die {
            commands.entity(entity).remove::<Fighter>();
        }
    }
}
