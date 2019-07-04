use specs::Component;
use specs::VecStorage;
use specs_derive::Component;

#[derive(Clone, Component)]
#[storage(VecStorage)]
pub struct Physics {
    pub gravity: na::Vector2<f32>,
    pub force: na::Vector2<f32>,
    pub on_ground: bool,
}

impl Physics {
    pub fn new() -> Self {
        Self {
            gravity: na::Vector2::new(0.0, -20.0),
            force: na::zero(),
            on_ground: false,
        }
    }
    pub fn apply_force(&mut self, force: na::Vector2<f32>) {
        self.force += force;
    }
}
