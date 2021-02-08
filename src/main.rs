mod lib;
mod graphics;
mod interface;
mod conf;
mod game;
mod event;
mod error;
mod world;

// use interface::*;
// use game::*;
use world::World;
use world::system::System;
use world::component::Component;
use world::storage::VecStorage;
use world::storage::ReadStorage;
use world::storage::WriteStorage;
use std::any::type_name;
use world::storage::Storage;
use world::storage::TrackedStorage;
use std::marker::PhantomData;
use std::cell::RefCell;
use world::entity::Entities;
use world::Fetch;
use world::Resource;
use std::collections::hash_map::Entry;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let mut test_world = World::new();

    let mut test_system0 = TestSystem1{};
    let mut test_system1 = TestSystem{};

    let test_resource = TestRes::new();

    test_world.insert::<Entities>(Entities {});

    test_world.insert::<u8>(8);

    test_world.register::<TestComp>();
    
    // {
    //     let mut test_comp = test_world.read_comp_storage::<TestComp>();
    //     (*test_comp).inner.inner.push(TestComp { test: (0, 10) });
    //     (*test_comp).inner.inner.push(TestComp { test: (0, 20) });
    //     (*test_comp).inner.inner.push(TestComp { test: (0, 30) });
    //     (*test_comp).inner.inner.push(TestComp { test: (0, 40) });
    // }

    // {
    //     let test_comp = test_world.fetch_comp_storage::<TestComp>();
    //     for i in (*test_comp).inner.inner.iter() {
    //         println!("{:?}", i.test);
    //     }
    // }

    // let test_entities: RefCell<Box<dyn Resource>> = RefCell::new(Box::new(Entities {}));
    // let test_f_entities = Fetch {
    //     inner: test_entities.borrow(),
    //     phantom: PhantomData
    // };

    // 
    // let mut test_storage_value = None;
    // match test_storage_entry.inner {
    //     Entry::Occupied(value) => {
            // test_storage_value = Some(value.get());
            // let test_storage: Fetch<TrackedStorage<TestComp>> = Fetch {
            //     inner: test_storage_value.unwrap().borrow(),
            //     phantom: PhantomData
            // };

            // for i in (*test_storage_entry).inner.inner.iter() {
            //     println!("{:?}", i.test);
            // }
        
            // let read_storage = ReadStorage {
            //     data: test_storage_entry,
            //     entities: test_f_entities,
            //     phantom: PhantomData,
            // };
        {
            let write_storage: WriteStorage<TestComp> = test_world.write_comp_storage::<TestComp>();
            test_system0.run(write_storage);
        }
        {
            let read_storage: ReadStorage<TestComp> = test_world.read_comp_storage::<TestComp>();
            test_system1.run(read_storage);
        }
        // },
    //     _ => {}
    // }

    test_world.remove::<u8>();

    // let (interface, event_loop) = InterfaceBuilder::new("test", "Dan").build().unwrap();

    // let game = Game {
    //     world: Vec::new()
    // };

    // event::run(interface, event_loop, game);
}

pub struct TestComp {
    test: (u8, u8),
}

impl TestComp {
    pub fn test(&self) -> (u8, u8) {
        return self.test;
    }
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

struct TestSystem1 {

}

impl<'a> System<'a> for TestSystem1 {
    type SystemData = WriteStorage<'a, TestComp>;

    fn run(&mut self, mut data0: Self::SystemData) {
        (*data0.data).inner.inner.push(TestComp { test: (0, 10) });
        (*data0.data).inner.inner.push(TestComp { test: (0, 20) });
        (*data0.data).inner.inner.push(TestComp { test: (0, 30) });
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