pub mod level;
pub mod menu;
pub mod stack;

use crate::scene::stack::*;
use crate::graphics::context::GraphicsContext;
use crate::interface::Interface;
use crate::ecs::World;
use crate::ecs;
use crate::error::*;

pub type Stack = SpaceStack<ecs::World>;

pub trait Space<C> {
    fn current_scene(&self) -> bool;
    fn update(&mut self, interface: &mut Interface, world: &mut World) -> SpaceSwitch<C>;
    fn draw(&mut self, world: &mut World, context: &mut GraphicsContext) -> GameResult<()>;
    fn input(&mut self, gameworld: &mut C, started: bool);
}

