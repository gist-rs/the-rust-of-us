use crate::{
    animations::{entities::AniType, utils::get_animation_name},
    brains::fight::TargetAt,
    characters::{
        actions::{Act, Action},
        entities::CharacterId,
    },
    core::{
        position::Position,
        stage::{CharacterInfo, GameStage, StageInfo},
    },
    interactions::damage::{Damage, DamageEvent},
};
use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

use super::{actions::AniAction, entities::CharacterKind};

#[allow(clippy::complexity)]
pub fn update_character<T>(
    game_stage: Res<GameStage>,
    mut characters: Query<
        (
            &CharacterId,
            &AniType,
            &mut Position,
            &mut Transform,
            &mut Sprite,
            &mut SpritesheetAnimation,
            &mut Action,
            &mut AniAction,
            &mut TargetAt,
            &T,
        ),
        With<T>,
    >,
    library: Res<AnimationLibrary>,
    mut damage_events: EventWriter<DamageEvent>,
) where
    T: CharacterInfo + 'static,
{
    if let Some(character_iter) = game_stage.0.get_characters_iter_by_type::<T>() {
        for character in character_iter {
            for (
                character_id,
                ani_type,
                character_position,
                mut character_transform,
                mut sprite,
                mut animation,
                action,
                mut ani_action,
                actor_target_at,
                character_info,
            ) in characters.iter_mut()
            {
                if character.character_id() == character_id {
                    // Position
                    match action.0 {
                        Act::Walk => {
                            // Look direction
                            sprite.flip_x =
                                character_transform.translation.x > character_position.xy.x;

                            // Snap
                            character_transform.translation.x = character_position.xy.x;
                            character_transform.translation.y = character_position.xy.y;

                            *ani_action = AniAction { act: action.0 };
                        }
                        Act::Attack => {
                            // Look direction
                            if let Some(actor_target_at_position) = actor_target_at.last_position {
                                sprite.flip_x = character_transform.translation.x
                                    > actor_target_at_position.xy.x;

                                // TODO: use total frame /2
                                // TOFIX: damage position max to radius
                                if animation.progress.frame == 3 && ani_action.act != Act::Attack {
                                    // Damage
                                    let actor_position = character_position;
                                    let delta = actor_target_at_position.xy - actor_position.xy;
                                    let _distance = delta.length();
                                    let direction = delta.normalize_or_zero();
                                    let damage_position = actor_position.xy + delta;

                                    let damage = Damage {
                                        by: *character_info.kind(),
                                        position: damage_position,
                                        power: character_info.attack() as f32,
                                        radius: 48.,
                                        direction,
                                        duration: 0.5,
                                    };

                                    println!("💥 DamageEvent:{:?}", damage);
                                    damage_events.send(DamageEvent(damage));
                                    *ani_action = AniAction { act: action.0 };
                                };

                                // This weird
                                if animation.progress.frame > 5 {
                                    *ani_action = AniAction { act: Act::Idle };
                                }
                            };
                        }
                        Act::Die => {
                            // TODO: play once by value from char.json
                            let total_frame = match character_info.kind() {
                                CharacterKind::Human => 12,
                                CharacterKind::Monster => 9,
                                CharacterKind::Animal => todo!(),
                            };

                            if animation.progress.repetition >= 1
                                && animation.progress.frame == total_frame
                            {
                                animation.playing = false;
                            } else {
                                *ani_action = AniAction { act: action.0 };
                            }
                        }
                        _ => (),
                    }

                    // Action
                    let animation_name = get_animation_name(ani_type, action.0);

                    if let Some(animation_id) = library.animation_with_name(animation_name) {
                        if animation.animation_id != animation_id {
                            animation.switch(animation_id);
                        }
                    }
                }
            }
        }
    }
}
