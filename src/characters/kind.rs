use bevy::prelude::Component;
use serde::Deserialize;
use strum_macros::{Display, EnumString};

#[derive(
    Component, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq, EnumString, Display,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CharacterKind {
    #[default]
    Human,
    Monster,
    Animal,
}
