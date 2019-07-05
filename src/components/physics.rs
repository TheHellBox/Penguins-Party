use crate::components::Vector2;

use specs::Component;
use specs::VecStorage;
use specs_derive::Component;

#[derive(Clone, Component)]
#[storage(VecStorage)]
pub struct Physics {
    pub gravity: Vector2,
    pub max_velocity: Vector2,
    pub force: Vector2,
    pub on_ground: bool,
    pub hit_left_wall: bool,
    pub hit_right_wall: bool,
}

impl Physics {
    pub fn new() -> Self {
        Self {
            gravity: na::Vector2::new(0.0, -0.3),
            max_velocity: na::Vector2::new(0.0, -1.0),
            force: na::zero(),
            on_ground: false,
            hit_left_wall: false,
            hit_right_wall: false,
        }
    }
    pub fn apply_force(&mut self, force: Vector2) {
        self.force += force;
    }
    pub fn collision(&mut self, normal: Vector2){
        if normal == nalgebra::Vector2::new(0.0, -1.0) {
            self.on_ground = true;
        }
        if normal == nalgebra::Vector2::new(-1.0, 0.0) {
            self.hit_left_wall = true;
        }
        else if normal == nalgebra::Vector2::new(1.0, 0.0) {
            self.hit_right_wall = true;
        }
        self.force = na::zero();
    }
}
