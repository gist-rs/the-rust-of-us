use bevy::prelude::*;
use bevy::utils::tracing::{debug, trace};
use big_brain::prelude::*;

use crate::core::chest::Chest;
use crate::core::play::Act;
use crate::core::position::Position;
use crate::core::stage::Enemy;

/// We steal the Guard component from the guard example.
#[derive(Component, Debug)]
pub struct Guard {
    /// How much duration the entity gets over time.
    pub per_second: f32,
    /// How much duration the entity currently has.
    pub satisfy: f32,
}

impl Guard {
    pub fn new(satisfy: f32, per_second: f32) -> Self {
        Self {
            satisfy,
            per_second,
        }
    }
}

/// A simple system that just pushes the guard value up over time.
/// Just a plain old Bevy system, big-brain is not involved yet.
pub fn guard_system(time: Res<Time>, mut guards: Query<&mut Guard>) {
    for mut guard in &mut guards {
        guard.satisfy += guard.per_second * time.delta_seconds();

        // Satisfaction is capped at 100.0
        if guard.satisfy >= 100.0 {
            guard.satisfy = 100.0;
        }

        trace!("Guard.duration: {}", guard.satisfy);
    }
}

/// An action where the actor moves to the closest chest
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct MoveToChest {
    // The movement speed of the actor.
    speed: f32,
}

/// Closest distance to a chest to be able to guard from it.
const MAX_DISTANCE: f32 = 32.;
#[allow(clippy::complexity)]
pub fn move_to_chest_action_system(
    time: Res<Time>,
    // Find all chests
    chests: Query<&Position, With<Chest>>,
    // We use Without to make disjoint queries.
    mut enemies: Query<
        (&mut Position, &mut crate::core::play::Action),
        (With<Enemy>, Without<Chest>),
    >,
    // A query on all current MoveToChest actions.
    mut action_query: Query<(&Actor, &mut ActionState, &MoveToChest, &ActionSpan)>,
) {
    // Loop through all actions, just like you'd loop over all entities in any other query.
    for (actor, mut action_state, move_to, span) in &mut action_query {
        let _guard = span.span().enter();

        // Different behavior depending on action state.
        match *action_state {
            // Action was just requested; it hasn't been seen before.
            ActionState::Requested => {
                debug!("🔥 Let's go find some chest!");
                // We don't really need any initialization code here, since the queries are cheap enough.
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                // Look up the actor's position.
                let actor_position = enemies.get_mut(actor.0).expect("actor has no position");
                let (mut actor_position, mut actor_action) = actor_position;

                // Look up the chest closest to them.
                let closest_chest = find_closest_chest(&chests, &actor_position);

                // Find how far we are from it.
                let delta = closest_chest.position - actor_position.position;

                let distance = delta.length();

                trace!("Distance: {}", distance);

                if distance > MAX_DISTANCE {
                    // We're still too far, take a step toward it!

                    trace!("Stepping closer.");

                    // How far can we travel during this frame?
                    let step_size = time.delta_seconds() * move_to.speed;
                    // Travel towards the chest-source position, but make sure to not overstep it.
                    let step = delta.normalize() * step_size.min(distance);

                    // Move the actor.
                    actor_position.position += step;

                    // Action
                    *actor_action = crate::core::play::Action(Act::Walk);
                } else {
                    // We're within the required distance! We can declare success.

                    debug!("🔥 We got there!");

                    // The action will be cleaned up automatically.
                    *action_state = ActionState::Success;

                    // Action
                    *actor_action = crate::core::play::Action(Act::Idle);
                }
            }
            ActionState::Cancelled => {
                // Always treat cancellations, or we might keep doing this forever!
                // You don't need to terminate immediately, by the way, this is only a flag that
                // the cancellation has been requested. If the actor is balancing on a tightrope,
                // for instance, you may let them walk off before ending the action.
                *action_state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

/// A utility function that finds the closest chest to the actor.
fn find_closest_chest(
    chests: &Query<&Position, With<Chest>>,
    actor_position: &Position,
) -> Position {
    *(chests
        .iter()
        .min_by(|a, b| {
            let da = (a.position - actor_position.position).length_squared();
            let db = (b.position - actor_position.position).length_squared();
            da.partial_cmp(&db).unwrap()
        })
        .expect("no chests"))
}

/// A simple action: the actor's guard shall decrease, but only if they are near a chest.
#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct LookAround {
    per_second: f32,
}

#[allow(clippy::type_complexity)]
pub fn guard_action_system(
    time: Res<Time>,
    mut guards: Query<(&Position, &mut Guard), (With<Enemy>, Without<Chest>)>,
    chests: Query<&Position, With<Chest>>,
    mut query: Query<(&Actor, &mut ActionState, &LookAround, &ActionSpan)>,
) {
    // Loop through all actions, just like you'd loop over all entities in any other query.
    for (Actor(actor), mut state, look, span) in &mut query {
        let _guard = span.span().enter();

        // Look up the actor's position and guard from the Actor component in the action entity.
        let (actor_position, mut guard) = guards.get_mut(*actor).expect("actor has no guard");

        match *state {
            ActionState::Requested => {
                // We'll start guarding as soon as we're requested to do so.
                debug!("🔥 Guarding the chest.");
                *state = ActionState::Executing;
            }
            ActionState::Executing => {
                // TODO: can be no chest
                let closest_chest = find_closest_chest(&chests, &actor_position);
                let distance = (closest_chest.position - actor_position.position).length();
                if distance < MAX_DISTANCE {
                    trace!("Guarding!");
                    guard.satisfy -= look.per_second * time.delta_seconds();

                    // Once we hit 0 duration, we stop guarding and report success.
                    if guard.satisfy <= 0.0 {
                        guard.satisfy = 0.0;
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
pub struct Guarding;

pub fn guarding_scorer_system(
    guards: Query<&Guard>,
    mut query: Query<(&Actor, &mut Score), With<Guarding>>,
) {
    for (Actor(actor), mut score) in &mut query {
        if let Ok(guard) = guards.get(*actor) {
            score.set(guard.satisfy / 100.);
        }
    }
}

pub fn get_thinker() -> ThinkerBuilder {
    let move_and_guard = Steps::build()
        .label("MoveAndGuard")
        // ...move to the chest...
        .step(MoveToChest { speed: 32.0 })
        // ...and then guard.
        .step(LookAround { per_second: 25.0 });

    Thinker::build()
        .label("GuardingThinker")
        // We don't do anything unless we're satisfy enough.
        .picker(FirstToScore { threshold: 0.8 })
        .when(Guarding, move_and_guard)
}
