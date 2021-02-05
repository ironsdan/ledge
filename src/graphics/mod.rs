pub mod context;
pub mod animation;
pub mod sprite;

use crate::interface::Interface;

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
    fn draw(&self, interface: &mut Interface, draw_settings: DrawSettings);
}

pub struct DrawSettings {}