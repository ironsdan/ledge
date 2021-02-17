mod lib;
mod graphics;
mod interface;
mod conf;
mod game;
mod event;
mod error;
mod ecs;

// use interface::*;
// use game::*;
use ecs::World;
use ecs::system::System;
use ecs::component::Component;
use ecs::storage::VecStorage;
use ecs::storage::ReadStorage;
use ecs::storage::WriteStorage;
use ecs::layeredbitmap::LayeredBitMap;
use ecs::join::*;
// use std::any::type_name;
// use ecs::entity::Entities;

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn main() {
    let mut test_bitset0 = LayeredBitMap::new();
    // let mut test_bitset1 = LayeredBitMap::new();
    // let mut test_bitset2 = LayeredBitMap::new();

    // test_bitset0.insert(0);
    // test_bitset0.insert(2);
    // test_bitset0.insert(5);
    // // test_bitset0.insert(5);
    // // test_bitset0.insert(6);
    // // test_bitset0.insert(255);
    // // test_bitset0.insert(1023);

    // test_bitset1.insert(0);
    // test_bitset1.insert(2);
    // test_bitset1.insert(5);
    // // test_bitset1.insert(255);
    // // test_bitset1.remove(255);
    // // test_bitset1.insert(1023);

    // test_bitset2.insert(0);
    // test_bitset2.insert(2);
    // // test_bitset2.insert(255);
    // // test_bitset2.insert(1023);
    
    // let joined = LayeredBitMap::join_set(&[test_bitset0, test_bitset1, test_bitset2]);

    // println!("{:?}", joined);

    // println!("{:b}", test_bitset.layer0[0]);

    // println!("{}", test_bitset.check(254));

    let mut test_world = World::new();

    let mut test_system0 = VelWrite{};
    let mut test_system1 = VelRead{};

    let test_resource = TestRes::new();

    test_world.insert::<u8>(8);
    test_world.insert::<TestRes>(test_resource);

    test_world.register::<Vel>();
    test_world.register::<Pos>();
    test_world.register::<Acc>();

    test_world.create_entity().with(Vel{test:(0,10)}).with(Pos {test: (10, 100)}).with(Acc {test: (0, 0)}).build();
    test_world.create_entity().with(Vel{test:(10,20)}).with(Pos {test: (10, 100)}).with(Acc {test: (0, 0)}).build();
    test_world.create_entity().with(Vel{test:(10,30)}).build();

    {
        let write_storage: WriteStorage<Vel> = test_world.write_comp_storage::<Vel>();
        test_system0.run(write_storage);
    }
    {
        let read_storage_vel: ReadStorage<Vel> = test_world.read_comp_storage::<Vel>();
        let read_storage_acc: ReadStorage<Acc> = test_world.read_comp_storage::<Acc>();
        let read_storage_pos: ReadStorage<Pos> = test_world.read_comp_storage::<Pos>();
        test_system1.run((&read_storage_vel, &read_storage_pos, &read_storage_acc));
    }

    test_world.remove::<u8>();
    test_world.remove::<TestRes>();

    // let (interface, event_loop) = InterfaceBuilder::new("test", "Dan").build().unwrap();

    // let game = Game {
    //     world: Vec::new()
    // };

    // event::run(interface, event_loop, game);
}

#[derive(Default)]
pub struct Vel {
    test: (u8, u8),
}

impl Vel {
    pub fn test(&self) -> (u8, u8) {
        return self.test;
    }
}

impl Component for Vel {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct Pos {
    test: (u8, u8),
}

impl Pos {
    pub fn test(&self) -> (u8, u8) {
        return self.test;
    }
}

impl Component for Pos {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct Acc {
    test: (u8, u8),
}

impl Acc {
    pub fn test(&self) -> (u8, u8) {
        return self.test;
    }
}

impl Component for Acc {
    type Storage = VecStorage<Self>;
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

struct VelWrite {}

impl<'a> System<'a> for VelWrite {
    type SystemData = WriteStorage<'a, Vel>;

    fn run(&mut self, mut vel: Self::SystemData) {
        for vel in (&mut vel).join() {
            vel.test = (10,10);
        }
    }
}

struct VelRead {}

impl<'a> System<'a> for VelRead {
    type SystemData = (&'a ReadStorage<'a, Vel>, &'a ReadStorage<'a, Pos>, &'a ReadStorage<'a, Acc>);

    fn run(&mut self, data: Self::SystemData) {
        println!("Began running VelRead");
        for (vel, pos, acc) in data.join() {
            println!("{:?} {:?} {:?}", vel.test, pos.test, acc.test);
        }
    }
}