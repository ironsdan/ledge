use std::any::Any;
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
    let mut test_world = World::new();

    let mut test_resource = TestRes::new();
    test_resource.test = 10;
    let mut test_resource_2 = TestRes::new();
    test_resource_2.test = 9;

    test_world.insert(test_resource);
    test_world.insert(test_resource_2);

    let test_fetch = test_world.fetch_mut::<TestRes>();

    println!("{}", test_fetch.test);
    
    *test_fetch = TestRes {
        test: 1,
    };

    let test_fetch = test_world.fetch::<TestRes>();

    println!("{}", test_fetch.test);
    
    // let (interface, event_loop) = InterfaceBuilder::new("test", "Dan").build().unwrap();

    // let game = Game {
    //     world: Vec::new()
    // };

    // event::run(interface, event_loop, game);
}

#[derive(Debug)]
pub struct TestRes {
    pub test: u8
}

impl TestRes {
    pub fn new() -> Self {
        Self {
            test: 0,
        }
    }
}

impl Resource for TestRes {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}