pub mod context;
pub mod animation;
pub mod sprite;

use crate::graphics::context::GraphicsContext;


pub mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/graphics/shader.vert"
    }
}

pub mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/graphics/shader.frag"
    }
}

pub trait Drawable {
    fn draw(&mut self, context: &mut GraphicsContext);
    fn name(&self) -> &str;
}

// pub struct DrawSettings {}