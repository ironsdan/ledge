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
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                interface.graphics_ctx.recreate_swapchain = true;
            }
            Event::MainEventsCleared => {
                // if let Err(e) = game_state.update(interface) {
                //     println!("Error on EventHandler::update(): {:?}", e);
                // }
            }
            Event::RedrawRequested(_) => {
                if let Err(e) = game_state.draw(interface) {
                    println!("Error on EventHandler::update(): {:?}", e);
                }
                sleep(Duration::from_millis(16 - now.elapsed().unwrap().as_secs_f64() as u64));
            }
            Event::RedrawEventsCleared => {
                // print!("Cleared: ");
            }
            _ => {
                // print!("Other: ");
            },
        }
        
        println!("{:?}", now.elapsed().unwrap());
    });
}

pub trait EventHandler {
    fn update(&mut self, interface: &mut Interface) -> GameResult;
    fn draw(&self, interface: &mut Interface) -> GameResult;

    // fn mouse_button_down_event(&mut self, interface: &mut Interface, button: MouseButton, x: f32, y: f32);
    // fn mouse_button_up_event();
    // fn mouse_motion_event();
    // fn mouse_wheel_event();
}