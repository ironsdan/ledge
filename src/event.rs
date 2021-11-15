// use std::time::{Duration, SystemTime};
use crate::{error::*, interface::*, graphics};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

pub fn run<S: 'static>(mut interface: Interface, event_loop: EventLoop<()>, mut game_state: S) -> !
where
    S: EventHandler,
{
    event_loop.run(move |event, _, control_flow| {
        let interface = &mut interface;

        interface.process_event(&event);

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(_) => {
                    interface.graphics_context.recreate_swapchain = true;
                }
                _ => {}
            },
            Event::DeviceEvent { .. } => {}
            Event::Resumed => {}
            Event::Suspended => {}
            Event::NewEvents(_) => {}
            Event::UserEvent(_) => {}
            Event::LoopDestroyed => {}
            Event::MainEventsCleared => {
                graphics::clear(&mut interface.graphics_context, graphics::Color::grey());

                interface.timer_state.tick();

                if let Err(e) = game_state.update(interface) {
                    println!("Error on EventHandler::update(): {:?}", e);
                }

                if let Err(e) = game_state.draw(interface) {
                    println!("Error on EventHandler::draw(): {:?}", e);
                }

                graphics::present(&mut interface.graphics_context);
            }
            Event::RedrawRequested(_) => {}
            Event::RedrawEventsCleared => {}
        }
    });
}

pub trait EventHandler {
    fn update(&mut self, interface: &mut Interface) -> GameResult;
    fn draw(&mut self, interface: &mut Interface) -> GameResult;
}
