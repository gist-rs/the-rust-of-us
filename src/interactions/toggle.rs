use bevy::prelude::*;

use crate::{
    characters::entities::CharacterKind,
    core::chest::{ChestId, ChestState, Chests},
};
use std::fmt::Debug;

#[allow(unused)]
#[derive(Resource, Default, Debug)]
pub struct Toggles(pub Vec<Toggle>);

#[allow(unused)]
#[derive(Clone, Default, Debug)]
pub struct Toggle {
    pub position: Vec2,
    pub by: CharacterKind,
    // TODO: more generic with switch, door, ...
    pub target: ChestId,
}

#[derive(Event)]
pub struct ToggleEvent(pub Toggle);

// TODO: more generic with switch, door, ...
#[allow(clippy::type_complexity)]
pub fn update_toggle_chest(
    mut toggle_events: EventReader<ToggleEvent>,
    mut chests: ResMut<Chests>,
) {
    for ToggleEvent(toggle) in toggle_events.read() {
        if let Some(chest) = chests.0.get_mut(&toggle.target.0) {
            if chest.status == ChestState::Close {
                // Update the state
                chest.status = ChestState::Open;
            }
        }
    }
}
