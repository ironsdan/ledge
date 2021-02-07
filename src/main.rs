use std::any::Any;
mod lib;
mod graphics;
mod interface;
mod conf;
mod game;
mod event;
mod error;
mod world;

use interface::*;
use game::*;
use world::*;
use world::system::System;
use world::component::Component;
use world::storage::VecStorage;
use world::storage::ReadStorage;

fn main() {
    let mut test_world = World::new();

    test_world.register::<TestComp>();

    let mut test_comp = test_world.fetch_mut::<TestComp>();

    (*test_comp).test.push(10);

    // test_world.::<TestComp>();

    // let mut test_resource = TestRes::new();
    // test_resource.test = 10;
    // let mut test_resource_2 = TestRes::new();
    // test_resource_2.test = 9;

    // test_world.insert(test_resource);
    // test_world.insert(test_resource_2);

    // let test_fetch = test_world.fetch_mut::<TestRes>();

    // println!("{}", test_fetch.test);
    
    // *test_fetch = TestRes {
    //     test: 1,
    // };

    // let test_fetch = test_world.fetch::<TestRes>();

    // println!("{}", test_fetch.test);
    
    // let (interface, event_loop) = InterfaceBuilder::new("test", "Dan").build().unwrap();

    // let game = Game {
    //     world: Vec::new()
    // };

    // event::run(interface, event_loop, game);
}

pub struct TestComp {
    test: Vec<u8>,
}

impl Component for TestComp {
    type Storage = VecStorage<TestComp>;
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

// struct TestSystem {

// }

// impl<'a> System<'a> for TestSystem {
//     type SystemData = (ReadStorage<'a, TestComp>, ReadStorage<'a, TestComp>);

//     fn run(&mut self, (data0, data1): Self::SystemData) {
//         for test in data0.data.inner.inner.iter() {
            
//         }
//     }
// }