use specs::Component;
use specs::VecStorage;
use specs_derive::Component;

#[derive(Clone, Component)]
#[storage(VecStorage)]
pub struct Transform {
    pub position: na::Point2<f32>,
    pub rotation: na::UnitQuaternion<f32>,
    pub size: Vector2,
}

#[allow(dead_code)]
impl Transform {
    pub fn set_position(&mut self, new_position: na::Point2<f32>) {
        self.position = new_position;
    }
    pub fn add_vector(&mut self, vector: Vector2) {
        self.position.coords += vector;
    }
    pub fn transform_matrix(&self) -> na::Matrix4<f32> {
        let point_vector = na::Vector3::new(self.position[0], -self.position[1], 0.0);
        let tranlation = na::Translation3::from(point_vector);
        let scale = scale_matrix(self.size);
        na::Isometry3::from_parts(tranlation, self.rotation).to_homogeneous() * scale
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: na::Point2::new(0.0, 0.0),
            rotation: na::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
            size: na::Vector2::repeat(1.0),
        }
    }
}

pub fn scale_matrix(scale: Vector2) -> na::Matrix4<f32> {
    na::Matrix4::new(
        scale.x, 0.0, 0.0, 0.0, 0.0, scale.y, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    )
}
