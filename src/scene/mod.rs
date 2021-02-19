pub mod level;
pub mod menu;
pub mod stack;

use crate::scene::stack::*;
use crate::interface::*;
use crate::ecs;
use crate::error::*;
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::command_buffer::pool::standard::StandardCommandPoolBuilder;

pub type Stack = SceneStack<ecs::World>;

pub trait Scene<C> {
    fn current_scene(&self) -> bool;
    fn update(&mut self, gameworld: &mut C, ctx: &mut Interface) -> SceneSwitch<C>;
    fn draw(&mut self, ctx: &mut Interface, builder: &mut AutoCommandBufferBuilder<StandardCommandPoolBuilder>) -> GameResult<()>;
    fn input(&mut self, gameworld: &mut C, started: bool);
}

