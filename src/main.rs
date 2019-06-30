extern crate nalgebra as na;
mod components;
mod game;
mod loaders;
mod render;
mod systems;

fn main() {
    let window = render::Window::new();
    let mut world = specs::World::new();
    components::register_default(&mut world);
    game::register_default(&mut world);

    let dispatcher = specs::DispatcherBuilder::new();

    let dispatcher = systems::register_systems(dispatcher);
    let dispatcher = game::register_systems(dispatcher);
    let dispatcher = dispatcher.with_thread_local(window);
    let mut dispatcher = dispatcher.build();

    game::setup_scene(&mut world);
    game::map_loader::load_map(&mut world, std::path::Path::new("./maps/Default.json"));
    loop {
        dispatcher.dispatch(&mut world.res);
    }
}
