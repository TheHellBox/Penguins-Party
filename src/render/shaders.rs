use glium::backend::Facade;
use std::collections::HashMap;

pub const VERTEX_SHADER: &'static str = r#"
    #version 140

    in vec2 position;
    in vec2 tex_coords;
    out vec2 uv_coords;

    uniform mat4 transform;
    uniform mat4 perspective;
    uniform mat4 view;

    void main() {
        uv_coords = tex_coords;
        gl_Position = perspective * (view * transform) * vec4(position, 0.0, 1.0);
    }
"#;

pub const VERTEX_SHADER_NO_PROJ: &'static str = r#"
    #version 140

    in vec2 position;
    in vec2 tex_coords;
    out vec2 uv_coords;

    uniform mat4 matrix;

    void main() {
        uv_coords = tex_coords;
        gl_Position = matrix * vec4(position, 0.0, 1.0);
    }
"#;

pub const FRAGMENT_SHADER: &'static str = r#"
    #version 140

    in vec2 uv_coords;
    out vec4 color;

    uniform sampler2D tex;
    uniform vec4 tex_color;
    uniform vec2 uv_bounds;
    uniform vec2 uv_offset;

    void main() {
        color = texture(tex, uv_coords * uv_bounds + uv_offset) * tex_color;
    }
"#;

pub fn compile_shaders<F: Facade + ?Sized>(facade: &F) -> HashMap<String, glium::Program> {
    let mut shaders = HashMap::with_capacity(8);
    println!("Compiling shaders...");
    let simple = glium::Program::from_source(facade, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();
    let background =
        glium::Program::from_source(facade, VERTEX_SHADER_NO_PROJ, FRAGMENT_SHADER, None).unwrap();
    shaders.insert("simple".to_string(), simple);
    shaders.insert("background".to_string(), background);
    println!("Done!");
    shaders
}
