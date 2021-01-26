mod lib;
mod entity;
mod sprite;
mod event;
mod graphics;
mod animation;
mod input;
mod interface;
mod error;
mod conf;

use event::*;
use interface::*;
use error::*;

fn main() {
    let (ctx, event_loop) = InterfaceBuilder::new("test", "Dan").build().unwrap();

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