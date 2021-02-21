use crate::event::*;
use crate::error::*;
use crate::interface::Interface;
use crate::scene;
use crate::ecs::World;

pub struct GameState {
    current_scene: Box<dyn scene::Scene<World>>,
    // scene_stack: scene::Stack,
}

impl GameState {
    pub fn new(default_scene: Box<dyn scene::Scene<World>>) -> Self {
        Self {
            current_scene: default_scene,
        }
    }

    // pub fn add_scene(&mut self, scene: Box<dyn scene::Scene<ecs::World>>) {
    //     self.scene_stack.push(scene);
    // }
}

impl EventHandler for GameState {
    fn update(&mut self, interface: &mut Interface) -> GameResult {
        return Ok(());
    }

    fn draw(&mut self, interface: &mut Interface, world: &mut World) -> GameResult {
        interface.graphics_ctx.begin_frame();

        self.current_scene.draw(world, &mut interface.graphics_ctx).unwrap();

        interface.graphics_ctx.present();
        return Ok(());
    }
}

