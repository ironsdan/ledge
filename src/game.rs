use crate::event::*;
use crate::error::*;
use crate::interface::Interface;

pub struct Game {

}

impl EventHandler for Game {
    fn update(&mut self, interface: &mut Interface) -> GameResult {
        return Ok(());
    }
    fn draw(&self, interface: &mut Interface) -> GameResult {
        return Ok(());
    }
}