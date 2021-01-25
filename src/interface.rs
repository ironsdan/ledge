use crate::graphics;
use winit::event_loop::EventLoop;

#[derive(Debug)]
pub enum GameError {
    // TODO Implement.
}

pub type GameResult<T = ()> = Result<T, GameError>;

pub struct InterfaceBuilder {
    // TODO Implement.
}

impl InterfaceBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(&self) -> Result<(Interface, winit::event_loop::EventLoop<()>), GameError> {
        let event_loop = EventLoop::new();
        Ok((Interface::new(&event_loop), event_loop))
    }
}

pub struct Interface {
    pub(crate) graphics_ctx: crate::graphics::context::GraphicsContext,
}

impl Interface {
    pub fn new(event_loop: &winit::event_loop::EventLoop<()>) -> Self {
        Self {
            graphics_ctx: crate::graphics::context::GraphicsContext::new(Some(event_loop)),
        }
    }
}