use crate::event::*;
use crate::error::*;
use crate::interface::Interface;
use crate::scene;
use crate::ecs;

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

    fn draw(&mut self, interface: &mut Interface) -> GameResult {
        interface.graphics_ctx.begin_frame();

        self.scene_stack.scenes[0].draw(interface).unwrap();

        interface.graphics_ctx.present();
        return Ok(());
    }
}