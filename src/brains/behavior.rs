use bevy::prelude::*;
use serde::Deserialize;
use strum_macros::{Display, EnumString};

use crate::{
    core::stage::{CharacterInfo, Enemy, Human, Npc},
    Guard,
};

use std::fmt::Debug;

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, EnumString, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Behavior {
    GUARD,   // When have duty to protect something
    EXPLORE, // When no target in range
    OPEN,    // When see object in range e.g. Treasure, Gate, Switch
    COLLECT, // When see items in range e.g. Key, Items
    HUNT,    // When low stocks.
    FOLLOW,  // When NPC ask.
    FIGHT,   // When near Monster.
    HARVEST, // When low health, find
    REST,    // When tried, low health.
    CHILL,   // When tried, low health.
    AVOID,   // When see monster in range and low health.
}

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct BehaviorSet {
    pub behaviors: Vec<Behavior>,
}

pub fn get_behavior<T>() -> Guard
where
    T: CharacterInfo + Clone + Debug + 'static,
{
    match std::any::TypeId::of::<T>() {
        id if id == std::any::TypeId::of::<Human>() => Guard::new(75.0, 10.0),
        id if id == std::any::TypeId::of::<Enemy>() => Guard::new(75.0, 10.0),
        id if id == std::any::TypeId::of::<Npc>() => {
            todo!()
        }
        _ => todo!(),
    }
}
