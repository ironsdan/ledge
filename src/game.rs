use crate::{
    event::*,
    error::*,
    interface::Interface,
    scene::*,
    ecs::World,
    graphics::vs,
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
    fn update(&mut self, interface: &mut Interface, world: &mut World) -> GameResult {
        const DESIRED_FPS: u32 = 60;

        while interface.timer_state.check_update_time(DESIRED_FPS) {
            self.space_stack.update(interface, world);
        }

        Ok(())
    }

    fn draw(&mut self, interface: &mut Interface, world: &mut World) -> GameResult {
        self.space_stack.draw(world, &mut interface.graphics_context);

        interface.graphics_context.present();
        return Ok(());
    }
}

