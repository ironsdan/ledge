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

fn main() {
    let mut test_world = World::new();

    test_world.insert(8);

    test_world.register::<TestComp>();
    
    {
        let mut test_comp = test_world.fetch_mut::<TestComp>();
        (*test_comp).test.push((0, 10));
        (*test_comp).test.push((0, 20));
        (*test_comp).test.push((0, 30));
        (*test_comp).test.push((0, 40));
    }

    {
        let test_comp = test_world.fetch::<TestComp>();
        for i in (*test_comp).test.iter() {
            println!("{:?}", i);
        }
    }

    // let (interface, event_loop) = InterfaceBuilder::new("test", "Dan").build().unwrap();

    // let game = Game {
    //     world: Vec::new()
    // };

    // event::run(interface, event_loop, game);
}

pub struct TestComp {
    test: Vec<(u8, u8)>,
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