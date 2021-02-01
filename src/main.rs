mod lib;
mod sprite;
mod event;
mod graphics;
mod animation;
mod input;
mod interface;
mod error;
mod conf;
mod game;

use game::*;
use event::*;
use interface::*;
use error::*;
use sprite::*;

use image::ImageFormat;
use vulkano::image::{Dimensions, ImmutableImage};
use vulkano::format::Format;
use vulkano::sync::GpuFuture;

fn main() {
    let (mut interface, event_loop) = InterfaceBuilder::new("test", "Dan").build().unwrap();

    let game = Game {};

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

    let sprite_test = Sprite::new("Dan".to_string(), texture.clone(), [0.0, 0.0], [16, 22], [1, 1], None);
    let sprite_test2 = Sprite::new("Dan".to_string(), rock_texture.clone(), [0.1, 0.1], [16, 22], [1, 1], None);


    interface.graphics_interface.sprites.push(sprite_test);
    interface.graphics_interface.sprites.push(sprite_test2);


    event::run(interface, event_loop, game);
}