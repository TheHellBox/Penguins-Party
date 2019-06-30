use crate::components::*;

use specs::Component;
use specs::VecStorage;
use specs_derive::Component;

#[derive(Clone, Component)]
#[storage(VecStorage)]
pub struct PlayerController;

pub struct PlayerControllerSystem;

impl<'a> specs::System<'a> for PlayerControllerSystem {
    type SystemData = (
        specs::Read<'a, Input>,
        specs::WriteStorage<'a, PlayerController>,
        specs::WriteStorage<'a, Transform>,
        specs::WriteStorage<'a, AnimationController>,
    );
    fn run(&mut self, (input, mut players, mut transforms, mut animations): Self::SystemData) {
        use glium::glutin::VirtualKeyCode as Key;
        use specs::Join;

        for (_player, transform, animation) in
            (&mut players, &mut transforms, &mut animations).join()
        {
            let mut velocity = nalgebra::Vector2::repeat(0.0);

            if input.key_pressed(&Key::D) {
                velocity += nalgebra::Vector2::new(0.02, 0.0);
            }
            if input.key_pressed(&Key::A) {
                velocity -= nalgebra::Vector2::new(0.02, 0.0);
            }
            if input.key_pressed(&Key::W) {
                velocity += nalgebra::Vector2::new(0.00, 0.05);
            }

            if velocity.x > 0.0 {
                transform.rotation = nalgebra::UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0);
            } else if velocity.x < 0.0 {
                transform.rotation =
                    nalgebra::UnitQuaternion::from_euler_angles(0.0, std::f32::consts::PI, 0.0);
            }

            if velocity.x != 0.0 {
                animation.running = true;
            } else {
                animation.running = false;
                animation.current_frame = 0;
            }
            transform.add_vector(velocity);
        }
    }
}

pub fn spawn_player(world: &mut specs::World) -> specs::Entity {
    use crate::components::*;
    use specs::world::Builder;

    let player = world
        .create_entity()
        .with(Drawable::new(String::from("penguin")))
        .with(Transform {
            position: nalgebra::Point3::new(0.0, 2.0, 0.0),
            size: nalgebra::Vector2::new(0.3, 0.3),
            ..Default::default()
        })
        .with(PlayerController {})
        .with(Physics::new())
        .with(AnimationController::new(16).frame_update_speed(50))
        .build();
    player
}
