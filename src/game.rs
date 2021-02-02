use crate::event::*;
use crate::error::*;
use crate::interface::Interface;
use crate::sprite::*;

pub struct Game {
    pub world: Vec<Sprite>,
}

impl EventHandler for Game {
    fn update_world(&mut self, sprite: Sprite) {
        self.world.push(sprite);
    }

    fn update(&mut self, interface: &mut Interface) -> GameResult {
        return Ok(());
    }

    fn draw(&self, interface: &mut Interface) -> GameResult {
        let mut builder = interface.graphics_ctx.begin_frame().unwrap();
        for sprite in self.world.iter() {
            interface.graphics_ctx.draw(&mut builder, &sprite);
        }
        interface.graphics_ctx.present(builder);
        return Ok(());
    }
}