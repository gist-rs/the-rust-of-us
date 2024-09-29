use crate::characters::actions::Act;
use bevy::prelude::Component;
use serde::Deserialize;
use strum_macros::{Display, EnumString};

#[derive(
    Component, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq, EnumString, Display,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AniType {
    #[default]
    Man,
    Skeleton,
    Crab,
    Gate,
    Chest,
}

pub fn get_animation_name(ani_type: &AniType, act: Act) -> String {
    format!("{ani_type}_{act}")
}
