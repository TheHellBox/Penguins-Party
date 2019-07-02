use glium::glutin::{ContextBuilder, EventsLoop, WindowBuilder};
use glium::texture::srgb_texture2d::SrgbTexture2d;
use glium::{glutin, Display, Program};

use std::collections::HashMap;

use crate::components::*;

pub struct Window {
    pub facade: Display,
    pub events_loop: EventsLoop,
    pub textures: HashMap<String, SrgbTexture2d>,
    pub shaders: HashMap<String, Program>,
    pub gilrs: gilrs::Gilrs
}

impl Window {
    pub fn new() -> Self {
        let window = WindowBuilder::new()
            .with_dimensions(glutin::dpi::LogicalSize::new(1024f64, 768f64))
            .with_title("Penguins Party");
        let context = ContextBuilder::new()
            .with_depth_buffer(24)
            .with_vsync(false);
        let events_loop = EventsLoop::new();

        let display = Display::new(window, context, &events_loop).unwrap();
        let mut textures = crate::loaders::png_loader::load_default_textures(&display);
        textures.extend(crate::loaders::tile_loader::load_tiles(&display));
        let shaders = crate::render::shaders::compile_shaders(&display);
        let gilrs = gilrs::Gilrs::new().unwrap();
        Self {
            facade: display,
            events_loop: events_loop,
            textures: textures,
            shaders: shaders,
            gilrs: gilrs
        }
    }
}

impl<'a> specs::System<'a> for Window {
    type SystemData = (
        specs::Write<'a, Input>,
        specs::ReadStorage<'a, Camera>,
        specs::ReadStorage<'a, Drawable>,
        specs::ReadStorage<'a, Transform>,
    );
    fn run(&mut self, (mut resource_input, cameras, drawables, transforms): Self::SystemData) {
        use glium::glutin::WindowEvent;
        use specs::Join;

        self.events_loop.poll_events(|event| match event {
            glium::glutin::Event::WindowEvent { ref event, .. } => {
                resource_input.window_events.push(event.clone());
                match event {
                    WindowEvent::KeyboardInput { input, .. } => {
                        let keycode = input.virtual_keycode;
                        if let Some(keycode) = keycode {
                            resource_input.keys_state.insert(
                                input::InputType::KeyboardButton(keycode),
                                input.state == glium::glutin::ElementState::Pressed,
                            );
                        }
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        resource_input.mouse_buttons_state.insert(
                            button.clone(),
                            *state == glium::glutin::ElementState::Pressed,
                        );
                    }
                    _ => {}
                }
            }
            _ => {}
        });

        // Gilrs events capture
        while let Some(gilrs::Event { id, event, ..}) = self.gilrs.next_event() {
            match event {
                gilrs::EventType::ButtonPressed(button, _) => {
                    resource_input.keys_state.insert(
                        input::InputType::ControllerButton(button, id),
                        true,
                    );
                },
                gilrs::EventType::ButtonReleased(button, _) => {
                    resource_input.keys_state.insert(
                        input::InputType::ControllerButton(button, id),
                        false,
                    );
                },
                _ => {}
            }
        }

        let mut fdi = self.prepare_frame();
        for (camera, transform) in (&cameras, &transforms).join() {
            let view = transform.transform_matrix();
            let perspective = camera.perspective.to_homogeneous();
            for (drawable, transform) in (&drawables, &transforms).join() {
                self.draw_object(
                    drawable,
                    transform,
                    view.into(),
                    perspective.into(),
                    &mut fdi,
                );
            }
        }
        self.finish_frame(fdi);
    }
}
