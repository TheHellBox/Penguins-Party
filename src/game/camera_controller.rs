use crate::components::{Camera, Transform};
use crate::game::PlayerController;
use nalgebra::center;
use nalgebra::distance;

pub struct CameraController;

impl<'a> specs::System<'a> for CameraController {
    type SystemData = (
        specs::WriteStorage<'a, Camera>,
        specs::ReadStorage<'a, PlayerController>,
        specs::WriteStorage<'a, Transform>,
    );
    fn run(&mut self, (mut cameras, controllers, mut transforms): Self::SystemData) {
        use specs::Join;

        let mut player_transforms = vec![];

        for (controller, transform) in (&controllers, &transforms).join() {
            if controller.alive() {
                player_transforms.push(transform.clone());
            }
        }

        for (_camera, camera_transform) in (&mut cameras, &mut transforms).join() {
            let mut camera_min_vec = na::Point2::new(999.0, 999.0);
            let mut camera_max_vec = na::Point2::new(-999.0, -999.0);

            for transform in &player_transforms {
                if transform.position.x > camera_max_vec.x {
                    camera_max_vec.x = transform.position.x;
                }
                if transform.position.y > camera_max_vec.y {
                    camera_max_vec.y = transform.position.y;
                }
                if transform.position.x < camera_min_vec.x {
                    camera_min_vec.x = transform.position.x;
                }
                if transform.position.y < camera_min_vec.y {
                    camera_min_vec.y = transform.position.y;
                }
            }

            let camera_middle_point = center(&camera_max_vec, &camera_min_vec);
            let distance = distance(&camera_max_vec, &camera_min_vec);
            let ratio = 1024.0 / 768.0;
            let fov = std::f32::consts::PI / 2.0;
            let far = (-distance * 0.7 / ratio) / (fov / 2.0).tan() - 3.0;
            camera_transform.position =
                na::Point3::new(-camera_middle_point.x, camera_middle_point.y, far);
        }
    }
}
