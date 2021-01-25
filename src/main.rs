mod lib;
mod entity;
mod sprite;
mod event;
mod graphics;
mod animation;
mod input;
mod interface;

use event::*;
use interface::*;

fn main() {
    let (ctx, event_loop) = InterfaceBuilder::new().build().unwrap();

    let game = Game {};

    event::run(ctx, event_loop, game);
}

struct Game {

}

impl EventHandler for Game {
    fn update() -> GameResult {
        return Ok(());
    }
    fn draw() -> GameResult {
        return Ok(());
    }
}