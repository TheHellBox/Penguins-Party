use glium::backend::Facade;
use glium::texture::{ClientFormat, RawImage2d, SrgbTexture2d};
use std::collections::HashMap;
use std::path::Path;

pub fn load_texture<F: Facade + ?Sized>(path: &Path, facade: &F) -> std::io::Result<SrgbTexture2d> {
    let raw = load_raw(path)?;
    Ok(SrgbTexture2d::new(facade, raw).unwrap())
}

pub fn load_raw(path: &Path) -> std::io::Result<RawImage2d<u8>> {
    use png::ColorType::*;

    let decoder = png::Decoder::new(std::fs::File::open(path)?);
    let (info, mut reader) = decoder.read_info()?;
    let mut img_data = vec![0; info.buffer_size()];
    reader.next_frame(&mut img_data)?;
    let (data, format) = match info.color_type {
        RGB => (img_data, ClientFormat::U8U8U8),
        RGBA => (img_data, ClientFormat::U8U8U8U8),
        _ => unreachable!("Error: Unrecognized image format. Please use RGB/RGBA textures"),
    };
    Ok(RawImage2d {
        data: std::borrow::Cow::Owned(data),
        width: info.width,
        height: info.height,
        format: format,
    })
}
pub fn load_default_textures<F: Facade + ?Sized>(facade: &F) -> HashMap<String, SrgbTexture2d> {
    let mut result = HashMap::with_capacity(16);
    result.insert(
        String::from("dev"),
        load_texture(&Path::new("./assets/dev.png"), facade).unwrap(),
    );
    result.insert(
        String::from("penguin"),
        load_texture(&Path::new("./assets/penguin.png"), facade).unwrap(),
    );
    result
}
