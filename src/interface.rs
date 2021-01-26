use crate::error::*;
use crate::conf::*;

pub struct InterfaceBuilder {
    pub(crate) game_name: String,
    pub(crate) author: String,
    pub(crate) configuration: Conf,
}

impl InterfaceBuilder {
    pub fn new(game_name: &str, author: &str) -> Self {
        Self {
            game_name: game_name.to_string(),
            author: author.to_string(),
            configuration: Conf::default(),
        }
    }

    pub fn build(self) -> GameResult<(Interface, winit::event_loop::EventLoop<()>)> {
        Interface::from_conf(self.configuration)
    }

    pub fn window_setup(mut self, setup: WindowSetup) -> Self {
        self.configuration.window_setup = setup;
        self
    }

    pub fn window_mode(mut self, mode: WindowMode) -> Self {
        self.configuration.window_mode = mode;
        self
    }
}

pub struct Interface {
    pub(crate) graphics_ctx: crate::graphics::context::GraphicsContext,
}

impl Interface {
    // pub fn new(event_loop: &winit::event_loop::EventLoop<()>) -> Self {
    //     Self {
    //         graphics_ctx: crate::graphics::context::GraphicsContext::new(Some(event_loop)),
    //     }
    // }

    // pub fn new() -> Self {

    // }

    pub fn from_conf(instance_conf: Conf) -> GameResult<(Self, winit::event_loop::EventLoop<()>)> {
        let event_loop = winit::event_loop::EventLoop::new();
        let interface_ctx = Interface {
            graphics_ctx: crate::graphics::context::GraphicsContext::new(&event_loop, instance_conf),
        };

        Ok((interface_ctx, event_loop))
    }
}