use crate::sprite::*;
use crate::lib::*;
use crate::physics::*;

use std::sync::Arc;

#[derive(Clone)]
pub struct Entity {
    pub name: String,
    id: u8,
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub sprite: Sprite,
    pub grounded: bool,
    jump_cleared: bool,
    pub horizontal_move: bool,
    direction: bool,
    gravity_multiplier: f32,
}

impl Entity {
    pub fn new(name: String, id: u8, position: [f32; 2], texture: Arc<vulkano::image::ImmutableImage<vulkano::format::Format>>, pos: [f32; 2], size: [f32; 2]) -> Self {
        let sprite = Sprite::new(texture, pos, size);
        Self {
            name: name,
            id: id,
            position: position,
            velocity: [0.0, 0.0],
            sprite: sprite,
            grounded: false,
            jump_cleared: false,
            horizontal_move: false,
            direction: true,
            gravity_multiplier: 1.0,
        }
    }

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
            self.sprite.rect[i].tex_coords = texture_coords[i];
        }
    }

    pub fn take_input(&mut self, input: MovementInput) {
        match input {
            MovementInput::UpPress => {
                if self.jump_cleared && self.grounded {
                    self.velocity[1] = 0.005;
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
}

impl Collidable for Entity {
    fn get_velocity(&self) -> [f32; 2] {
        return self.velocity;
    }
    fn set_velocity(&mut self, velocity: [f32; 2]) {
        self.velocity = velocity;
    }
    fn get_position(&self) -> [f32; 2] {
        return self.position
    }
    fn get_grounded(&self) -> bool {
        return self.grounded;
    }
    fn set_grounded(&mut self, is_grounded: bool) {
        self.grounded = is_grounded;
    }
    fn set_pos(&mut self, pos: [f32; 2]) {
        self.position = pos;
    }
    fn get_sprite(&self) -> &Sprite {
        return &self.sprite;
    }
    fn get_horizontal_move(&self) -> bool {
        return self.horizontal_move;
    }
    fn update_pos(&mut self, position: [f32;2]) {
        // println!("Update position: {:?}", position);
        let default = [
            [          0.0,                          0.0           ],
            [          0.0,              self.sprite.screen_size[1]],
            [self.sprite.screen_size[0],             0.0           ],
            [self.sprite.screen_size[0], self.sprite.screen_size[1]],
        ];

        for i in 0..4 {
            for j in 0..2 {
                self.sprite.rect[i].position[j] = default[i][j] + position[j];
            }            
        }
        // self.position = position;
    }

    fn get_name(&self) -> String {
        return self.name.clone();
    }
}