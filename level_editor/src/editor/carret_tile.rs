use crate::components::*;
use crate::editor::editor_events::{EditorEvent, EditorEvents};
use specs::Component;
use specs::Join;
use specs::VecStorage;
use specs_derive::Component;

#[derive(Clone, Component)]
#[storage(VecStorage)]
pub struct CarretTile(pub usize);

pub struct CarretTileSystem;

impl<'a> specs::System<'a> for CarretTileSystem {
    type SystemData = (
        specs::Read<'a, Input>,
        specs::Read<'a, TileList>,
        specs::Write<'a, EditorEvents>,
        specs::ReadStorage<'a, Transform>,
        specs::WriteStorage<'a, CarretTile>,
        specs::WriteStorage<'a, Drawable>,
    );
    fn run(
        &mut self,
        (input, tile_list, mut editor_events, transforms, mut controllers, mut drawables): Self::SystemData,
    ) {
        use glium::glutin::VirtualKeyCode as Key;
        use glium::glutin::WindowEvent;
        for (mut controller, mut drawable, transform) in
            (&mut controllers, &mut drawables, &transforms).join()
        {
            for event in &input.events {
                match event {
                    WindowEvent::KeyboardInput { input, .. } => {
                        let key = input.virtual_keycode.unwrap();
                        if input.state == glium::glutin::ElementState::Pressed {
                            match key {
                                Key::F => {
                                    controller.0 += 1;
                                    if controller.0 >= tile_list.0.len() {
                                        controller.0 = 0;
                                    }
                                    drawable.sprite = tile_list.0[controller.0].clone();
                                }
                                Key::Space => editor_events.0.push(EditorEvent::AddTile(
                                    tile_list.0[controller.0].clone(),
                                    transform.position,
                                )),
                                Key::X => editor_events
                                    .0
                                    .push(EditorEvent::RemoveTile(transform.position)),
                                Key::G => editor_events
                                    .0
                                    .push(EditorEvent::Export(String::from("Default"))),
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
