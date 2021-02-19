mod lib;
mod graphics;
mod interface;
mod conf;
mod game;
mod event;
mod error;
mod ecs;
mod scene;
mod asset;

use interface::*;
use game::*;
use ecs::World;
use error::*;
use asset::*;
use scene::*;
use scene::stack::*;
use graphics::{ Drawable, DrawSettings};
use graphics::sprite::Sprite;
use vulkano::format::Format;
use vulkano::image::{Dimensions, ImmutableImage};
use image::ImageFormat;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::pool::standard::StandardCommandPoolBuilder;

fn main() {
    // let mut world = World::new();
    let (mut interface, event_loop) = InterfaceBuilder::new("test", "Dan").build().unwrap();

    let asset_storage = storage::AssetStorage::<types::Texture>::new();

    interface.resources.insert(asset_storage);

    let file_bytes = include_bytes!("images/pokeball.png");

    let mut texture_assets = interface.resources.fetch_mut::<storage::AssetStorage<types::Texture>>();

    let (texture, _) = {
        let image = image::load_from_memory_with_format(file_bytes,
            ImageFormat::Png).unwrap().to_rgba8();
        let dimensions = image.dimensions();
        let image_data = image.into_raw().clone();

        ImmutableImage::from_iter(
            image_data.iter().cloned(),
            Dimensions::Dim2d { width: dimensions.0, height: dimensions.1 },
            Format::R8G8B8A8Srgb,
            interface.graphics_ctx.queue.clone(),
        )
        .unwrap()
    };

    let texture_test = types::Texture::new(texture);
    let pokeball_texture_handle = texture_assets.insert(texture_test);

    let test_sprite = Sprite::new("pokeball".to_string(), pokeball_texture_handle.clone(), [0.0, 0.0], [400, 300], [1, 1], None);
    // let mut test_scene = TestScene {
    //     elements: Vec::new(),
    // };

    // test_scene.elements.push(Box::new(test_sprite));

    // let mut game = GameState::new();

    // game.add_scene(Box::new(test_scene));

    // event::run(interface, event_loop, game);
}

pub struct TestScene {
    pub elements: Vec<Box<dyn Drawable>>
}

impl Scene<World> for TestScene {
    fn update(&mut self, gameworld: &mut World, ctx: &mut Interface) -> SceneSwitch<World> {
        SceneSwitch::None
    }
    fn draw(&mut self, gameworld: &mut World, ctx: &mut Interface, builder: &mut AutoCommandBufferBuilder<StandardCommandPoolBuilder>) -> GameResult<()> {
        for element in self.elements.iter_mut() {
            element.draw(ctx, DrawSettings {}, builder);
        }
        Ok(())
    }
    fn input(&mut self, gameworld: &mut World, started: bool) {

    }

    fn current_scene(&self) -> bool {
        true
    }
}

// #[derive(Default)]
// pub struct Vel {
//     test: (u8, u8),
// }

// impl Vel {
//     pub fn test(&self) -> (u8, u8) {
//         return self.test;
//     }
// }

// impl Component for Vel {
//     type Storage = VecStorage<Self>;
// }

// #[derive(Default)]
// pub struct Pos {
//     test: (u8, u8),
// }

// impl Pos {
//     pub fn test(&self) -> (u8, u8) {
//         return self.test;
//     }
// }

// impl Component for Pos {
//     type Storage = VecStorage<Self>;
// }

// #[derive(Default)]
// pub struct Acc {
//     test: (u8, u8),
// }

// impl Acc {
//     pub fn test(&self) -> (u8, u8) {
//         return self.test;
//     }
// }

// impl Component for Acc {
//     type Storage = VecStorage<Self>;
// }

// #[derive(Debug)]
// pub struct TestRes {
//     pub test: u8
// }

// impl TestRes {
//     pub fn new() -> Self {
//         Self {
//             test: 0,
//         }
//     }
// }

// struct VelWrite {}

// impl<'a> System<'a> for VelWrite {
//     type SystemData = WriteStorage<'a, Vel>;

//     fn run(&mut self, mut vel: Self::SystemData) {
//         for vel in (&mut vel).join() {
//             vel.test = (10,10);
//         }
//     }
// }

// struct VelRead {}

// impl<'a> System<'a> for VelRead {
//     type SystemData = (&'a ReadStorage<'a, Vel>, &'a ReadStorage<'a, Pos>, &'a ReadStorage<'a, Acc>);

//     fn run(&mut self, data: Self::SystemData) {
//         println!("Began running VelRead");
//         for (vel, pos, acc) in data.join() {
//             println!("{:?} {:?} {:?}", vel.test, pos.test, acc.test);
//         }
//     }
// }