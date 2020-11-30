// mod lib;
// use::lib::Rect;
use crate::sprite::*;
use std::rc::Rc;
use std::cell::RefCell;

const ACC: [f32; 2] = [0.0, -0.00003];

pub struct CollisionWorld<T, U> {
    collidables: Vec<Rc<RefCell<T>>>,
    pub entities: Vec<Rc<RefCell<U>>>,
    num_collidables: u16,
}

// fn check_bodies(rect1: Rect, rect2: Rect) -> bool {
//     let mut collision = false;
    
//     if rect1.x < rect2.x + rect2.width() &&
//        rect1.x + rect1.width() > rect2.x &&
//        rect1.y < rect2.y + rect2.height() &&
//        rect1.y + rect1.height() > rect2.y {
//            collision = true;
//     }

//     return collision;
// }

impl<T, U> CollisionWorld<T, U> where T: Collidable, U: Collidable {
    pub fn new() -> Self {
        Self {
            collidables: Vec::new(),
            entities: Vec::new(),
            num_collidables: 0,
        }
    }
    pub fn step(&mut self, timestep: f32) {
        // Steps:
        // Apply gravity.
        // Check collisions.
        for entity_ref in (self.entities).iter_mut() {
            CollisionWorld::<T, U>::apply_gravity(&mut entity_ref.borrow_mut(), timestep);
        }

        // println!("step.");
        // for entity in self.entities.iter_mut() {
        //     for collidable in self.collidables.iter() {
                // if check_bodies(entity.bounding_box, collidable.bounding_box) {
                //     entity.grounded = true;
                //     entity.bounding_box.y = collidable.bounding_box.y - entity.bounding_box.height() + 1.0;
                // } else {
                //     entity.grounded = false;
                // }
        //     }
        // }
    }

    pub fn apply_gravity(entity: &mut U, timestep: f32) where T: Collidable, U: Collidable {
        let mut pos_x = entity.get_position()[0];
        let mut pos_y = entity.get_position()[1];
        let mut vel_x = entity.get_velocity()[0];
        let mut vel_y = entity.get_velocity()[1];

        if vel_x > 0.001 {
            vel_x = 0.001;
        } else if vel_x < -0.001 {
            vel_x = -0.001;
        }

        pos_x = pos_x + (timestep * (vel_x + timestep * ACC[0]/2.0));

        if !entity.get_grounded() {     
            pos_y = pos_y - (timestep * (vel_y + timestep * ACC[1]/2.0));
            vel_y = vel_y + timestep * ACC[1];
            if vel_y < -0.05 {
                vel_y = -0.05;
            }
        }

        // println!("{} {} {} {} {} {} {}", pos_x, pos_y, vel_x, vel_y, entity.get_grounded(), entity.get_horizontal_move(), timestep);
        entity.set_velocity([vel_x, vel_y]);
        entity.update_pos([pos_x, pos_y]);
        entity.set_pos([pos_x, pos_y]);

        if entity.get_position()[1] > 0.7 {
            entity.set_grounded(true);
        }
    
        if !entity.get_horizontal_move() {
            entity.set_velocity([0.0, entity.get_velocity()[1]]);
        }
    }
}

pub trait Collidable {
    fn get_velocity(&self) -> [f32; 2];
    fn set_velocity(&mut self, v: [f32; 2]);
    fn get_position(&self) -> [f32; 2];
    fn get_grounded(&self) -> bool;
    fn set_grounded(&mut self, is_grounded: bool);
    fn set_pos(&mut self, pos: [f32; 2]);
    fn get_sprite(&self) -> &Sprite;
    fn get_horizontal_move(&self) -> bool;
    fn update_pos(&mut self, position: [f32; 2]);
    fn get_name(&self) -> String;
}

// struct Collidable {
//     obj_type: CollidableObj,
//     id: u32,
//     name: String,
//     pub bounding_box: Rect,
//     pub grounded: bool,
// }

// struct BoundingBox {
//     height: u32,
//     width: u32,
//     x: u32,
//     y: u32,
// }

// pub enum CollidableObj {
//     Solid,
//     Platform,
//     Entity,
// }

// pub struct Rect {
//     pub x: f32,
//     pub y: f32,
//     pub height: f32,
//     pub width: f32,
// }

// impl Rect {
//     pub fn width(&self) -> f32 {
//         return self.width;
//     }

//     pub fn height(&self) -> f32 {
//         return self.height;
//     }
// }