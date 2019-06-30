use crate::components::*;
use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct JsonTile {
    position: [f32; 2],
    sprite: String,
    tile_groups: Vec<u8>
}
#[derive(Serialize, Deserialize)]
struct MapFile{
    name: String,
    tiles: Vec<JsonTile>
}

pub fn export(name: String, transforms: Vec<Transform>, drawables: Vec<Drawable>){
    use std::io::prelude::*;
    let export_path = std::path::Path::new("./maps/").join(format!("{}.json", name.clone()));
    println!("{:?}", export_path);
    let mut map_file = MapFile{
        name: name.clone(),
        tiles: vec![]
    };
    for (transform, drawable) in transforms.iter().zip(drawables){
        let tile = JsonTile{
            position: [transform.position.x, transform.position.y],
            sprite: drawable.sprite.clone(),
            tile_groups: vec![0]
        };
        map_file.tiles.push(tile);
    }
    let file_content = serde_json::to_string_pretty(&map_file).unwrap();
    let mut file = std::fs::File::create(export_path).unwrap();
    file.write_all(file_content.as_bytes()).unwrap();
}
