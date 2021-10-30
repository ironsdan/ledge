use std::time::{Duration, SystemTime};
use winit::{
    event_loop::{ControlFlow, EventLoop},
    event::{Event, WindowEvent}
};
use crate::{
    error::*,
    interface::*,
    input:: {keyboard::*, mouse::*},
    physics::*,
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
                },
                WindowEvent::Resized(_) => {
                    interface.graphics_context.recreate_swapchain = true;
                },
                _ => {},
            },
            Event::DeviceEvent { event, .. } => match event {
                _ => (),
            },
            Event::Resumed => {},
            Event::Suspended => {},
            Event::NewEvents(_) => {},
            Event::UserEvent(_) => {},
            Event::LoopDestroyed => {},
            Event::MainEventsCleared => { 
                interface.timer_state.tick();

                if let Err(e) = game_state.update(interface, world) {
                    println!("Error on EventHandler::update(): {:?}", e);
                }

                if let Err(e) = game_state.draw(interface, world) {
                    println!("Error on EventHandler::update(): {:?}", e);
                }
            },
            Event::RedrawRequested(_) => {},
            Event::RedrawEventsCleared => {},
        }
    });
}

pub trait EventHandler {
    fn update(&mut self, interface: &mut Interface, world: &mut World) -> GameResult;
    fn draw(&mut self, interface: &mut Interface, world: &mut World) -> GameResult;

    // fn mouse_button_down_event(&mut self, interface: &mut Interface, button: MouseButton, x: f32, y: f32);
    // fn mouse_button_up_event();
    // fn mouse_motion_event();
    // fn mouse_wheel_event();
}