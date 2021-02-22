use crate::{
    ecs::{
        component::Component,
        system::System,
        storage::{ 
            NullStorage,
            VecStorage,
            WriteStorage,
            ReadStorage,
        },
        join::Joinable,
    },
    graphics::{
        sprite::Sprite,
    }
};

use std::time::Duration;

#[derive(Default)]
pub struct RigidBody {
    pub(crate) velocity: (f32, f32),
    pub(crate) transition_speed: (f32, f32),
    pub(crate) desired_velocity: (f32, f32),
}

impl Component for RigidBody {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct Position(pub f32, pub f32);

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct DynamicObject {}

impl Component for DynamicObject {
    type Storage = NullStorage<Self>;
}

pub struct GravitySystem {}

impl<'a> System<'a> for GravitySystem {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, RigidBody>);

    fn run(&mut self, (mut pos, rigid_body): Self::SystemData) {
        for (pos, _) in (&mut pos, &rigid_body).join() {
            if pos.1 < 0.9 {
                pos.1 += 0.05;
            }
        }
    }
}

pub struct MovementSystem {}

impl<'a> System<'a> for MovementSystem {
    type SystemData = (WriteStorage<'a, RigidBody>, Duration);

    fn run(&mut self, (mut rigid_body, delta_time): Self::SystemData) {
        for rigid_body in (&mut rigid_body).join() {
            let mut velocity: (f32, f32) = (0.0, 0.0);
            velocity.0 = rigid_body.velocity.0 * 
                    (1.0 - delta_time.as_secs_f32() * rigid_body.transition_speed.0) + 
                    rigid_body.desired_velocity.0 * (delta_time.as_secs_f32() * rigid_body.transition_speed.0);
            
            velocity.1 = rigid_body.velocity.1 * 
                    (1.0 - delta_time.as_secs_f32() * rigid_body.transition_speed.1) + 
                    rigid_body.desired_velocity.1 * (delta_time.as_secs_f32() * rigid_body.transition_speed.1);

            if (velocity.0 < 0.005 && velocity.0 > -0.005) && rigid_body.desired_velocity.0 == 0.0 { velocity.0 = 0.0}
            if (velocity.1 < 0.005 && velocity.1 > -0.005) && rigid_body.desired_velocity.1 == 0.0 { velocity.1 = 0.0}

            // println!("({} * {}) + ({} * {})", rigid_body.velocity.0, (1.0 - delta_time.as_secs_f32() * rigid_body.transition_speed.0), rigid_body.desired_velocity.0, (delta_time.as_secs_f32() * rigid_body.transition_speed.0));

            rigid_body.velocity = velocity;
        }
    }
}

pub struct PositionSystem {}

impl<'a> System<'a> for PositionSystem {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, RigidBody>, Duration);

    fn run(&mut self, (mut pos, rigid_body, delta_time): Self::SystemData) {
        for (pos, rigid_body) in (&mut pos, &rigid_body).join() {
            pos.0 += rigid_body.velocity.0 * delta_time.as_secs_f32(); 
            pos.1 += rigid_body.velocity.1 * delta_time.as_secs_f32(); 
        }
    }
}

pub struct SpriteMove {}

impl<'a> System<'a> for SpriteMove {
    type SystemData = (WriteStorage<'a, Sprite>, ReadStorage<'a, Position>);

    fn run(&mut self, (mut sprite, pos): Self::SystemData) {
        for (sprite, pos) in (&mut sprite, &pos).join() {
            sprite.update_rect([pos.0 as f32, pos.1 as f32]);
        }
    }
}