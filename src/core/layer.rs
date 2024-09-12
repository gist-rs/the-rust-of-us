use bevy::prelude::*;

use extol_sprite_layer::LayerIndex;

#[derive(Debug, Copy, Clone, Component, PartialEq, Eq, Hash)]
pub enum SpriteLayer {
    Background,
    Ground,
    Ui,
}

impl LayerIndex for SpriteLayer {
    // Convert your type to an actual z-coordinate.
    fn as_z_coordinate(&self) -> f32 {
        use SpriteLayer::*;
        match *self {
            Background => 0.,
            Ground => 500.,
            Ui => 900.,
        }
    }
}

/// Component to sort entities by their y position.
/// Takes in a base value usually the sprite default Z with possibly an height offset.
/// this value could be tweaked to implement virtual Z for jumping
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct YSort(pub f32);

/// Applies the y-sorting to the entities Z position.
pub fn y_sort(mut query: Query<(&mut Transform, &YSort)>) {
    for (mut transform, ysort) in query.iter_mut() {
        transform.translation.z = ysort.0 - transform.translation.y;
    }
}
