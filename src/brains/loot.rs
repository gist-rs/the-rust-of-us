use crate::characters::actions::{Act, Action};

use crate::core::position::Position;

use crate::core::stage::{CharacterInfo, Human, Monster, Npc};

use crate::{char_type, find_closest_target_without_looted};
use bevy::{ecs::system::EntityCommands, prelude::*};
use big_brain::prelude::*;
use std::fmt::Debug;

use super::fight::TargetAt;

// #[derive(Component, Copy, Clone, Debug, Default)]
// pub struct LootTargetAt {
//     pub position: Option<Position>,
// }

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct Loot {}

#[derive(Component, Debug, Reflect)]
pub struct Looter {
    pub is_looting: bool,
    pub attention: f32,
    pub per_second: f32,
}

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct LootScorer;

#[derive(Component)]
pub struct Looted;

#[allow(clippy::type_complexity)]
pub fn loot_system<T, U>(
    time: Res<Time>,
    mut loots: Query<&mut Looter>,
    mut characters: Query<(&mut Position, &T), (With<T>, Without<U>)>,
    targets: Query<&Position, (With<U>, Without<Looted>)>,
    mut action_query: Query<&Actor>,
) where
    T: CharacterInfo + Clone + Debug + Component + 'static,
    U: Clone + Debug + Component + 'static,
{
    match char_type!(T) {
        id if id == char_type!(Human) => {
            for Actor(actor) in &mut action_query {
                // Use the loot_action's actor to look up the corresponding Looter Component.
                if let Ok(mut looter) = loots.get_mut(*actor) {
                    // Look up the actor's action.
                    if let Ok((actor_position, character_info)) = characters.get_mut(*actor) {
                        // Look up the target closest to them.
                        match find_closest_target_without_looted(&targets, &actor_position) {
                            Some((closest_target)) => {
                                // Find how far we are from it.
                                let delta = closest_target.xy - actor_position.xy;
                                let distance = delta.length();

                                // Get attention when lootable getting close.
                                if distance < character_info.line_of_sight() {
                                    looter.attention += looter.per_second * time.delta_seconds();
                                    if looter.attention >= 100.0 {
                                        looter.attention = 100.0;
                                    }

                                    trace!("Loot.attention: {}", looter.attention);
                                }
                            }
                            None => {
                                looter.attention = 0.;
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
pub fn loot_scorer_system<T: Component + Debug + Clone>(
    mut last_score: Local<Option<f32>>,
    loots: Query<&Looter>,
    mut query: Query<(&Actor, &mut Score, &ScorerSpan), (With<LootScorer>, Without<T>)>,
) {
    match char_type!(T) {
        id if id == char_type!(Human) || id == char_type!(Monster) => {
            for (Actor(actor), mut score, span) in &mut query {
                if let Ok(looter) = loots.get(*actor) {
                    let new_score = looter.attention / 100.0;

                    if looter.is_looting {
                        let _score = last_score.get_or_insert(new_score);

                        score.set(*_score);
                    } else {
                        last_score.take();
                        score.set(new_score);

                        if looter.attention >= 80.0 {
                            span.span().in_scope(|| {
                                trace!("Loot above threshold! Score: {}", looter.attention / 100.0)
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

pub fn get_looter<T>(entity_commands: &mut EntityCommands)
where
    T: CharacterInfo + Clone + Debug + 'static,
{
    match char_type!(T) {
        id if id == char_type!(Human) || id == char_type!(Monster) => {
            entity_commands.insert((
                Looter {
                    is_looting: false,
                    per_second: 4.0,
                    attention: 70.0,
                },
                LootScorer,
            ));
        }
        id if id == char_type!(Npc) => (),
        _ => (),
    }
}

#[allow(clippy::type_complexity)]
pub fn loot_action_system<T, U>(
    mut loots: Query<&mut Looter>,
    mut characters: Query<
        (&mut TargetAt, &mut Position, &mut Action, &mut Sprite),
        (With<T>, Without<U>),
    >,
    targets: Query<(&Position), (With<U>, Without<Looted>)>,
    mut action_query: Query<(&Actor, &mut ActionState, &Loot, &ActionSpan)>,
) where
    T: CharacterInfo + Clone + Debug + 'static,
    U: Component + Clone + Debug + 'static,
{
    for (Actor(actor), mut state, _loot, span) in &mut action_query {
        let _guard = span.span().enter();

        // Use the loot_action's actor to look up the corresponding Looter Component.
        if let Ok(mut looter) = loots.get_mut(*actor) {
            println!("looter:{:?}", looter);

            // Look up the actor's action.
            if let Ok((mut actor_target_at, actor_position, mut actor_action, mut sprite)) =
                characters.get_mut(*actor)
            {
                println!("state:{:?}", state);
                match *state {
                    ActionState::Requested => {
                        debug!("🦀 Time to loot! :{}", char_type!(T));
                        looter.is_looting = true;
                        *state = ActionState::Executing;
                    }
                    ActionState::Executing => {
                        trace!("Looting...");

                        // Look up the target closest to them.
                        match find_closest_target_without_looted(&targets, &actor_position) {
                            Some((closest_target)) => {
                                debug!("🦀 closest_target:{:?}", closest_target);
                                // Look direction
                                sprite.flip_x = actor_position.xy.x > closest_target.xy.x;

                                // Lock target
                                actor_target_at.last_position = Some(closest_target);

                                // Action
                                *actor_action = Action(Act::Open);
                            }
                            None => {
                                debug!("🦀 find_closest_target_with_entity NOT FOUND");
                                // Unlock target
                                actor_target_at.last_position = None;

                                // Action
                                if actor_action.0 != Act::Die {
                                    *actor_action = Action(Act::Idle);
                                }

                                // Done
                                looter.is_looting = false;
                                looter.attention = 0.;
                                *state = ActionState::Success;
                            }
                        }
                    }
                    // All Actions should make sure to handle cancellations!
                    ActionState::Cancelled => {
                        debug!("Loot was interrupted.");
                        looter.is_looting = false;
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
