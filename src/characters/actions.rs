use bevy::prelude::*;
use serde::Deserialize;
use strum_macros::{Display, EnumString};

#[derive(Component, Debug)]
#[allow(dead_code)]
pub struct Action(pub Act);

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, EnumString, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Act {
    Idle,
    Walk,
    Attack,
    Open,
    Hurt,
    Die,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum LookDirection {
    Left,
    Right,
}
