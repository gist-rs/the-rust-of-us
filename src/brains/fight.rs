use crate::characters::actions::{Act, Action};
use crate::characters::bar::Health;
use crate::core::damage::Death;
use crate::core::position::Position;
use crate::core::stage::{CharacterInfo, Human, Monster, Npc};
use crate::core::state::GameState;
use crate::{char_type, find_closest_target_with_health};
use bevy::{ecs::system::EntityCommands, prelude::*};
use big_brain::prelude::*;
use std::fmt::Debug;

#[derive(Component, Clone, Debug, Default)]
pub struct TargetAt {
    pub next_position: Option<Position>,
    pub last_position: Option<Position>,
}

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct Fight {}

#[derive(Component, Debug, Reflect)]
pub struct Fighter {
    pub is_fighting: bool,
    pub per_second: f32,
    pub attention: f32,
}

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct FightScorer;

#[allow(clippy::type_complexity)]
pub fn fight_system<T, U>(
    time: Res<Time>,
    mut fights: Query<&mut Fighter>,
    mut characters: Query<(&mut Position, &T), (With<T>, Without<U>)>,
    targets: Query<(&Health, &Position), (With<U>, Without<Death>)>,
    mut action_query: Query<&Actor>,
) where
    T: CharacterInfo + Clone + Debug + Component + 'static,
    U: CharacterInfo + Clone + Debug + Component + 'static,
{
    match char_type!(T) {
        id if id == char_type!(Human) || id == char_type!(Monster) => {
            for Actor(actor) in &mut action_query {
                // Use the fight_action's actor to look up the corresponding Fighter Component.
                if let Ok(mut fight) = fights.get_mut(*actor) {
                    // Look up the actor's action.
                    if let Ok((actor_position, character_info)) = characters.get_mut(*actor) {
                        // Look up the target closest to them.
                        match find_closest_target_with_health::<U>(&targets, &actor_position) {
                            Some((target_health_value, closest_target)) => {
                                if target_health_value > 0. {
                                    // Find how far we are from it.
                                    let delta = closest_target.xy - actor_position.xy;
                                    let distance = delta.length();

                                    // Get attention when enemy getting close.
                                    if distance < character_info.line_of_sight() {
                                        fight.attention += fight.per_second * time.delta_seconds();
                                        if fight.attention >= 100.0 {
                                            fight.attention = 100.0;
                                        }
                                        trace!("Fight.attention: {}", fight.attention);
                                    }
                                } else {
                                    fight.attention = 0.;
                                }
                            }
                            None => {
                                // TODO
                            }
                        }
                    }
                }
            }
        }
        id if id == char_type!(Npc) => (),
        _ => (),
    }
}

#[allow(clippy::type_complexity)]
pub fn fight_scorer_system<T: Component + Debug + Clone>(
    mut last_score: Local<Option<f32>>,
    fights: Query<&Fighter>,
    mut query: Query<(&Actor, &mut Score, &ScorerSpan), With<FightScorer>>,
) {
    match char_type!(T) {
        id if id == char_type!(Human) || id == char_type!(Monster) => {
            for (Actor(actor), mut score, span) in &mut query {
                if let Ok(fight) = fights.get(*actor) {
                    let new_score = fight.attention / 100.0;

                    if fight.is_fighting {
                        let _score = last_score.get_or_insert(new_score);

                        score.set(*_score);
                    } else {
                        last_score.take();
                        score.set(new_score);

                        if fight.attention >= 80.0 {
                            span.span().in_scope(|| {
                                trace!("Fight above threshold! Score: {}", fight.attention / 100.0)
                            });
                        }
                    }
                }
            }
        }
        id if id == char_type!(Npc) => (),
        _ => (),
    }
}

pub fn get_fighter<T>(entity_commands: &mut EntityCommands)
where
    T: CharacterInfo + Clone + Debug + 'static,
{
    match char_type!(T) {
        id if id == char_type!(Human) || id == char_type!(Monster) => {
            entity_commands.insert((
                Fighter {
                    is_fighting: false,
                    per_second: 4.0,
                    attention: 70.0,
                },
                FightScorer,
            ));
        }
        id if id == char_type!(Npc) => (),
        _ => (),
    }
}

