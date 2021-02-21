pub mod level;
pub mod menu;
pub mod stack;

use crate::scene::stack::*;
use crate::graphics::context::GraphicsContext;
use crate::interface::Interface;
use crate::ecs::World;
use crate::ecs;
use crate::error::*;

pub type Stack = SceneStack<ecs::World>;

pub trait Scene<C> {
    fn current_scene(&self) -> bool;
    fn setup(&mut self, interface: &mut Interface, world: &mut World);
    fn update(&mut self, context: &mut GraphicsContext) -> SceneSwitch<C>;
    fn draw(&mut self, world: &mut World, context: &mut GraphicsContext) -> GameResult<()>;
    fn input(&mut self, gameworld: &mut C, started: bool);
}

