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
use graphics::sprite::Sprite;
use event::*;
use graphics::*;
use graphics::context::GraphicsContext;
use ecs::component::Component;
use ecs::storage::VecStorage;
use ecs::entity::Entity;
use crate::ecs::storage::WriteStorage;
use crate::ecs::storage::ReadStorage;
use crate::ecs::system::System;
use crate::ecs::join::Joinable;

fn main() {
    let mut world = World::new();
    let asset_storage = storage::AssetStorage::<types::Texture>::new();
    world.insert(asset_storage);

    let (mut interface, event_loop) = InterfaceBuilder::new("test", "Dan").build().unwrap();

    // ECS //
    world.register::<Sprite>();
    world.register::<IsVisible>();
    world.register::<Pos>();
    world.register::<Moveable>();

    let mut test_scene = TestScene { entities: Vec::new() };
    test_scene.setup(&mut interface, &mut world);

    let game = GameState::new(Box::new(test_scene.clone()));

    event::run(interface, world, event_loop, game);
}

#[derive(Default, Clone)]
pub struct TestScene {
    pub entities: Vec<Entity>,
}

impl Scene<World> for TestScene {
    fn setup(&mut self, interface: &mut Interface, world: &mut World) {

        let is_visible = IsVisible {};
        let is_moveable = Moveable {};
        let texture_test = types::Texture::from_file_vulkano(include_bytes!("images/pokeball.png"), &interface.graphics_ctx);
        let texture_test1 = types::Texture::from_file_vulkano(include_bytes!("images/rock.png"), &interface.graphics_ctx);

        let pokeball_texture_handle: handle::Handle<types::Texture>;
        let rock_texture_handle;
    
        {
            let mut texture_assets = world.fetch_mut::<storage::AssetStorage<types::Texture>>();
            pokeball_texture_handle = texture_assets.insert(texture_test);
            rock_texture_handle = texture_assets.insert(texture_test1);
        }
    
        let test_sprite = Sprite::new(&interface, &world, "pokeball".to_string(), pokeball_texture_handle.clone(), [0.0, 0.0], [400, 300], [1, 1], None);
        let test_sprite1 = Sprite::new(&interface, &world, "rock".to_string(), rock_texture_handle.clone(), [0.0, 0.0], [400, 300], [1, 1], None);
    
        let entity = world.create_entity().with::<Sprite>(test_sprite)
                                          .with::<IsVisible>(is_visible.clone())
                                          .with::<Moveable>(is_moveable)
                                          .with::<Pos>(Pos {test: (-1.0,-1.0)}).build();
        self.entities.push(entity);

        let entity = world.create_entity().with::<Sprite>(test_sprite1)
                                          .with::<IsVisible>(is_visible)
                                          .with::<Pos>(Pos {test: (-1.0,-1.0)}).build();

        self.entities.push(entity);
    }

    fn update(&mut self, _context: &mut GraphicsContext) -> SceneSwitch<World> {
        SceneSwitch::None
    }

    fn draw(&mut self, world: &mut World, context: &mut GraphicsContext) -> GameResult<()> {
        let mut sprite_system = SpriteDraw {
            context
        };

        sprite_system.run((world.write_comp_storage::<Sprite>(), world.read_comp_storage::<IsVisible>()));

        Ok(())
    }

    fn input(&mut self, _gameworld: &mut World, _started: bool) {

    }

    fn current_scene(&self) -> bool {
        true
    }
}

#[derive(Default, Clone)]
pub struct IsVisible {}

impl Component for IsVisible {
    type Storage = VecStorage<Self>;
}

struct SpriteDraw<'a> {
    context: &'a mut GraphicsContext,
}

impl<'a> System<'a> for SpriteDraw<'a> {
    type SystemData = (WriteStorage<'a, Sprite>, ReadStorage<'a, IsVisible>);

    fn run(&mut self, (mut sprite, scene): Self::SystemData) {
        for (sprite, _) in (&mut sprite, &scene).join() {
            sprite.draw(&mut self.context);
        }
    }
}