#[allow(clippy::type_complexity)]
pub fn fight_action_system<T, U>(
    mut fights: Query<&mut Fighter>,
    mut characters: Query<
        (&mut TargetAt, &mut Position, &mut Action, &mut Sprite),
        (With<T>, Without<U>),
    >,
    targets: Query<(&Health, &Position), (With<U>, Without<Death>)>,
    mut action_query: Query<(&Actor, &mut ActionState, &Fight, &ActionSpan)>,
) where
    T: CharacterInfo + Clone + Debug + 'static,
    U: CharacterInfo + Clone + Debug + 'static,
{
    for (Actor(actor), mut state, _fight, span) in &mut action_query {
        let _guard = span.span().enter();

        // Use the fight_action's actor to look up the corresponding Fighter Component.
        if let Ok(mut fighter) = fights.get_mut(*actor) {
            // Look up the actor's action.
            if let Ok((mut actor_target_at, actor_position, mut actor_action, mut sprite)) =
                characters.get_mut(*actor)
            {
                match *state {
                    ActionState::Requested => {
                        debug!("🦀 Time to fight! :{}", char_type!(T));
                        fighter.is_fighting = true;
                        *state = ActionState::Executing;
                    }
                    ActionState::Executing => {
                        trace!("Fighting...");

                        // Look up the target closest to them.
                        match find_closest_target_with_health::<U>(&targets, &actor_position) {
                            Some((target_health_value, closest_target)) => {
                                if target_health_value > 0. {
                                    debug!("🦀 target_health_value:{}", target_health_value);
                                    // Look direction
                                    sprite.flip_x = actor_position.xy.x > closest_target.xy.x;

                                    // Lock target
                                    actor_target_at.last_position = Some(closest_target);

                                    // Action
                                    *actor_action = Action(Act::Attack);
                                } else {
                                    debug!("🦀🦀 NO TARGET w/ HEALTH");
                                    // Unlock target
                                    actor_target_at.last_position = None;

                                    // Action
                                    // *actor_action = Action(Act::Idle);

                                    // Done
                                    fighter.is_fighting = false;
                                    fighter.attention = 0.;
                                    *state = ActionState::Success;
                                }
                            }
                            None => {
                                debug!("🦀 find_closest_target_with_health NOT FOUND");
                                // Unlock target
                                actor_target_at.last_position = None;

                                // Action
                                if actor_action.0 != Act::Die {
                                    *actor_action = Action(Act::Idle);
                                }

                                // Done
                                fighter.is_fighting = false;
                                fighter.attention = 0.;
                                *state = ActionState::Success;
                            }
                        }
                    }
                    // All Actions should make sure to handle cancellations!
                    ActionState::Cancelled => {
                        debug!("Fight was interrupted.");
                        fighter.is_fighting = false;
                        *state = ActionState::Failure;

                        // Action
                        if actor_action.0 != Act::Die {
                            *actor_action = Action(Act::Idle);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

// #[allow(clippy::type_complexity)]
// pub fn death_system(
//     mut commands: Commands,
//     mut thinkers: Query<(&Action, Entity), With<HasThinker>>,
//     mut action_query: Query<(&Actor, &ActionState)>,
// ) {
//     for (actor, _action_state) in action_query.iter_mut() {
//         if let Some((action, thinker_entity)) =
//             thinkers.iter().find(|(_, entity)| *entity == actor.0)
//         {
//             if let Ok(actor_action) = thinkers.get(thinker_entity) {
//                 println!("action:{:?}", action.0);
//                 match actor_action.0 .0 {
//                     Act::Die => {
//                         println!("===========despawn============");
//                         commands.entity(actor.0).despawn();
//                     }
//                     _ => (),
//                 }
//             }
//         }
//     }
// }

pub fn game_over_system(game_state: Res<State<GameState>>) {
    // println!("game_state:{:?}", game_state.get());
    match game_state.get() {
        GameState::Running => {
            // Do nothing
        }
        GameState::Over => {
            // TODO: show "Game Over" text
        }
    }
}
