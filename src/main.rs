mod lib;
mod sprite;
mod event;
mod graphics;
mod animation;
mod input;
mod interface;
mod error;
mod conf;
mod game;

use game::*;
use event::*;
use interface::*;
use error::*;

fn main() {
    let (ctx, event_loop) = InterfaceBuilder::new("test", "Dan").build().unwrap();

    let game = Game {};

    event::run(ctx, event_loop, game);
}