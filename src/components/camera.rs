use specs::Component;
use specs::VecStorage;
use specs_derive::Component;

#[derive(Clone, Component)]
#[storage(VecStorage)]
pub struct Camera {
    pub ratio: f32,
    pub perspective: na::Perspective3<f32>,
}

impl Camera {
    pub fn new(resolution: (f32, f32), farz: f32) -> Self {
        let ratio = resolution.0 / resolution.1;
        Self {
            ratio,
            perspective: na::Perspective3::new(ratio, std::f32::consts::PI / 2.0, 0.01, farz),
            ..Default::default()
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            ratio: 16.0 / 9.0,
            perspective: na::Perspective3::new(16.0 / 9.0, std::f32::consts::PI / 2.0, 0.01, 100.0),
        }
    }
}
