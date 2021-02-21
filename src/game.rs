use crate::{
    event::*,
    error::*,
    interface::Interface,
    scene::*,
    ecs::World,
};

pub struct GameState {
    space_stack: Stack,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            space_stack: Stack::new(),
        }
    }

    pub fn add_space(&mut self, scene: Box<dyn Space<World>>) {
        self.space_stack.push(scene);
    }
}

impl EventHandler for GameState {
    fn update(&mut self, interface: &mut Interface) -> GameResult {
        self.space_stack.update();
        Ok(())
    }

    fn draw(&mut self, interface: &mut Interface, world: &mut World) -> GameResult {
        interface.graphics_context.begin_frame();

        self.space_stack.draw(world, &mut interface.graphics_context);

        interface.graphics_context.present();
        return Ok(());
    }
}

