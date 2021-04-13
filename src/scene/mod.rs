pub mod level;
pub mod menu;
pub mod stack;
pub mod error;

use crate::scene::stack::*;
use crate::graphics::context::GraphicsContext;
use crate::scene::error::*;

// pub type Stack = SceneStack<ecs::World>;

pub trait Scene<C> {
    // fn current_scene(&self) -> bool;
    // fn update(&mut self, interface: &mut Interface, world: &mut World) -> SceneSwitch<C>;
    // fn draw(&mut self, world: &mut World, context: &mut GraphicsContext) -> Result<(), SceneError>;
    // fn input(&mut self, gameworld: &mut C, started: bool);
}

