use bevy::prelude::*;
use serde::Deserialize;
use strum_macros::{Display, EnumString};

#[derive(Component, Debug)]
#[allow(dead_code)]
pub struct Action(pub Act);

#[derive(Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq, EnumString, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Act {
    #[default]
    Idle,
    Walk,
    Attack,
    Open,
    Hurt,
    Die,
}

#[derive(Deserialize, Default, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum LookDirection {
    Left,
    #[default]
    Right,
}

#[derive(Component, Default)]
pub struct AniAction {
    pub act: Act,
}
