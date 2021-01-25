use crate::graphics::*;
use winit::event_loop::EventLoop;

#[derive(Debug)]
pub enum GameError {
    // TODO Implement.
}

pub type GameResult<T = ()> = Result<T, GameError>;

pub struct ContextBuilder {
    // TODO Implement.
}

impl ContextBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(&self) -> Result<(InterfaceContext, winit::event_loop::EventLoop<()>), GameError> {
        let event_loop = EventLoop::new();
        Ok((InterfaceContext::new(Some(&event_loop)), event_loop))
    }
}