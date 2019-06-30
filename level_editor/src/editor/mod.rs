pub mod carret_controller;
pub mod carret_tile;
pub mod editor_events;
pub mod json_export;


use crate::components::*;
use specs::world::Builder;

pub fn register_systems<'a>(
    builder: specs::DispatcherBuilder<'a, 'a>,
) -> specs::DispatcherBuilder<'a, 'a> {
    builder
    .with(
        carret_controller::CarretControllerSystem,
        "Carret Controller System",
        &[],
    )
    .with(
        carret_tile::CarretTileSystem,
        "Carret Tile System",
        &[],
    )
}

pub fn register_default(world: &mut specs::World) {
    world.register::<carret_controller::CarretController>();
    world.register::<carret_tile::CarretTile>();
    add_default_resources(world);
}

fn add_default_resources(world: &mut specs::World) {
    world.add_resource(editor_events::EditorEvents{..Default::default()});
}


pub fn setup_scene(world: &mut specs::World) {
    let tile_list = world.read_resource::<TileList>().clone();
    let carret = world
        .create_entity()
        .with(carret_controller::CarretController)
        .with(Drawable{
            sprite: String::from("carret"),
            layer: 5,
            ..Default::default()
        })
        .with(Transform {
            position: nalgebra::Point2::new(0.0, 0.0),
            size: nalgebra::Vector2::repeat(0.2),
            ..Default::default()
        })
        .build();
    let _carret_tile = world
        .create_entity()
        .with(carret_tile::CarretTile(0))
        .with(Drawable{
            sprite: tile_list.0[0].clone(),
            layer: 4,
            ..Default::default()
        })
        .with(Transform {
            position: nalgebra::Point2::new(0.0, 0.0),
            size: nalgebra::Vector2::repeat(0.2),
            ..Default::default()
        })
        .with(Child{
            parent: carret
        })
        .build();
}
