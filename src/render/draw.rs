use glium::backend::Facade;
use glium::DrawParameters;
use glium::{uniform, Surface};

use crate::components::*;
use crate::render::Window;
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};

pub struct FrameDrawInfo<S: Surface + ?Sized> {
    surface: Box<S>,
}

impl Window {
    pub fn prepare_frame(&self) -> FrameDrawInfo<glium::Frame> {
        let resolution = self.facade.get_framebuffer_dimensions();
        let mut window_frame = glium::Frame::new(self.facade.get_context().clone(), resolution);
        window_frame.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0);
        FrameDrawInfo {
            surface: Box::new(window_frame),
        }
    }
    pub fn draw_object<S: Surface + ?Sized>(
        &self,
        drawable: &Drawable,
        transform: &Transform,
        view: [[f32; 4]; 4],
        perspective: [[f32; 4]; 4],
        target: &mut FrameDrawInfo<S>,
    ) {
        if let Some(texture) = self.textures.get(&drawable.sprite) {
            let vertex_buffer = self.construct_box();
            let index_buffer = glium::IndexBuffer::new(
                &self.facade,
                glium::index::PrimitiveType::TriangleStrip,
                &[1 as u16, 2, 0, 3],
            )
            .unwrap();

            let texture = texture
                .sampled()
                .minify_filter(MinifySamplerFilter::Nearest)
                .magnify_filter(MagnifySamplerFilter::Nearest);
            let matrix: [[f32; 4]; 4] = transform.transform_matrix().into();
            let color = [
                drawable.color.0,
                drawable.color.1,
                drawable.color.2,
                drawable.color.3,
            ];
            target.surface.draw(
                &vertex_buffer,
                &index_buffer,
                &self.shaders["simple"],
                &uniform!(tex: texture, uv_bounds: drawable.uv_bounds, uv_offset: drawable.uv_offset,
                    tex_color: color, perspective: perspective, view: view,
                    transform: matrix),
                &get_params()
            ).unwrap();
        }
    }
    pub fn finish_frame(&self, target: FrameDrawInfo<glium::Frame>) {
        target.surface.finish().unwrap();
    }
}

fn get_params() -> DrawParameters<'static> {
    use glium::{draw_parameters, Depth, DepthTest};
    DrawParameters {
        depth: Depth {
            test: DepthTest::Overwrite,
            write: true,
            ..Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullingDisabled,
        dithering: false,
        blend: draw_parameters::Blend::alpha_blending(),
        ..Default::default()
    }
}
