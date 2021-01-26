use crate::interface::*;
use crate::graphics::context::*;
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;
use winit::event::{Event, WindowEvent};
use vulkano::image::{Dimensions, ImmutableImage};
use vulkano::format::Format;
use vulkano::sync::GpuFuture;
use crate::error::*;

pub fn run<S: 'static>(mut interface: Interface, event_loop: EventLoop<()>, mut game_state: S) -> !
where
    S: EventHandler,
{
    event_loop.run(move |event, _, control_flow| {

        // Execute Input processing for the context.

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
            }
            Event::MainEventsCleared => {
                if let Err(e) = game_state.update(interface) {
                    println!("Error on EventHandler::update(): {:?}", e);
                }
            }
            Event::RedrawRequested(_) => {
                if let Err(e) = game_state.draw(interface) {
                    println!("Error on EventHandler::update(): {:?}", e);
                }
            }
            Event::RedrawEventsCleared => {
                // print!("Cleared: ");
            }
            _ => {
                // print!("Other: ");
            },
        }
        // println!("{:?}", start_entire.elapsed().unwrap());
    });
}

pub trait EventHandler {
    fn update(&mut self, interface: &mut Interface) -> GameResult;
    fn draw(&self, interface: &mut Interface) -> GameResult;
}