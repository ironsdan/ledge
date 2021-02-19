pub mod level;
pub mod menu;
pub mod stack;

use crate::scene::stack::*;
use crate::interface::*;
use crate::ecs;
use crate::error::*;

pub type Stack = SceneStack<ecs::World>;

pub trait Scene<C> {
    fn current_scene(&self) -> bool;
    fn update(&mut self, gameworld: &mut C, ctx: &mut Interface) -> SceneSwitch<C>;
    fn draw(&mut self, ctx: &mut Interface) -> GameResult<()>;
    fn input(&mut self, gameworld: &mut C, started: bool);
}

