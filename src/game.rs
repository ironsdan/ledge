use crate::event::*;
use crate::error::*;
use crate::sprite::*;
use crate::interface::Interface;
use image::ImageFormat;
use vulkano::image::{Dimensions, ImmutableImage};
use vulkano::format::Format;
use vulkano::sync::GpuFuture;

pub struct Game {

}

impl EventHandler for Game {
    fn update(&mut self, interface: &mut Interface) -> GameResult {
        return Ok(());
    }
    fn draw(&self, interface: &mut Interface) -> GameResult {
        let (texture, tex_future) = {
            let image = image::load_from_memory_with_format(include_bytes!("images/SweaterGuy.png"),
                ImageFormat::Png).unwrap().to_rgba8();
            let dimensions = [image.width(), image.height()];
            let image_data = image.into_raw().clone();
    
            ImmutableImage::from_iter(
                image_data.iter().cloned(),
                Dimensions::Dim2d { width: dimensions[0], height: dimensions[1] },
                Format::R8G8B8A8Srgb,
                interface.graphics_interface.queue.clone()
            ).unwrap()
        };

        let mut sprite_test = Sprite::new("Dan".to_string(), texture.clone(), [0.0, 0.0], [16, 22], [1, 1], None);
        let mut sprite_test2 = Sprite::new("Dan".to_string(), texture, [0.1, 0.1], [16, 22], [1, 1], None);
        let mut recreate_swapchain = false;
        let mut previous_frame_end = Some(tex_future.boxed());

        interface.graphics_interface.begin_frame(&mut previous_frame_end);
        interface.graphics_interface.draw(sprite_test);
        interface.graphics_interface.draw(sprite_test2);
        interface.graphics_interface.present(&mut previous_frame_end);
        return Ok(());
    }
}