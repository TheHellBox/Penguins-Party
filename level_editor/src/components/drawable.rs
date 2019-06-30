use crate::render::Color;
use specs::Component;
use specs::VecStorage;
use specs_derive::Component;

#[derive(Clone, Component)]
#[storage(VecStorage)]
pub struct Drawable {
    pub sprite: String,
    pub color: Color,
    pub layer: u8,
    pub uv_bounds: [f32; 2],
    pub uv_offset: [f32; 2],
}

impl Drawable {
    pub fn new(sprite: String) -> Self {
        Self {
            sprite,
            ..Default::default()
        }
    }
}

impl Default for Drawable {
    fn default() -> Self{
        Self {
            sprite: String::from("dev"),
            uv_bounds: [1.0, 1.0],
            uv_offset: [0.0, 0.0],
            layer: 0,
            color: Default::default()
        }
    }
}
