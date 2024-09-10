use bevy::prelude::*;

use extol_sprite_layer::LayerIndex;

#[derive(Debug, Copy, Clone, Component, PartialEq, Eq, Hash)]
pub enum SpriteLayer {
    Background,
    Character,
    Ui,
}

impl LayerIndex for SpriteLayer {
    // Convert your type to an actual z-coordinate.
    fn as_z_coordinate(&self) -> f32 {
        use SpriteLayer::*;
        match *self {
            Background => 0.,
            Character => 800.,
            Ui => 900.,
        }
    }
}
