use crate::loaders::png_loader::load_texture;
use glium::backend::Facade;
use glium::texture::{SrgbTexture2d};
use std::collections::HashMap;
use std::path::Path;

pub fn load_tiles<F: Facade + ?Sized>(facade: &F) -> HashMap<String, SrgbTexture2d> {
    let mut result = HashMap::with_capacity(64);
    let directory = Path::new("./assets/tiles");

    for dir_path in directory.read_dir().unwrap() {
        let dir_path = dir_path.unwrap().path();
        if dir_path.is_dir() {
            let dir_name = dir_path.file_name().unwrap().to_str().unwrap().to_string();
            for path in dir_path.read_dir().unwrap() {
                let path = path.unwrap().path();
                let name = path.file_stem().unwrap().to_str().unwrap().to_string();
                let texture_name = format!("{}_{}", dir_name, name);
                println!("{}", texture_name);
                result.insert(texture_name, load_texture(&path, facade).unwrap());
            }
        }
    }

    result
}
