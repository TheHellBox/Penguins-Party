use crate::loaders::png_loader::load_texture;
use glium::backend::Facade;
use glium::texture::SrgbTexture2d;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Clone)]
pub struct TileData(pub HashMap<String, TileDescription>);

#[derive(Deserialize, Clone)]
pub struct TileDescription {
    pub collider_offset: [f32; 2],
    pub collider_size: [f32; 2],
    pub tile_size: [f32; 2],
    pub tile_type: usize,
}

pub fn load_tiles<F: Facade + ?Sized>(facade: &F) -> (HashMap<String, SrgbTexture2d>, TileData) {
    let mut result = HashMap::with_capacity(64);
    let mut description = HashMap::with_capacity(64);
    let directory = Path::new("./assets/tiles");

    for dir_path in directory.read_dir().unwrap() {
        let dir_path = dir_path.unwrap().path();
        if dir_path.is_dir() {
            let dir_name = dir_path.file_name().unwrap().to_str().unwrap().to_string();
            for path in dir_path.read_dir().unwrap() {
                let path = path.unwrap().path();
                let name = path.file_stem().unwrap().to_str().unwrap().to_string();
                let texture_name = format!("{}_{}", dir_name, name);
                match path.extension().unwrap().to_str().unwrap() {
                    "png" => {
                        result.insert(texture_name.clone(), load_texture(&path, facade).unwrap());
                    }
                    "json" => {
                        let tile_info_file = std::fs::File::open(path).unwrap();
                        let reader = std::io::BufReader::new(tile_info_file);
                        let tile_info: TileDescription = serde_json::from_reader(reader).unwrap();

                        description.insert(texture_name, tile_info);
                    }
                    _ => {}
                }
            }
        }
    }
    (result, TileData(description))
}
