pub mod draw;
pub mod shaders;
pub mod window;

use glium::implement_vertex;
use glium::vertex::VertexBufferAny;

pub type Window = window::Window;

#[derive(Copy, Clone)]
// R G B A
pub struct Color(f32, f32, f32, f32);

impl Default for Color {
    fn default() -> Self {
        Color(1.0, 1.0, 1.0, 1.0)
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

impl Window {
    pub fn construct_box(&self) -> VertexBufferAny {
        glium::VertexBuffer::new(
            &self.facade,
            &[
                Vertex {
                    position: [-1.0, -1.0],
                    tex_coords: [0.0, 0.0],
                },
                Vertex {
                    position: [-1.0, 1.0],
                    tex_coords: [0.0, 1.0],
                },
                Vertex {
                    position: [1.0, 1.0],
                    tex_coords: [1.0, 1.0],
                },
                Vertex {
                    position: [1.0, -1.0],
                    tex_coords: [1.0, 0.0],
                },
            ],
        )
        .unwrap()
        .into_vertex_buffer_any()
    }
}
