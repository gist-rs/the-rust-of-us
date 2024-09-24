use bevy::prelude::*;
use bevy::utils::tracing::{debug, trace};
use big_brain::prelude::*;

use crate::characters::actions::{Act, Action};
use crate::core::point::Exit;
use crate::core::stage::{CharacterInfo, Monster, Human, Npc};
use crate::core::{chest::Chest, grave::Grave, position::Position};
use std::fmt::Debug;

const MAX_DISTANCE: f32 = 32.;

#[derive(Component, Debug)]
pub struct Guard {
    pub per_second: f32,
    pub concern: f32,
}

impl Guard {
    pub fn new(concern: f32, per_second: f32) -> Self {
        Self {
            concern,
            per_second,
        }
    }
}

pub fn guard_system(time: Res<Time>, mut guards: Query<&mut Guard>) {
    for mut guard in &mut guards {
        guard.concern += guard.per_second * time.delta_seconds();

        if guard.concern >= 100.0 {
            guard.concern = 100.0;
        }
    }
}

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct LookAround {
    per_second: f32,
    distance: f32,
}

#[allow(clippy::type_complexity)]
pub fn guard_action_system<T: Component + Debug + Clone>(
    time: Res<Time>,
    mut guards: Query<(&Position, &mut Guard), Without<T>>,
    targets: Query<&Position, With<T>>,
    mut query: Query<(&Actor, &mut ActionState, &LookAround, &ActionSpan)>,
) {
    // Loop through all actions, just like you'd loop over all entities in any other query.
    for (Actor(actor), mut state, look_around, span) in &mut query {
        let _guard = span.span().enter();

        // Look up the actor's position and guard from the Actor component in the action entity.
        let (actor_position, mut guard) = guards.get_mut(*actor).expect("actor has no guard");

        match *state {
            ActionState::Requested => {
                // We'll start guarding as soon as we're requested to do so.
                debug!("🔥 Guarding...{}", guard.concern);
                *state = ActionState::Executing;
            }
            ActionState::Executing => {
                // TODO: can be no chest
                let closest_chest = find_closest_target::<T>(&targets, actor_position);
                let distance = (closest_chest.position - actor_position.position).length();
                if distance < look_around.distance {
                    trace!("Guarding!");
                    guard.concern -= look_around.per_second * time.delta_seconds();

                    // Once we hit 0 concern, we stop guarding and report success.
                    if guard.concern <= 0.0 {
                        guard.concern = 0.0;
                        *state = ActionState::Success;

                        debug!("🔥 Guarding success!");
                    }
                } else {
                    debug!("We're too far away!");
                    *state = ActionState::Failure;
                }
            }

            ActionState::Cancelled => {
                *state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct Duty;

pub fn guarding_scorer_system(
    guards: Query<&Guard>,
    mut query: Query<(&Actor, &mut Score), With<Duty>>,
) {
    for (Actor(actor), mut score) in &mut query {
        if let Ok(guard) = guards.get(*actor) {
            score.set(guard.concern / 100.);
        }
    }
}

const MOVEMENT_SPEED: f32 = 32.;

pub fn get_thinker<T>() -> ThinkerBuilder
where
    T: CharacterInfo + Clone + Debug + 'static,
{
    match std::any::TypeId::of::<T>() {
        id if id == std::any::TypeId::of::<Human>() => {
            let move_and_guard = Steps::build()
                .label("MoveAndGuard")
                .step(MoveToNearest::<Chest>::new(MOVEMENT_SPEED, MAX_DISTANCE))
                .step(LookAround {
                    per_second: 25.0,
                    distance: MAX_DISTANCE,
                })
                .step(MoveToNearest::<Exit>::new(MOVEMENT_SPEED, 0.));

            Thinker::build()
                .label("GuardingThinker")
                .picker(Highest)
                .when(Duty, move_and_guard)
        }
        id if id == std::any::TypeId::of::<Monster>() => {
            let move_and_guard = Steps::build()
                .label("MoveAndGuard")
                .step(MoveToNearest::<Chest>::new(MOVEMENT_SPEED, MAX_DISTANCE))
                .step(LookAround {
                    per_second: 25.0,
                    distance: MAX_DISTANCE,
                })
                .step(MoveToNearest::<Grave>::new(MOVEMENT_SPEED, MAX_DISTANCE));

            Thinker::build()
                .label("GuardingThinker")
                .picker(FirstToScore { threshold: 0.8 })
                .when(Duty, move_and_guard)
        }
        id if id == std::any::TypeId::of::<Npc>() => {
            todo!()
        }
        _ => todo!(),
    }
}

#[derive(Debug, Clone, Component, ActionBuilder)]
#[action_label = "RallyPoint"]
pub struct MoveToNearest<T: Component + Debug + Clone> {
    // We use a PhantomData to store the type of the component we're moving to.
    _marker: std::marker::PhantomData<T>,
    speed: f32,
    distance: f32,
}

impl<T: Component + Debug + Clone> MoveToNearest<T> {
    pub fn new(speed: f32, distance: f32) -> Self {
        Self {
            _marker: std::marker::PhantomData,
            speed,
            distance,
        }
    }
}

fn find_closest_target<T: Component + Debug + Clone>(
    targets: &Query<&Position, With<T>>,
    actor_position: &Position,
) -> Position {
    *(targets
        .iter()
        .min_by(|a, b| {
            let da = (a.position - actor_position.position).length_squared();
            let db = (b.position - actor_position.position).length_squared();
            da.partial_cmp(&db).unwrap()
        })
        .unwrap_or_else(|| panic!("no {:?}", std::any::type_name::<T>())))
}

#[allow(clippy::type_complexity)]
pub fn move_to_nearest_system<T: Component + Debug + Clone>(
    time: Res<Time>,
    targets: Query<&Position, With<T>>,
    mut characters: Query<(&mut Position, &mut Action), (With<HasThinker>, Without<T>)>,
    mut action_query: Query<(&Actor, &mut ActionState, &MoveToNearest<T>, &ActionSpan)>,
) {
    if targets.is_empty() {
        return;
    };

    for (actor, mut action_state, move_to, span) in &mut action_query {
        let _guard = span.span().enter();

        match *action_state {
            ActionState::Requested => {
                debug!("🔥 Let's go find a {:?}", std::any::type_name::<T>());

                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                // Look up the actor's position.
                let actor_position = characters.get_mut(actor.0).expect("actor has no position");
                let (mut actor_position, mut actor_action) = actor_position;

                // Look up the chest closest to them.
                let closest_chest = find_closest_target::<T>(&targets, &actor_position);

                // Find how far we are from it.
                let delta = closest_chest.position - actor_position.position;

                let distance = delta.length();

                trace!("Distance: {}", distance);

                if distance > move_to.distance {
                    trace!("Stepping closer.");

                    let step_size = time.delta_seconds() * move_to.speed;
                    let step = delta.normalize() * step_size.min(distance);

                    // Move the actor.
                    actor_position.position += step;

                    // Action
                    *actor_action = Action(Act::Walk);
                } else {
                    debug!("🔥 We got there!");

                    *action_state = ActionState::Success;

                    // Action
                    *actor_action = Action(Act::Idle);
                }
            }
            ActionState::Cancelled => {
                *action_state = ActionState::Failure;
            }
            _ => {}
        }
    }
}
