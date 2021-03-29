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
    
    let sprite_batch = SpriteBatch::new(texture_handle.clone(), 
                                        &world,
                                        &mut interface, 
                                        BlendMode::Default,
                                        512,
                                        512,
                                    );

    interface.graphics_context.register_batch(texture_handle.id.clone(), sprite_batch);

    let mut level_space_builder = LevelSpaceBuilder::new();

    for i in 0..3 {
        for j in 0..3 {
            let mut draw_info = DrawInfo {
                texture_handle: texture_handle.clone(),
                texture_rect: Rect { x: i as f32 * 0.33, y: j as f32 * 0.33, w: 0.33, h: 0.33 },
                color: [0.0, 0.0, 0.0, 1.0],
                transform: Transform::default(),
            };

            // println!("{:?}", draw_info.texture_rect);
    
            draw_info.translate(-1.0 + (i as f32 /3.0) , -1.0 + (j as f32 /3.0), 0.0);
            draw_info.scale(0.33);
            let entity = world.create_entity().with::<DrawInfo>(draw_info)
                                                .is::<Visible>()
                                                .with::<Position>(Position { 
                                                    previous_position: (-1.0,-1.0), 
                                                    current_position: (-1.0, -1.0) 
                                                }).build();
            level_space_builder = level_space_builder.with_entity(entity);
        }
    }

    let level_space = level_space_builder.build();

    // Game Creation and Running //
    let mut game = GameState::new();

    game.add_space(Box::new(level_space));

    event::run(interface, world, event_loop, game);
}