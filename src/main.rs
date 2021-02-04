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
use world::*;

fn main() {
    let test_resource = TestRes::new();
    let mut test_world = World::new();


    test_world.insert(test_resource);
    // let (interface, event_loop) = InterfaceBuilder::new("test", "Dan").build().unwrap();

    // let game = Game {
    //     world: Vec::new()
    // };

    // event::run(interface, event_loop, game);
}

pub struct TestRes {
    test: u8
}

impl TestRes {
    pub fn new() -> Self {
        Self {
            test: 0,
        }
    }
}

impl Resource for TestRes {}