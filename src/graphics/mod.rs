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

pub mod vs_color {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/graphics/color.vert"
    }
}

pub mod fs_color {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/graphics/color.frag"
    }
}

pub trait Drawable {
    fn draw(&mut self, context: &mut GraphicsContext);
}

#[derive(Clone, PartialEq)]
pub struct DrawSettings {}