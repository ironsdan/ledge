use ledge_engine::interface::*;
use ledge_engine::asset::*;
use ledge_engine::physics::*;
use ledge_engine::scene::level::*;
use ledge_engine::graphics::sprite::{SpriteBatch, SpriteId};
use ledge_engine::graphics::BlendMode;
use ledge_engine::graphics::DrawInfo;
use ledge_engine::graphics::Transform;
use ledge_engine::graphics::Rect;
use ledge_engine::game::GameState;
use ledge_engine::ecs::World;
use ledge_engine::event;

use cgmath::Matrix4;

fn main() {
    let mut world = World::new();
    let asset_storage = storage::AssetStorage::<types::Texture>::new();
    world.insert(asset_storage);

    let (mut interface, event_loop) = InterfaceBuilder::new("Example01", "Dan").build().unwrap();

    // ECS //
    world.register::<SpriteBatch>();
    world.register::<DrawInfo>();
    world.register::<SpriteId>();
    world.register::<Visible>();
    world.register::<Position>();
    world.register::<DynamicObject>();
    world.register::<RigidBody>();

    // Texture Creation //
    let texture = types::Texture::from_file_vulkano(include_bytes!("images/small-man-walk-se.png"), &interface.graphics_context);
    let texture_handle = world.fetch_mut::<storage::AssetStorage<types::Texture>>().insert(texture);
    
    let mut sprite_batch = SpriteBatch::new(texture_handle.clone(), 
                                        &world,
                                        &mut interface, 
                                        BlendMode::Default,
                                        512,
                                        512,
                                    );

    let draw_info = DrawInfo::with_rect(Rect { x: 0.33, y: 0.33, w: 0.33, h: 0.33 });

    println!("{:?}", draw_info.transform.as_mat4());

    let sprite_id = sprite_batch.add(draw_info.clone());

    // Entity Creation //
    let entity = world.create_entity().with::<DrawInfo>(draw_info)
                                    .is::<Visible>()
                                    // .is::<DynamicObject>()
                                    // .with::<SpriteId>(sprite_id)
                                    // .with::<RigidBody>(RigidBody { 
                                    //     velocity: (0.0, 0.0), 
                                    //     previous_velocity: (0.0, 0.0), 
                                    //     desired_velocity: (0.0, 0.0), 
                                    //     transition_speed: (20.0, 20.0)
                                    // })
                                    .with::<Position>(Position { 
                                        previous_position: (-1.0,-1.0), 
                                        current_position: (-1.0, -1.0) 
                                    }).build();
    
    // Level Builder //
    let mut level_space = LevelSpaceBuilder::new().with_entity(entity).build();
    level_space.sprite_batch = sprite_batch;

    // Game Creation and Running //
    let mut game = GameState::new();

    game.add_space(Box::new(level_space));

    event::run(interface, world, event_loop, game);
}