use bevy::prelude::Component;
use serde::Deserialize;
use strum_macros::{Display, EnumString};

#[derive(Deserialize, Clone, Debug)]
pub struct AnimationDetails {
    pub action_name: String,
    pub x: usize,
    pub y: usize,
    pub count: usize,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Ani {
    pub ani_type: AniType,
    pub texture_path: String,
    pub width: u32,
    pub height: u32,
    pub animations: Vec<AnimationDetails>,
}

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
