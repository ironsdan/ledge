use crate::sprite::*;
use crate::lib::*;
// use crate::physics::*;

use std::sync::Arc;
use vulkano::image::SwapchainImage;
use winit::window::Window;

#[derive(Clone)]
pub struct Entity {
    pub name: String,
    id: u8,
    pub size: [u32; 2],
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub sprite: Sprite,
    pub grounded: bool,
    jump_cleared: bool,
    bounding_box: Rect,
    pub horizontal_move: bool,
    direction: bool,
    gravity_multiplier: f32,
}

impl Entity {
    pub fn new(name: String, id: u8, position: [f32; 2], sprite: Sprite, size: [u32; 2]) -> Self {
        let texture_coord = [
            [0.0, 0.0],
            [0.0, 0.0],
            [0.0, 0.0],
            [0.0, 0.0],
        ];
        let screen_size = convert_to_screen_space([size[0] - 5, size[1]], [600, 800]);
        let bounding_box = Rect::new(screen_size[0], screen_size[1], position, texture_coord);

        Self {
            name: name,
            id: id,
            size: size,
            position: position,
            velocity: [0.0, 0.0],
            sprite: sprite,
            grounded: false,
            jump_cleared: true,
            bounding_box: bounding_box,
            horizontal_move: false,
            direction: true,
            gravity_multiplier: 1.0,
        }
    }

    // pub fn new(name: String, id: u8, position: [f32; 2], texture: Arc<vulkano::image::ImmutableImage<vulkano::format::Format>>, matrix_dims: [u32; 2], size: [u32; 2], gravity_multiplier: f32) -> Self {
    //     let sprite = Sprite::new(name.clone(), texture, position, size, matrix_dims);
    //     let texture_coord = [
    //         [0.0, 0.0],
    //         [0.0, 0.0],
    //         [0.0, 0.0],
    //         [0.0, 0.0],
    //     ];
    //     let screen_size = convert_to_screen_space([size[0] - 5, size[1]], [600, 800]);
    //     let bounding_box = Rect::new(screen_size[0], screen_size[1], position, texture_coord);

    //     Self {
    //         name: name,
    //         id: id,
    //         size: size,
    //         position: position,
    //         velocity: [0.0, 0.0],
    //         sprite: sprite,
    //         grounded: false,
    //         jump_cleared: true,
    //         bounding_box: bounding_box,
    //         horizontal_move: false,
    //         direction: true,
    //         gravity_multiplier: gravity_multiplier,
    //     }
    // }

    pub fn set_texture_coords(&mut self) {
        let texture_coords;
        
        if self.direction {
            texture_coords = [
                [0.0, 0.0],
                [0.0, 1.0],
                [1.0, 0.0],
                [1.0, 1.0],
            ];
        } else {
            texture_coords = [
                [1.0, 0.0],
                [1.0, 1.0],
                [0.0, 0.0],
                [0.0, 1.0],
            ];
        }

        for i in 0..4 {
            self.sprite.rect.vertices[i].tex_coords = texture_coords[i];
        }
    }

    pub fn take_input(&mut self, input: MovementInput) {
        match input {
            MovementInput::UpPress => {
                if self.jump_cleared && self.grounded {
                    self.velocity[1] = 0.008;
                    self.grounded = false;
                    self.jump_cleared = false;
                }
            }
            MovementInput::UpRelease => {
                self.jump_cleared = true;
            }
            MovementInput::Left => {
                if self.grounded {
                    self.velocity[0] -= 0.00025;
                } else {
                    self.velocity[0] -= 0.00025;
                }
                self.direction = false;
                self.horizontal_move = true;
            }
            MovementInput::Right => {
                if self.grounded {
                    self.velocity[0] += 0.00025;
                } else {
                    self.velocity[0] += 0.00025;
                }
                self.direction = true;
                self.horizontal_move = true;
            }
            // _ => ()
        }
        self.set_texture_coords();
    }

    pub fn resize(&mut self, images: &[Arc<SwapchainImage<Window>>]) {
        let dimensions = images[0].dimensions();

        let mut screen_size = convert_to_screen_space([self.size[0] - 5, self.size[1]], dimensions);
        self.bounding_box.update_size(screen_size);
        screen_size = convert_to_screen_space([self.size[0], self.size[1]], dimensions);
        self.sprite.update_size(screen_size);
    }
}

// impl Collidable for Entity {
//     fn get_velocity(&self) -> [f32; 2] {
//         return self.velocity;
//     }
//     fn set_velocity(&mut self, velocity: [f32; 2]) {
//         self.velocity = velocity;
//     }
//     fn get_position(&self) -> [f32; 2] {
//         return self.position
//     }
//     fn get_grounded(&self) -> bool {
//         return self.grounded;
//     }
//     fn set_grounded(&mut self, is_grounded: bool) {
//         self.grounded = is_grounded;
//     }
//     fn get_sprite(&self) -> &Sprite {
//         return &self.sprite;
//     }
//     fn get_horizontal_move(&self) -> bool {
//         return self.horizontal_move;
//     }
//     fn update_position(&mut self, position: [f32;2]) {
//         self.position = position;
//         self.bounding_box.update(position);
//         self.sprite.update_rect(position);
//     }
//     fn get_name(&self) -> String {
//         return self.name.clone();
//     }

//     fn get_bounding_box(&self) -> &Rect {
//         return &self.bounding_box;
//     }

//     fn get_gravity_multiplier(&self) -> f32 {
//         return self.gravity_multiplier;
//     }
// }