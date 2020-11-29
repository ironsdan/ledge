use crate::lib::*;
use std::sync::Arc;

pub struct Sprite {
    texture: Arc<vulkano::image::ImmutableImage<vulkano::format::Format>>,
    pub rect: [Vertex; 4],
    pub size: [f32; 2],
    pub screen_size: [f32; 2],
}

impl Sprite {
    pub fn new(texture: Arc<vulkano::image::ImmutableImage<vulkano::format::Format>>, pos: [f32; 2], size: [f32; 2]) -> Self {
        let pixel_size = size;

        let aspect = 0.75;
    
        let char_width = pixel_size[0]*aspect;
        let char_height = pixel_size[1];
    
        let screen_width = 0.15;
        let screen_height = (char_height / char_width) * screen_width;

        let screen_size = [screen_width, screen_height];

        let texture_coord = [
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 0.0],
            [0.0, 1.0],
        ];

        let rect = [
                Vertex {
                    position: [pos[0], pos[1]],
                    tex_coords: texture_coord[0],
                },
                Vertex {
                    position: [pos[0], pos[1] + screen_height],
                    tex_coords: texture_coord[1],
                },
                Vertex {
                    position: [pos[0] + screen_width, pos[1]],
                    tex_coords: texture_coord[2],
                },
                Vertex {
                    position: [pos[0] + screen_width, pos[1] + screen_height],
                    tex_coords: texture_coord[3],
                },
            ];
        Self {
            texture: texture,
            rect: rect,
            size: size,
            screen_size: screen_size,
        }
    }
}