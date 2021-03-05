pub mod context;
pub mod animation;
pub mod sprite;
pub mod shader;

use crate::graphics::context::GraphicsContext;

pub mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/graphics/texture.vert"
    }
}

pub mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/graphics/texture.frag"
    }
}

pub trait Drawable {
    fn draw(&mut self, context: &mut GraphicsContext);
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Vertex {
    pub a_pos: [f32; 2],
    pub a_uv: [f32; 2],
    pub a_vert_color: [f32; 4],
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DrawSettings {
    pub texture_coords: [f32; 2],
    pub color: [f32; 4],
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct InstanceData {
    a_src: [f32; 4],
    a_color: [f32; 4],
    a_transform: [[f32; 4]; 4],
}