use crate::components::*;
use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};
use serde_json::Result;
use specs::world::Builder;

#[derive(Deserialize)]
struct JsonTile {
    position: [f32; 2],
    sprite: String,
    tile_groups: Vec<u8>
}
#[derive(Deserialize)]
struct MapFile{
    name: String,
    tiles: Vec<JsonTile>
}


pub fn load_map(world: &mut specs::World, map_path: &std::path::Path){
    let map_file = std::fs::File::open(map_path).unwrap();
    let reader = std::io::BufReader::new(map_file);
    let map: MapFile = serde_json::from_reader(reader).unwrap();
    let mut colliders = vec![];
    for tile in map.tiles{
        let tile = world
            .create_entity()
            .with(Drawable::new(tile.sprite.clone()))
            .with(Transform {
                position: nalgebra::Point3::new(tile.position[0], tile.position[1], 0.0),
                size: nalgebra::Vector2::repeat(0.2),
                ..Default::default()
            })
            .build();
        colliders.push(tile);
    }
    {
        let mut collision_world = world.write_resource::<collision::CollisionWorld>();

        for collider in colliders{
            let tile_collider = collision::ColliderBuilder::new()
                .bounds(nalgebra::Vector2::repeat(0.2))
                .build(&mut collision_world, collider);
            world
                .write_storage::<Collider>()
                .insert(collider, tile_collider)
                .unwrap();
        }
    }
}
