pub mod context;
pub mod animation;
pub mod sprite;

use crate::interface::Interface;
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::command_buffer::pool::standard::StandardCommandPoolBuilder;


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
    fn draw(&mut self, interface: &mut Interface, draw_settings: DrawSettings, builder: &mut AutoCommandBufferBuilder<StandardCommandPoolBuilder>);
    fn name(&self) -> &str;
}

pub struct DrawSettings {}