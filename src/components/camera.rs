pub struct Camera {
    pub position: nalgebra::Translation3<f32>,
    pub rotation: nalgebra::UnitQuaternion<f32>,
    pub perspective: nalgebra::Perspective3<f32>,
}

impl Camera {
    pub fn new(resolution: (f32, f32)) -> Self {
        Self {
            perspective: nalgebra::Perspective3::new(
                resolution.0 / resolution.1,
                std::f32::consts::PI / 2.0,
                0.01,
                1000.0,
            ),
            ..Default::default()
        }
    }
    pub fn view(&self) -> nalgebra::Matrix4<f32> {
        nalgebra::Isometry3::from_parts(self.position, self.rotation).to_homogeneous()
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: nalgebra::Translation3::new(0.0, 0.0, -2.0),
            rotation: nalgebra::UnitQuaternion::from_euler_angles(std::f32::consts::PI, 0.0, 0.0),
            perspective: nalgebra::Perspective3::new(
                16.0 / 9.0,
                std::f32::consts::PI / 2.0,
                0.01,
                100.0,
            ),
        }
    }
}
