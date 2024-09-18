use bevy::{prelude::*, utils::HashMap};
use bevy_spritesheet_animation::prelude::*;

use super::scene::Decor;

#[derive(Resource, Default, Debug)]
pub struct Gates(pub HashMap<String, Gate>);

#[derive(Debug, Clone)]
pub struct Gate {
    pub status: GateState,
    // TODO
    #[allow(dead_code)]
    pub key: Option<String>,
}

#[derive(Component, Debug)]
pub struct GateId(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateState {
    Close,
    Open,
}

#[allow(clippy::type_complexity)]
pub fn update_gate(
    library: Res<AnimationLibrary>,
    mut gate: Query<(&GateId, &mut SpritesheetAnimation), With<Decor>>,
    gates: Res<Gates>,
) {
    for (gate_id, mut animation) in gate.iter_mut() {
        if let Some(gate) = gates.0.get(&gate_id.0) {
            if gate.status == GateState::Open {
                if let Some(open_animation_id) = library.animation_with_name("gate_open") {
                    animation.switch(open_animation_id);
                }
            }
        }
    }
}
