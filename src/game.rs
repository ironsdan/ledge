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

        let (rock_texture, rock_tex_future) = {
            let image = image::load_from_memory_with_format(include_bytes!("images/rock.png"),
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

        let (back_texture, back_tex_future) = {
            let image = image::load_from_memory_with_format(include_bytes!("images/background.png"),
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

        let (poke_texture, poke_tex_future) = {
            let image = image::load_from_memory_with_format(include_bytes!("images/pokeball.png"),
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

        let (test_texture, test_tex_future) = {
            let image = image::load_from_memory_with_format(include_bytes!("images/test.png"),
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

        let (_, empty_future) = {
            ImmutableImage::from_iter(
                [0, 0, 0, 0].iter().cloned(),
                Dimensions::Dim2d { width: 1, height: 1 },
                Format::R8G8B8A8Srgb,
                interface.graphics_interface.queue.clone()
            ).unwrap()
        };

        let mut sprite_test = Sprite::new("Dan".to_string(), texture.clone(), [0.0, 0.0], [16, 22], [1, 1], None);
        let mut sprite_test2 = Sprite::new("Dan".to_string(), rock_texture.clone(), [0.1, 0.1], [16, 22], [1, 1], None);
        // let mut sprite_test3 = Sprite::new("Dan".to_string(), back_texture.clone(), [-0.1, 0.1], [16, 22], [1, 1], None);
        // let mut sprite_test4 = Sprite::new("Dan".to_string(), poke_texture.clone(), [0.1, -0.1], [16, 22], [1, 1], None);
        // let mut sprite_test5 = Sprite::new("Dan".to_string(), test_texture.clone(), [-0.1, -0.1], [16, 22], [1, 1], None);
        let mut previous_frame_end = Some(empty_future.boxed());

        interface.graphics_interface.begin_frame(&mut previous_frame_end);
        sprite_test.draw(&mut interface.graphics_interface);
        sprite_test2.draw(&mut interface.graphics_interface);
        // sprite_test3.draw(&mut interface.graphics_interface);
        // sprite_test4.draw(&mut interface.graphics_interface);
        // sprite_test5.draw(&mut interface.graphics_interface);

        interface.graphics_interface.present(&mut previous_frame_end);
        return Ok(());
    }
}