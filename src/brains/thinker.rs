use bevy::prelude::*;
use bevy::utils::tracing::{debug, trace};
use big_brain::prelude::*;

use crate::characters::actions::{Act, Action};
use crate::characters::bar::Health;
use crate::characters::entities::CharacterId;
use crate::core::chest::Chest;
use crate::core::grave::Grave;
use crate::core::map::{find_path, get_map_from_position, get_position_from_map};
use crate::core::point::Exit;
use crate::core::position::Position;
use crate::core::scene::ChunkMap;
use crate::core::stage::{CharacterInfo, Human, Monster, Npc};
use crate::core::state::GameState;
use crate::dialogs::ask::{AskDialogContent, AskDialogEvent};
use crate::get_type_id;
use crate::interactions::damage::Death;
use std::cmp::Ordering;
use std::fmt::Debug;

use super::fight::{Fight, FightScorer};
use super::loot::{Loot, LootScorer, Looted};

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
    mut guards: Query<(&Position, &mut Guard), (Without<T>, Without<Death>)>,
    targets: Query<&Position, (With<T>, Without<Death>)>,
    mut query: Query<(&Actor, &mut ActionState, &LookAround, &ActionSpan), Without<Death>>,
) {
    // Loop through all actions, just like you'd loop over all entities in any other query.
    for (Actor(actor), mut state, look_around, span) in &mut query {
        let _guard = span.span().enter();

        // Look up the actor's position and guard from the Actor component in the action entity.
        let (actor_position, mut guard) = guards.get_mut(*actor).expect("actor has no guard");

        match *state {
            ActionState::Requested => {
                // We'll start guarding as soon as we're requested to do so.
                // debug!("🔥 Guarding...{}", guard.concern);
                *state = ActionState::Executing;
            }
            ActionState::Executing => {
                match find_closest_target::<T>(&targets, actor_position) {
                    Some(closest_target) => {
                        let distance = (closest_target.xy - actor_position.xy).length();
                        if distance < look_around.distance {
                            // trace!("Guarding!");
                            guard.concern -= look_around.per_second * time.delta_seconds();

                            // Once we hit 0 concern, we stop guarding and report success.
                            if guard.concern <= 0.0 {
                                guard.concern = 0.0;
                                *state = ActionState::Success;

                                // debug!("🔥 Guarding success!");
                            }
                        } else {
                            debug!("We're too far away!");
                            *state = ActionState::Failure;
                        }
                    }
                    None => {
                        // TODO
                    }
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

#[allow(clippy::type_complexity)]
pub fn guarding_scorer_system(
    guards: Query<&Guard>,
    mut query: Query<(&Actor, &mut Score), (With<Duty>, Without<Death>)>,
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
    match get_type_id!(T) {
        id if id == get_type_id!(Human) => {
            let move_and_exit = Steps::build()
                .label("MoveAndLExit")
                .step(MoveToNearest::<Exit>::new(MOVEMENT_SPEED, 0.))
                .step(LookAround {
                    per_second: 25.0,
                    distance: MAX_DISTANCE,
                });

            let move_and_loot = Steps::build()
                .label("MoveAndLoot")
                .step(MoveToNearest::<Chest>::new(MOVEMENT_SPEED, MAX_DISTANCE))
                .step(Loot {})
                .step(MoveToNearest::<Exit>::new(MOVEMENT_SPEED, 0.))
                .step(LookAround {
                    per_second: 25.0,
                    distance: MAX_DISTANCE,
                });

            let move_and_fight = Steps::build()
                .label("MoveAndFight")
                .step(MoveToNearest::<Monster>::new(MOVEMENT_SPEED, MAX_DISTANCE))
                .step(Fight {});

            Thinker::build()
                .label("GuardingThinker")
                .picker(Highest)
                .when(LootScorer, move_and_loot)
                .when(FightScorer, move_and_fight)
                .when(Duty, move_and_exit)
        }
        id if id == get_type_id!(Monster) => {
            let move_and_guard = Steps::build()
                .label("MoveAndGuard")
                .step(MoveToNearest::<Chest>::new(MOVEMENT_SPEED, MAX_DISTANCE))
                .step(LookAround {
                    per_second: 25.0,
                    distance: MAX_DISTANCE,
                })
                .step(MoveToNearest::<Grave>::new(MOVEMENT_SPEED, MAX_DISTANCE));

            let move_and_fight = Steps::build()
                .label("MoveAndFight")
                .step(MoveToNearest::<Human>::new(MOVEMENT_SPEED, MAX_DISTANCE))
                .step(Fight {});

            Thinker::build()
                .label("GuardingThinker")
                .picker(Highest)
                .when(FightScorer, move_and_fight)
                .when(Duty, move_and_guard)
        }
        id if id == get_type_id!(Npc) => {
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

#[allow(clippy::type_complexity)]
pub fn find_closest_target_with_health<T: Component + Debug + Clone>(
    targets: &Query<(&Health, &Position), (With<T>, Without<Death>)>,
    actor_position: &Position,
) -> Option<(f32, Position)> {
    targets
        .iter()
        .min_by(|(_, a_pos), (_, b_pos)| {
            let da = (a_pos.xy - actor_position.xy).length_squared();
            let db = (b_pos.xy - actor_position.xy).length_squared();
            da.partial_cmp(&db).unwrap_or(Ordering::Equal)
        })
        .map(|(health, position)| (health.value, *position))
}

// TODO: generic with find_closest_target
pub fn find_closest_target_without_looted<T: Component + Debug + Clone>(
    targets: &Query<&Position, (With<T>, Without<Looted>)>,
    actor_position: &Position,
) -> Option<Position> {
    targets
        .iter()
        .min_by(|a, b| {
            let da = (a.xy - actor_position.xy).length_squared();
            let db = (b.xy - actor_position.xy).length_squared();
            da.partial_cmp(&db).unwrap_or(Ordering::Equal)
        })
        .cloned()
}

pub fn find_closest_target<T: Component + Debug + Clone>(
    targets: &Query<&Position, (With<T>, Without<Death>)>,
    actor_position: &Position,
) -> Option<Position> {
    targets
        .iter()
        .min_by(|a, b| {
            let da = (a.xy - actor_position.xy).length_squared();
            let db = (b.xy - actor_position.xy).length_squared();
            da.partial_cmp(&db).unwrap_or(Ordering::Equal)
        })
        .cloned()
}

#[allow(clippy::type_complexity)]
pub fn move_to_nearest_system<T: Component + Debug + Clone>(
    time: Res<Time>,
    targets: Query<&Position, (With<T>, Without<Death>)>,
    mut characters: Query<
        (&mut Position, &mut Action, &CharacterId),
        (With<HasThinker>, Without<T>, Without<Death>),
    >,
    mut action_query: Query<
        (&Actor, &mut ActionState, &MoveToNearest<T>, &ActionSpan),
        Without<Death>,
    >,
    chunk_map: Res<ChunkMap>,
    mut ask_dialog_events: EventWriter<AskDialogEvent>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if targets.is_empty() {
        return;
    };

    for (Actor(actor), mut action_state, move_to, span) in &mut action_query {
        let _guard = span.span().enter();

        match *action_state {
            ActionState::Requested => {
                // debug!("🔥 Let's go find a {:?}", std::any::type_name::<T>());

                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                // Look up the actor's position.
                if let Ok((mut actor_position, mut actor_action, character_id)) =
                    characters.get_mut(*actor)
                {
                    // Look up the target closest to them.
                    let closest_target = find_closest_target::<T>(&targets, &actor_position);

                    match closest_target {
                        Some(closest_target) => {
                            // Find path to target
                            let start = get_map_from_position(actor_position.xy, None);
                            let goal = get_map_from_position(closest_target.xy, None);

                            let path_cost =
                                match find_path(&chunk_map.walkables, start, goal, false) {
                                    Ok(path_cost) => Some(path_cost),
                                    Err(_) => None,
                                };

                            if let Some(path_cost) = path_cost {
                                // How close to next position
                                let (x, y) = if path_cost.path.len() == 1 {
                                    path_cost.path[0]
                                } else {
                                    path_cost.path[1]
                                };
                                let next_position_transform = get_position_from_map(x, y, None);
                                let next_translation = next_position_transform.translation;
                                let delta = next_translation.xy() - actor_position.xy;
                                let distance = delta.length();

                                let delta2 = closest_target.xy - actor_position.xy;
                                let distance2 = delta2.length();

                                if distance > move_to.distance || distance2 > move_to.distance {
                                    // Too far, walk to it
                                    trace!("Stepping closer.");

                                    let step_size = time.delta_seconds() * move_to.speed;
                                    let step = delta.normalize() * step_size.min(distance);

                                    // Move the actor.
                                    actor_position.xy += step;

                                    // Action
                                    *actor_action = Action(Act::Walk);
                                } else {
                                    // debug!("🔥 We got there!");
                                    *action_state = ActionState::Success;

                                    // Action
                                    match std::any::type_name::<T>() {
                                        // TODO: const this?
                                        "the_rust_of_us::core::point::Exit" => {
                                            let ask_dialog = AskDialogContent {
                                                position: actor_position.xy,
                                                by: character_id.clone(),
                                                content: "I did it!".to_owned(),
                                            };
                                            // println!(
                                            //     "💥 AskDialogEvent:{:?}, {:?}",
                                            //     character_id.clone(),
                                            //     ask_dialog
                                            // );
                                            ask_dialog_events.send(AskDialogEvent(ask_dialog));

                                            game_state.set(GameState::Clear);
                                        }
                                        _ => {
                                            *actor_action = Action(Act::Idle);
                                        }
                                    }
                                }
                            }
                        }
                        None => {
                            // TODO
                        }
                    }
                }
            }
            ActionState::Cancelled => {
                *action_state = ActionState::Failure;
            }
            _ => {}
        }
    }
}
