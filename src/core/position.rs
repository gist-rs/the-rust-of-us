use crate::Component;
use crate::Vec2;

#[derive(Component, Debug, Copy, Clone, Default)]
pub struct Position {
    pub position: Vec2,
}
