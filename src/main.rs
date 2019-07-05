extern crate nalgebra as na;

mod components;
mod game;
mod loaders;
mod render;
mod systems;

use crate::components::*;

fn main() {
    let window = render::Window::new();
    let mut world = specs::World::new();
    components::register_default(&mut world);
    game::register_default(&mut world);

    let dispatcher = specs::DispatcherBuilder::new();

    let dispatcher = game::register_systems(dispatcher);
    let dispatcher = systems::register_systems(dispatcher);
    let dispatcher = components::register_systems(dispatcher);
    let dispatcher = dispatcher.with_thread_local(window);

    let mut dispatcher = dispatcher.build();

    game::setup_scene(&mut world);
    game::map_loader::load_map(&mut world, std::path::Path::new("./maps/Default.json"));
    loop {
        let loop_start_time = std::time::Instant::now();
        game::update(&mut world);
        {
            dispatcher.dispatch(&mut world.res);
            let time_elapsed_sec = loop_start_time.elapsed().as_micros() as f32 / 1000000.0;
            let mut game_state = world.write_resource::<GameState>();
            game_state.frame_time_elapsed = time_elapsed_sec;
        }
    }
}
