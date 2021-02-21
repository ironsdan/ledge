use crate::event::*;
use crate::error::*;
use crate::interface::Interface;
use crate::scene;
use crate::ecs::World;

pub struct GameState {
    // current_scene: Box<dyn scene::Scene<World>>,
    space_stack: scene::Stack,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            space_stack: scene::Stack::new(),
        }
    }

    pub fn add_space(&mut self, scene: Box<dyn scene::Space<World>>) {
        self.space_stack.push(scene);
    }
}

impl EventHandler for GameState {
    fn update(&mut self, interface: &mut Interface) -> GameResult {
        return Ok(());
    }

    fn draw(&mut self, interface: &mut Interface, world: &mut World) -> GameResult {
        interface.graphics_ctx.begin_frame();

        self.space_stack.draw(world, &mut interface.graphics_ctx);

        interface.graphics_ctx.present();
        return Ok(());
    }
}

