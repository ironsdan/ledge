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
use event::*;
use asset::*;
use std::marker::{PhantomData};

fn main() {
    let mut world = World::new();
    let (mut interface, event_loop) = InterfaceBuilder::new("test", "Dan").build().unwrap();

    let asset_storage = storage::AssetStorage::<types::Texture>::new();

    world.insert(asset_storage);

    let mut game = GameState::new();

    // let mut test_scene = TestScene {
    //     elements: Vec::new(),
    // };

    // ECS //
    world.register::<Sprite>();
    world.register::<Pos>();

    let texture_test = types::Texture::from_file_vulkano(include_bytes!("images/pokeball.png"), &interface.graphics_ctx);
    // let texture_test1 = types::Texture::from_file_vulkano(include_bytes!("images/background.png"), &interface.graphics_ctx);
    // let texture_test2 = types::Texture::from_file_vulkano(include_bytes!("images/test.png"), &interface.graphics_ctx);
    // let texture_test3 = types::Texture::from_file_vulkano(include_bytes!("images/rock.png"), &interface.graphics_ctx);

    let mut test_sprite;
    let mut pokeball_texture_handle: handle::Handle<types::Texture>;

    {
        let mut texture_assets = world.fetch_mut::<storage::AssetStorage<types::Texture>>();
        pokeball_texture_handle = texture_assets.insert(texture_test);
        // let test_sprite1 = Sprite::new("back".to_string(), back_texture_handle.clone(), [0.0, -1.0], [400, 300], [1, 1], None);
        // let test_sprite2 = Sprite::new("test".to_string(), test_texture_handle.clone(), [-1.0, 0.0], [400, 300], [1, 1], None);
        // let test_sprite3 = Sprite::new("rock".to_string(), rock_texture_handle.clone(), [-1.0, -1.0], [400, 300], [1, 1], None);
        
        // test_scene.elements.push(Box::new(test_sprite));
        // test_scene.elements.push(Box::new(test_sprite1));
        // test_scene.elements.push(Box::new(test_sprite2));
        // test_scene.elements.push(Box::new(test_sprite3));

        // game.add_scene(Box::new(test_scene));
    }

    test_sprite = Sprite::new(&interface, &world, "pokeball".to_string(), pokeball_texture_handle.clone(), [0.0, 0.0], [400, 300], [1, 1], None);

    world.create_entity().with::<Sprite>(test_sprite).with::<Pos>(Pos {test: (-1.0,-1.0)});

    event::run(interface, world, event_loop, game);
}

// pub struct TestScene {
//     pub elements: Vec<Box<dyn Drawable>>
// }

// impl Scene<World> for TestScene {
//     fn update(&mut self, gameworld: &mut World, ctx: &mut Interface) -> SceneSwitch<World> {
//         SceneSwitch::None
//     }
//     fn draw(&mut self, ctx: &mut Interface) -> GameResult<()> {
//         for element in self.elements.iter_mut() {
//             element.draw(ctx, DrawSettings {});
//         }
//         Ok(())
//     }
//     fn input(&mut self, gameworld: &mut World, started: bool) {

//     }

//     fn current_scene(&self) -> bool {
//         true
//     }
// }

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