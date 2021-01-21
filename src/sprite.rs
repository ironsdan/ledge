use crate::lib::*;
use std::sync::Arc;

#[derive(Clone, PartialEq)]
pub struct Sprite {
    pub name: String,
    pub texture: Arc<vulkano::image::ImmutableImage<vulkano::format::Format>>,
    pub rect: Rect,
    pub size: [f32; 2],
    pub screen_size: [f32; 2],
}

impl Sprite {
    pub fn new(name: String, texture: Arc<vulkano::image::ImmutableImage<vulkano::format::Format>>, pos: [f32; 2], size: [f32; 2]) -> Self {
        let screen_size = convert_to_screen_space(size, [800, 600]);
        // println!("Sprite => s_w: {}, s_h: {}, w: {}, h: {}", screen_size[0], screen_size[1], size[0], size[1]);
        let texture_coord = [
            [0.0, 0.0],
            [0.0, 1.0],
            [1.0, 0.0],
            [1.0, 1.0],
        ];

        let rect = Rect::new(screen_size[0], screen_size[1], pos, texture_coord);
        // println!("Rect: {:?}", rect);
        Self {
            name: name,
            texture: texture,
            rect: rect,
            size: size,
            screen_size: screen_size,
        }
    }

    pub fn update_rect(&mut self, position: [f32; 2]) {
        self.rect.update(position);
    }

    pub fn update_size(&mut self, size: [f32; 2]) {
        self.screen_size = size;
        self.rect.update_size(size);
    }
}