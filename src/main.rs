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
use asset::*;
use graphics::sprite::Sprite;
use event::*;
use scene::level::*;

fn main() {
    let mut world = World::new();
    let asset_storage = storage::AssetStorage::<types::Texture>::new();
    world.insert(asset_storage);

    let (interface, event_loop) = InterfaceBuilder::new("test", "Dan").build().unwrap();

    // ECS //
    world.register::<Sprite>();
    world.register::<Visible>();
    world.register::<Pos>();
    world.register::<DynamicObject>();

    // Texture Creation //
    let pokeball_texture_handle;
    {
        let texture_test = types::Texture::from_file_vulkano(include_bytes!("images/pokeball.png"), &interface.graphics_ctx);
        let mut texture_assets = world.fetch_mut::<storage::AssetStorage<types::Texture>>();
        pokeball_texture_handle = texture_assets.insert(texture_test);
    }
    let test_sprite = Sprite::new(&interface, &world, "pokeball".to_string(), pokeball_texture_handle.clone(), [0.0, 0.0], [400, 300], [1, 1], None);

    // Entity Creation //
    let pokeball = world.create_entity().with::<Sprite>(test_sprite.clone())
                                      .is::<Visible>()
                                      .is::<DynamicObject>()
                                      .with::<Pos>(Pos {test: (-1.0,-1.0)}).build();
    let pokeball1 = world.create_entity().with::<Sprite>(test_sprite.clone())
                                       .is::<Visible>()
                                       .with::<Pos>(Pos {test: (-1.0,-1.0)}).build();

    // Level Builder //
    let level_space = LevelSpaceBuilder::new().with_entity(pokeball).with_entity(pokeball1).build();

    // Game Creation and Running //
    let mut game = GameState::new();

    game.add_space(Box::new(level_space));

    event::run(interface, world, event_loop, game);
}