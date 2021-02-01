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
    fn draw(&self, interface: &mut Interface, previous_frame_end: &mut std::option::Option<std::boxed::Box<dyn vulkano::sync::GpuFuture>>) -> GameResult {
        // interface.graphics_interface.begin_frame(&mut previous_frame_end);
        // sprite_test.draw(&mut interface.graphics_interface);
        // sprite_test2.draw(&mut interface.graphics_interface);

        interface.graphics_interface.present(previous_frame_end);
        return Ok(());
    }
}