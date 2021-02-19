use crate::interface::*;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::event::{Event, WindowEvent};
use crate::error::*;
use std::time::{Duration, SystemTime};
use std::thread::sleep;

pub fn run<S: 'static>(mut interface: Interface, event_loop: EventLoop<()>, mut game_state: S) -> !
where
    S: EventHandler,
{    
    event_loop.run(move |event, _, control_flow| {
        let now = SystemTime::now();
        let interface = &mut interface;

        interface.process_event(&event);
        
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                },
                WindowEvent::Resized(_) => {
                    interface.graphics_ctx.recreate_swapchain = true;
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
                if let Err(e) = game_state.update(interface) {
                    println!("Error on EventHandler::update(): {:?}", e);
                }
                sleep(Duration::from_millis(16 - now.elapsed().unwrap().as_secs_f64() as u64));
            },
            Event::RedrawRequested(_) => {
                if let Err(e) = game_state.draw(interface) {
                    println!("Error on EventHandler::update(): {:?}", e);
                }
                sleep(Duration::from_millis(16 - now.elapsed().unwrap().as_secs_f64() as u64));
            },
            Event::RedrawEventsCleared => {
                sleep(Duration::from_millis(16 - now.elapsed().unwrap().as_secs_f64() as u64));
            },
        }
        
        println!("{:?}", now.elapsed().unwrap());
    });
}

pub trait EventHandler {
    fn update(&mut self, interface: &mut Interface) -> GameResult;
    fn draw(&mut self, interface: &mut Interface) -> GameResult;

    // fn mouse_button_down_event(&mut self, interface: &mut Interface, button: MouseButton, x: f32, y: f32);
    // fn mouse_button_up_event();
    // fn mouse_motion_event();
    // fn mouse_wheel_event();
}