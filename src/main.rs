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
use world::storage::Storage;
use std::marker::PhantomData;
use std::cell::RefCell;
use world::entity::Entities;
use world::Fetch;
use std::collections::hash_map::OccupiedEntry;
use std::collections::hash_map::Entry;

fn main() {
    let mut test_world = World::new();

    let mut test_system = TestSystem{};

    test_world.register::<TestComp>();
    
    // {
        let mut test_comp = test_world.fetch_mut::<TestComp>();
        // (*test_comp).test.push(10);
        // (*test_comp).test.push(20);
        // (*test_comp).test.push(30);
        // (*test_comp).test.push(40);
    // }

    // let test_entities = RefCell::new(Entities {});
    // let test_f_entities = Fetch {
    //     inner: test_entities.borrow(),
    //     phantom: PhantomData
    // };

    // let test_storage_entry = test_world.entry::<TestComp>();
    // let mut test_storage_value = None;
    // match test_storage_entry.inner {
    //     Entry::Occupied(value) => {
    //         test_storage_value = Some(value.get());
    //         let test_storage = Fetch {
    //             inner: test_storage_value.unwrap().borrow(),
    //             phantom: PhantomData
    //         };
        
    //         let read_storage = Storage {
    //             data: test_storage,
    //             entities: test_f_entities,
    //             phantom: PhantomData,
    //         };
        
    //         test_system.run(read_storage);
    //     },
    //     _ => {}
    // }



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

struct TestSystem {

}

impl<'a> System<'a> for TestSystem {
    type SystemData = ReadStorage<'a, TestComp>;

    fn run(&mut self, data0: Self::SystemData) {
        for test in (*data0.data).inner.inner.iter() {
            println!("{:?}", test.test);
        }
    }
}