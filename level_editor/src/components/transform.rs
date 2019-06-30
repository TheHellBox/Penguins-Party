use specs::Component;
use specs::VecStorage;
use specs_derive::Component;

#[derive(Clone, Component)]
#[storage(VecStorage)]
pub struct Transform {
    pub position: nalgebra::Point2<f32>,
    pub rotation: nalgebra::UnitQuaternion<f32>,
    pub size: nalgebra::Vector2<f32>,
}

#[allow(dead_code)]
impl Transform {
    pub fn set_position(&mut self, new_position: nalgebra::Point2<f32>) {
        self.position = new_position;
    }
    pub fn add_vector(&mut self, vector: nalgebra::Vector2<f32>) {
        self.position.coords += vector;
    }
    pub fn transform_matrix(&self) -> nalgebra::Matrix4<f32> {
        let point_vector = nalgebra::Vector3::new(self.position[0], -self.position[1], 0.0);
        let tranlation = nalgebra::Translation3::from(point_vector);
        let scale = scale_matrix(self.size);
        nalgebra::Isometry3::from_parts(tranlation, self.rotation).to_homogeneous() * scale
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: nalgebra::Point2::new(0.0, 0.0),
            rotation: nalgebra::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
            size: nalgebra::Vector2::repeat(1.0),
        }
    }
}

pub fn scale_matrix(scale: nalgebra::Vector2<f32>) -> nalgebra::Matrix4<f32> {
    nalgebra::Matrix4::new(
        scale.x, 0.0, 0.0, 0.0, 0.0, scale.y, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    )
}
