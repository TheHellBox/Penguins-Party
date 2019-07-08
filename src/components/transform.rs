use crate::components::{Point3, Vector2};

use specs::Component;
use specs::VecStorage;
use specs_derive::Component;

#[derive(Clone, Component)]
#[storage(VecStorage)]
pub struct Transform {
    pub position: Point3,
    pub prev_position: Point3,
    pub physics_velocity: Vector2,
    pub rotation: na::UnitQuaternion<f32>,
    pub size: Vector2,
}

#[allow(dead_code)]
impl Transform {
    pub fn set_position(&mut self, new_position: na::Point2<f32>) {
        self.position.x = new_position.x;
        self.position.y = new_position.y;
    }
    pub fn add_vector(&mut self, vector: Vector2) {
        self.position.coords += na::Vector3::new(vector.x, vector.y, 0.0);
    }
    pub fn is_flip(&self) -> bool {
        self.rotation == na::UnitQuaternion::from_euler_angles(0.0, std::f32::consts::PI, 0.0)
    }
    pub fn transform_matrix(&self) -> na::Matrix4<f32> {
        let point_vector = na::Vector3::new(self.position[0], -self.position[1], self.position[2]);
        let tranlation = na::Translation3::from(point_vector);
        let scale = scale_matrix(self.size);
        na::Isometry3::from_parts(tranlation, self.rotation).to_homogeneous() * scale
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: na::Point3::new(0.0, 0.0, 0.0),
            prev_position: na::Point3::new(0.0, 0.0, 0.0),
            physics_velocity: na::zero(),
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
