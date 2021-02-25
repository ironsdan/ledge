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
mod input;
mod physics;
mod timer;

use interface::*;
use asset::*;
use physics::*;
use scene::level::*;
use graphics::sprite::SpriteBatch;
use game::GameState;
use ecs::World;

fn main() {
    let mut world = World::new();
    let asset_storage = storage::AssetStorage::<types::Texture>::new();
    world.insert(asset_storage);

    let (interface, event_loop) = InterfaceBuilder::new("test", "Dan").build().unwrap();

    // ECS //
    world.register::<SpriteBatch>();
    world.register::<Visible>();
    world.register::<Position>();
    world.register::<DynamicObject>();
    world.register::<RigidBody>();

    // Texture Creation //
    let sweater_texture_handle;
    let rock_texture_handle;
    {
        let texture_sweater = types::Texture::from_file_vulkano(include_bytes!("images/SweaterGuy.png"), &interface.graphics_context);
        let texture_rock = types::Texture::from_file_vulkano(include_bytes!("images/pokeball.png"), &interface.graphics_context);

        let mut texture_assets = world.fetch_mut::<storage::AssetStorage<types::Texture>>();
        sweater_texture_handle = texture_assets.insert(texture_sweater);
        rock_texture_handle = texture_assets.insert(texture_rock);
    }
    let sweat_sprite = SpriteBatch::new(sweater_texture_handle.clone());
    let rock_sprite = SpriteBatch::new(rock_texture_handle.clone());

    // Entity Creation //
    let pokeball1 = world.create_entity().with::<SpriteBatch>(rock_sprite)
                                       .is::<Visible>()
                                       .with::<Position>(Position { previous_position: (-1.0,-1.0), current_position: (-1.0, -1.0) }).build();
    let pokeball = world.create_entity().with::<SpriteBatch>(sweat_sprite.clone())
                                        .is::<Visible>()
                                        .is::<DynamicObject>()
                                        .with::<RigidBody>(RigidBody { velocity: (0.0, 0.0), previous_velocity: (0.0, 0.0), desired_velocity: (0.0, 0.0), transition_speed: (20.0, 20.0)})
                                        .with::<Position>(Position { previous_position: (0.0, 0.0), current_position: (0.0, 0.0) }).build();
    // Level Builder //
    let level_space = LevelSpaceBuilder::new().with_entity(pokeball).with_entity(pokeball1).build();

    // Game Creation and Running //
    let mut game = GameState::new();

    game.add_space(Box::new(level_space));

    event::run(interface, world, event_loop, game);
}