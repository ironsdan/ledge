use crate::conf::*;
use crate::error::*;

#[allow(unused)]
pub struct InterfaceBuilder {
    pub(crate) game_name: String,
    pub(crate) author: String,
    pub(crate) configuration: Conf,
}

impl InterfaceBuilder {
    pub fn new(game_name: &str, author: &str) -> Self {
        let configuration = Conf::new(game_name);
        Self {
            game_name: game_name.to_string(),
            author: author.to_string(),
            configuration: configuration,
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
    pub graphics_context: crate::graphics::context::GraphicsContext,
    pub keyboard_context: crate::input::keyboard::KeyboardContext,
    pub mouse_context: crate::input::mouse::MouseContext,
    pub timer_state: crate::timer::TimerState,
}

impl Interface {
    pub fn from_conf(instance_conf: Conf) -> GameResult<(Self, winit::event_loop::EventLoop<()>)> {
        let (graphics_context, event_loop) =
            crate::graphics::context::GraphicsContext::new(instance_conf);
        let interface_ctx = Interface {
            graphics_context,
            keyboard_context: crate::input::keyboard::KeyboardContext::new(),
            mouse_context: crate::input::mouse::MouseContext::new(),
            timer_state: crate::timer::TimerState::new(),
        };

        Ok((interface_ctx, event_loop))
    }

    pub fn process_event(&mut self, event: &winit::event::Event<()>) {
        match event {
            // Window events.
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::Resized(_) => {
                    self.graphics_context.recreate_swapchain = true;
                }
                winit::event::WindowEvent::CursorMoved { position, .. } => {
                    self.mouse_context
                        .set_last_position((position.x / 400.0 - 1.0, position.y / 300.0 - 1.0));
                }
                winit::event::WindowEvent::MouseInput { state, button, .. } => {
                    let button = match button {
                        winit::event::MouseButton::Left => crate::input::mouse::MouseButton::Left,
                        winit::event::MouseButton::Right => crate::input::mouse::MouseButton::Right,
                        winit::event::MouseButton::Middle => crate::input::mouse::MouseButton::Middle,
                        winit::event::MouseButton::Other(val) => crate::input::mouse::MouseButton::Misc(*val),
                    };

                    let pressed = match state {
                        winit::event::ElementState::Pressed => true,
                        winit::event::ElementState::Released => false,
                    };

                    self.mouse_context.set_button(button, pressed);
                }
                winit::event::WindowEvent::KeyboardInput {
                    input:
                        winit::event::KeyboardInput {
                            state,
                            virtual_keycode: Some(keycode),
                            ..
                        },
                    ..
                } => {
                    let pressed = match state {
                        winit::event::ElementState::Pressed => true,
                        winit::event::ElementState::Released => false,
                    };
                    self.keyboard_context.set_key(*keycode, pressed);
                }
                _ => {}
            },
            // Others.
            _ => {}
        }
    }
}
