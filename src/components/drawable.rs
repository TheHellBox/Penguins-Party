use crate::render::Color;
use specs::Component;
use specs::VecStorage;
use specs_derive::Component;

#[derive(Default, Clone, Component)]
#[storage(VecStorage)]
pub struct Drawable {
    pub sprite: String,
    pub color: Color,
    pub uv_bounds: [f32; 2],
    pub uv_offset: [f32; 2],
    pub enabled: bool,
}

impl Drawable {
    pub fn new(sprite: String) -> Self {
        Self {
            sprite,
            uv_bounds: [1.0, 1.0],
            uv_offset: [0.0, 0.0],
            enabled: true,
            ..Default::default()
        }
    }
}
