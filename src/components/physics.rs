use specs::Component;
use specs::VecStorage;
use specs_derive::Component;

#[derive(Clone, Component)]
#[storage(VecStorage)]
pub struct Physics {
    pub gravity: na::Vector2<f32>,
    pub on_ground: bool,
}

impl Physics {
    pub fn new() -> Self {
        Self {
            gravity: na::Vector2::new(0.0, -1.5),
            on_ground: false,
        }
    }
}
