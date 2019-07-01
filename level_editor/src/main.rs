extern crate nalgebra as na;

pub mod components;
pub mod editor;
pub mod loaders;
pub mod render;

fn main() {
    let window = render::window::Window::new();

    let mut world = specs::World::new();
    components::register_default(&mut world);
    editor::register_default(&mut world);
    let dispatcher = specs::DispatcherBuilder::new().with_thread_local(window);
    let dispatcher = editor::register_systems(dispatcher);
    let dispatcher = components::register_systems(dispatcher);
    let mut dispatcher = dispatcher.build();

    editor::setup_scene(&mut world);
    loop {
        {
            dispatcher.dispatch(&mut world.res);
        }
        {
            let mut events = world
                .read_resource::<editor::editor_events::EditorEvents>()
                .clone();
            editor::editor_events::update_events(events, &mut world);
            let mut events = world.write_resource::<editor::editor_events::EditorEvents>();
            events.0.clear();
        }
    }
}
