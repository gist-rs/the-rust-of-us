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

#[derive(Component, Default, Clone, Deserialize, Debug, Eq, PartialEq)]
pub struct CharacterId(pub String);

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
