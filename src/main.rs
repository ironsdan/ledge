mod lib;
// mod entity;
mod sprite;
// mod physics;
mod graphics;
mod animation;
mod interface;
mod conf;
mod game;
mod event;
mod error;
mod world;
mod system;
mod component;

use interface::*;
use game::*;

fn main() {
    let (interface, event_loop) = InterfaceBuilder::new("test", "Dan").build().unwrap();

    let game = Game {
        world: Vec::new()
    };

    event::run(interface, event_loop, game);
}