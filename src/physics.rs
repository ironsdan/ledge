// mod lib;
// use::lib::Rect;
use crate::sprite::*;
use crate::lib::Rect;
use std::rc::Rc;
use std::cell::RefCell;

const ACC: [f32; 2] = [0.0, -0.00003];

pub struct CollisionWorld<T, U> {
    pub collidables: Vec<Rc<RefCell<T>>>,
    pub entities: Vec<Rc<RefCell<U>>>,
    // num_collidables: u16,
}

fn check_bodies(rect1: &Rect, rect2: &Rect) -> bool {
    let mut t1 = false;
    let mut t2 = false;
    let mut t3 = false;
    let mut t4 = false;
    
    if rect1.x < rect2.x + rect2.width() {
        t1 = true;
    }
    if   rect1.x + rect1.width() > rect2.x {
        t2 = true;
    }
    if   rect1.y < rect2.y + rect2.height() {
        t3 = true;
    }
    if   rect1.y + rect1.height() > rect2.y {
        t4 = true;
    }

    return t1 & t2 & t3 & t4;
}

impl<T, U> CollisionWorld<T, U> where T: Collidable, U: Collidable {
    pub fn new() -> Self {
        Self {
            collidables: Vec::new(),
            entities: Vec::new(),
            // num_collidables: 0,
        }
    }
    pub fn step(&mut self, timestep: f32) {
        // Steps:
        // Apply gravity.
        // Check collisions.
        for entity_ref in (self.entities).iter_mut() {
            CollisionWorld::<T, U>::apply_gravity(&mut entity_ref.borrow_mut(), timestep);
        }

        for entity_ref in (self.entities).iter_mut() {
            // println!("{}", entity_ref.borrow().get_position()[1]);
            if entity_ref.borrow().get_position()[1] > 0.7 {
                entity_ref.borrow_mut().set_grounded(true);
                let entity_pos = entity_ref.borrow_mut().get_position();
                let pos_y = 0.7;
                entity_ref.borrow_mut().update_position([entity_pos[0], pos_y]);
            }
        }

        // for i in 0..self.entities.len() {
        //     let entity_box = self.entities[i].borrow_mut().get_bounding_box().clone();
        //     for j in 0..self.entities.len() {
        //         let other_entity_box = self.entities[j].borrow_mut().get_bounding_box().clone();
        //         if entity_box != other_entity_box {
        //             if check_bodies(&entity_box, &other_entity_box) {
        //                 // println!("They are colliding");
        //             } else {
        //                 // println!("not colliding");
        //             }
        //         }
        //     }
        // }
        
        // for i in 0..self.entities.len() {
        //     let mut entity = self.entities[i].borrow_mut();
        //     for j in 0..self.collidables.len() {
        //         let platform_box = self.collidables[j].borrow_mut().get_bounding_box().clone();
        //         if *entity.get_bounding_box() != platform_box {
        //             if check_bodies(&entity.get_bounding_box(), &platform_box) {
        //                 println!("They are colliding");
        //                 entity.update_position([platform_box.x, platform_box.y]);
        //             } else {
        //                 println!("not colliding");
        //             }
        //         }
        //     }
        // }
    }

    pub fn apply_gravity(entity: &mut U, timestep: f32) where T: Collidable, U: Collidable {
        // println!("{} {} {}", entity.get_name(), entity.get_sprite().rect.width(), entity.get_sprite().rect.height());
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
            pos_y = pos_y - entity.get_gravity_multiplier() * (timestep * (vel_y + timestep * ACC[1]/2.0));
            vel_y = vel_y + timestep * ACC[1];
            if vel_y < -0.05 {
                vel_y = -0.05;
            }
        }

        entity.set_velocity([vel_x, vel_y]);
        entity.update_position([pos_x, pos_y]);
    
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
    fn get_sprite(&self) -> &Sprite;
    fn get_horizontal_move(&self) -> bool;
    fn update_position(&mut self, position: [f32; 2]);
    fn get_name(&self) -> String;
    fn get_bounding_box(&self) -> &Rect;
    fn get_gravity_multiplier(&self) -> f32;
}