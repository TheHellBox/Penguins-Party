use crate::components::*;
use serde_derive::Deserialize;
use specs::world::Builder;

#[allow(dead_code)]
#[derive(Deserialize)]
struct Tile {
    position: [f32; 2],
    sprite: String,
    tile_groups: Vec<u8>,
}
#[allow(dead_code)]
#[derive(Deserialize)]
struct MapFile {
    name: String,
    tiles: Vec<Tile>,
}

pub fn load_map(world: &mut specs::World, map_path: &std::path::Path) {
    use crate::systems::physics::collision_groups::*;
    let tiles_data = world
        .read_resource::<crate::loaders::tile_loader::TileData>()
        .clone();
    let map_file = std::fs::File::open(map_path).unwrap();
    let reader = std::io::BufReader::new(map_file);
    let map: MapFile = serde_json::from_reader(reader).unwrap();
    let mut colliders = vec![];
    for tile in map.tiles {
        let tile_data = tiles_data.0.get(&tile.sprite).unwrap();
        let tile = world
            .create_entity()
            .with(Drawable::new(tile.sprite.clone()))
            .with(Transform {
                position: na::Point3::new(tile.position[0], tile.position[1], 0.0),
                size: na::Vector2::new(tile_data.tile_size[0], tile_data.tile_size[1]),
                ..Default::default()
            })
            .build();
        colliders.push((tile, tile_data));
    }
    {
        let mut collision_world = world.write_resource::<collision::CollisionWorld>();

        for (collider, data) in colliders {
            let tile_collider = collision::ColliderBuilder::new()
                .bounds(na::Vector2::new(
                    data.collider_size[0],
                    data.collider_size[1],
                ))
                .offset(na::Vector2::new(
                    data.collider_offset[0],
                    data.collider_offset[1],
                ));
            let tile_collider = match data.tile_type {
                1 => tile_collider.membership(&[ONE_WAY]),
                _ => tile_collider.membership(&[TILE]),
            };
            let tile_collider = tile_collider.build(&mut collision_world, collider);
            world
                .write_storage::<Colliders>()
                .insert(collider, collision::Colliders(vec![tile_collider]))
                .unwrap();
        }
    }
}
