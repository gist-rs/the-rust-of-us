use crate::characters::actions::Act;

use super::entities::AniType;

pub fn get_animation_name(ani_type: &AniType, act: Act) -> String {
    format!("{ani_type}_{act}")
}
