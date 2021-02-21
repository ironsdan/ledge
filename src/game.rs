use crate::event::*;
use crate::error::*;
use crate::interface::Interface;
use crate::scene;
use crate::ecs;

use crate::ecs::system::System;
use crate::ecs::component::Component;
use crate::ecs::storage::VecStorage;
use crate::ecs::storage::WriteStorage;
use crate::ecs::storage::ReadStorage;
use crate::ecs::join::Joinable;
use crate::graphics::sprite::Sprite;
use crate::graphics::Drawable;
use crate::graphics::DrawSettings;
use crate::ecs::World;

pub struct GameState {
    scene_stack: scene::Stack,
}

impl GameState {
    pub fn new() -> Self {
        let scene_stack = scene::Stack::new();
        Self {
            scene_stack,
        }
    }

    pub fn add_scene(&mut self, scene: Box<dyn scene::Scene<ecs::World>>) {
        self.scene_stack.push(scene);
    }
}

impl EventHandler for GameState {
    fn update(&mut self, interface: &mut Interface) -> GameResult {
        return Ok(());
    }

    fn draw(&mut self, interface: &mut Interface, world: &World) -> GameResult {
        interface.graphics_ctx.begin_frame();

        // self.scene_stack.scenes[0].draw(interface).unwrap();
        let mut sprite_system = SpriteDraw{
            interface
        };

        sprite_system.run(world.write_comp_storage::<Sprite>());

        interface.graphics_ctx.present();
        return Ok(());
    }
}

struct SpriteDraw<'a> {
    interface: &'a mut Interface,
}

impl<'a> System<'a> for SpriteDraw<'a> {
    type SystemData = WriteStorage<'a, Sprite>;

    fn run(&mut self, mut sprite: Self::SystemData) {
        for sprite in (&mut sprite).join() {
            sprite.draw(self.interface);
        }
    }
}