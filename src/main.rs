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
use graphics::sprite::Sprite;
use game::GameState;
use ecs::World;

fn main() {
    let mut world = World::new();
    let asset_storage = storage::AssetStorage::<types::Texture>::new();
    world.insert(asset_storage);

    let (interface, event_loop) = InterfaceBuilder::new("test", "Dan").build().unwrap();

    // ECS //
    world.register::<Sprite>();
    world.register::<Visible>();
    world.register::<Position>();
    world.register::<DynamicObject>();
    world.register::<RigidBody>();

    // Texture Creation //
    let pokeball_texture_handle;
    let rock_texture_handle;
    {
        let texture_poke = types::Texture::from_file_vulkano(include_bytes!("images/pokeball.png"), &interface.graphics_context);
        let texture_rock = types::Texture::from_file_vulkano(include_bytes!("images/rock.png"), &interface.graphics_context);

        let mut texture_assets = world.fetch_mut::<storage::AssetStorage<types::Texture>>();
        pokeball_texture_handle = texture_assets.insert(texture_poke);
        rock_texture_handle = texture_assets.insert(texture_rock);
    }
    let poke_sprite = Sprite::new(&interface, &world, 
        "pokeball".to_string(), 
        pokeball_texture_handle.clone(), 
        [0.0, 0.0], 
        [40, 30], 
        [1, 1], 
        None
    );
    let rock_sprite = Sprite::new(&interface, &world, 
        "rock".to_string(), 
        rock_texture_handle.clone(), 
        [0.0, 0.0], 
        [800, 600], 
        [1, 1], 
        None
    );

    // Entity Creation //
    let pokeball1 = world.create_entity().with::<Sprite>(rock_sprite)
                                       .is::<Visible>()
                                       .with::<Position>(Position { previous_position: (-1.0,-1.0), current_position: (-1.0, -1.0) }).build();
    let pokeball = world.create_entity().with::<Sprite>(poke_sprite)
                                        .is::<Visible>()
                                        .is::<DynamicObject>()
                                        .with::<RigidBody>(RigidBody { velocity: (0.0, 0.0), previous_velocity: (0.0, 0.0), desired_velocity: (0.0, 0.0), transition_speed: (50.0, 50.0)})
                                        .with::<Position>(Position { previous_position: (-1.0,-1.0), current_position: (-1.0, -1.0) }).build();

    // Level Builder //
    let level_space = LevelSpaceBuilder::new().with_entity(pokeball).with_entity(pokeball1).build();

    // Game Creation and Running //
    let mut game = GameState::new();

    game.add_space(Box::new(level_space));

    event::run(interface, world, event_loop, game);
}