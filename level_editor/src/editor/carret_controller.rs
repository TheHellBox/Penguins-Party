use crate::components::*;
use specs::Component;
use specs::Join;
use specs::VecStorage;
use specs_derive::Component;

#[derive(Clone, Component)]
#[storage(VecStorage)]
pub struct CarretController;

pub struct CarretControllerSystem;

impl<'a> specs::System<'a> for CarretControllerSystem {
    type SystemData = (
        specs::Read<'a, Input>,
        specs::ReadStorage<'a, CarretController>,
        specs::WriteStorage<'a, Transform>,
    );
    fn run(&mut self, (input, controllers, mut transforms): Self::SystemData) {
        use glium::glutin::VirtualKeyCode as Key;
        use glium::glutin::WindowEvent;
        for (_controller, transform) in (&controllers, &mut transforms).join() {
            for event in &input.events {
                match event {
                    WindowEvent::KeyboardInput { input, .. } => {
                        let key = input.virtual_keycode.unwrap();
                        if input.state == glium::glutin::ElementState::Pressed {
                            match key {
                                Key::A => transform.add_vector(na::Vector2::new(-0.4, 0.0)),
                                Key::D => transform.add_vector(na::Vector2::new(0.4, 0.0)),
                                Key::W => transform.add_vector(na::Vector2::new(0.0, 0.4)),
                                Key::S => transform.add_vector(na::Vector2::new(0.0, -0.4)),
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
            transform.position.x = (transform.position.x * 10.0).round() / 10.0;
            transform.position.y = (transform.position.y * 10.0).round() / 10.0;
        }
    }
}
