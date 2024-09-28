use crate::characters::actions::{Act, Action};
use crate::core::position::Position;
use crate::core::stage::{CharacterInfo, Human, Monster, Npc};
use crate::core::state::GameState;
use crate::{char_type, find_closest_target};
use bevy::{ecs::system::EntityCommands, prelude::*};
use big_brain::prelude::*;
use std::fmt::Debug;

#[derive(Component, Clone, Debug, Default)]
pub struct TargetAt {
    pub position: Option<Position>,
}

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct Fight {
    pub until: f32,
    pub per_second: f32,
}

#[derive(Component, Debug, Reflect)]
pub struct Fighter {
    pub is_fighting: bool,
    pub per_second: f32,
    pub angry: f32,
}

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct FightScorer;

#[allow(clippy::type_complexity)]
pub fn fight_system<T, U>(
    time: Res<Time>,
    mut fights: Query<&mut Fighter>,
    mut characters: Query<(&mut Position, &T), (With<T>, Without<U>)>,
    targets: Query<&Position, With<U>>,
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
                        match find_closest_target::<U>(&targets, &actor_position) {
                            Some(closest_target) => {
                                // Find how far we are from it.
                                let delta = closest_target.position - actor_position.position;
                                let distance = delta.length();

                                // Get angry when enemy getting close.
                                if distance < character_info.line_of_sight() {
                                    fight.angry += fight.per_second * time.delta_seconds();
                                    if fight.angry >= 100.0 {
                                        fight.angry = 100.0;
                                    }
                                    trace!("Fight.angry: {}", fight.angry);
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
                    let new_score = fight.angry / 100.0;

                    if fight.is_fighting {
                        let _score = last_score.get_or_insert(new_score);

                        score.set(*_score);
                    } else {
                        last_score.take();
                        score.set(new_score);

                        if fight.angry >= 80.0 {
                            span.span().in_scope(|| {
                                trace!("Fight above threshold! Score: {}", fight.angry / 100.0)
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
                    angry: 70.0,
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
    time: Res<Time>,
    mut fights: Query<&mut Fighter>,
    mut characters: Query<
        (&mut TargetAt, &mut Position, &mut Action, &mut Sprite),
        (With<T>, Without<U>),
    >,
    targets: Query<&Position, With<U>>,
    mut action_query: Query<(&Actor, &mut ActionState, &Fight, &ActionSpan)>,
) where
    T: CharacterInfo + Clone + Debug + 'static,
    U: CharacterInfo + Clone + Debug + 'static,
{
    for (Actor(actor), mut state, fight, span) in &mut action_query {
        let _guard = span.span().enter();

        // Use the fight_action's actor to look up the corresponding Fighter Component.
        if let Ok(mut fighter) = fights.get_mut(*actor) {
            // Look up the actor's action.
            if let Ok((mut actor_target_at, actor_position, mut actor_action, mut sprite)) =
                characters.get_mut(*actor)
            {
                match *state {
                    ActionState::Requested => {
                        debug!("🦀 Time to fight!");
                        fighter.is_fighting = true;
                        *state = ActionState::Executing;
                    }
                    ActionState::Executing => {
                        trace!("Fighting...");
                        // TODO: Fight until target hp reach 0 or target position out of range.
                        fighter.angry -= fight.per_second * time.delta_seconds();

                        if fighter.angry <= fight.until {
                            // To "finish" an action, we set its state to Success or
                            // Failure.
                            debug!("🦀 Fight complete!");
                            fighter.is_fighting = false;
                            *state = ActionState::Success;

                            // Unlock target
                            actor_target_at.position = None;

                            // Action
                            *actor_action = Action(Act::Idle);
                        } else {
                            // Look up the target closest to them.
                            match find_closest_target::<U>(&targets, &actor_position) {
                                Some(closest_target) => {
                                    // Look direction
                                    sprite.flip_x =
                                        actor_position.position.x > closest_target.position.x;

                                    // Lock target
                                    actor_target_at.position = Some(closest_target);

                                    // Action
                                    *actor_action = Action(Act::Attack);
                                }
                                None => {
                                    // TODO
                                }
                            }
                        }
                    }
                    // All Actions should make sure to handle cancellations!
                    ActionState::Cancelled => {
                        debug!("Fight was interrupted.");
                        fighter.is_fighting = false;
                        *state = ActionState::Failure;

                        // Action
                        *actor_action = Action(Act::Idle);
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn game_over_system(
    mut characters: Query<&mut Action>,
    mut action_query: Query<&Actor>,
    game_state: Res<State<GameState>>,
) {
    for Actor(actor) in &mut action_query {
        if let Ok(mut actor_action) = characters.get_mut(*actor) {
            // println!("game_state:{:?}", game_state.get());
            match game_state.get() {
                GameState::Running => {
                    // Do nothing
                }
                GameState::Over => {
                    if actor_action.0 != Act::Die {
                        *actor_action = Action(Act::Idle);
                    }
                }
            }
        }
    }
}
