use specs::Component;
use specs::VecStorage;
use specs_derive::Component;

#[derive(Clone, Component)]
#[storage(VecStorage)]
pub struct Physics {
    pub gravity: nalgebra::Vector2<f32>,
    pub on_ground: bool,
}

impl Physics {
    pub fn new() -> Self {
        Self {
            gravity: nalgebra::Vector2::new(0.0, -0.02),
            on_ground: false,
        }
    }
}