use crate::interface::*;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::event::{Event, WindowEvent};
use image::ImageFormat;
use vulkano::image::{Dimensions, ImmutableImage};
use vulkano::format::Format;
use crate::error::*;
use crate::sprite::Sprite;
use crate::graphics::DrawSettings;
use vulkano::sync::GpuFuture;

pub fn run<S: 'static>(mut interface: Interface, event_loop: EventLoop<()>, mut game_state: S) -> !
where
    S: EventHandler,
{
    event_loop.run(move |event, _, control_flow| {

        // Execute Input processing for the context.

        let interface = &mut interface;

        interface.process_event(&event);

        let (_, empty_future) = {
            ImmutableImage::from_iter(
                [0, 0, 0, 0].iter().cloned(),
                Dimensions::Dim2d { width: 1, height: 1 },
                Format::R8G8B8A8Srgb,
                interface.graphics_interface.queue.clone()
            ).unwrap()
        };

        let mut previous_frame_end = Some(empty_future.boxed());
        
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
                // if let Err(e) = game_state.update(interface) {
                //     println!("Error on EventHandler::update(): {:?}", e);
                // }
            }
            Event::RedrawRequested(_) => {
                // if let Err(e) = game_state.draw(interface, &mut previous_frame_end) {
                //     println!("Error on EventHandler::update(): {:?}", e);
                // }
                interface.graphics_interface.present(&mut previous_frame_end);
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
    fn draw(&self, interface: &mut Interface, previous_frame_end: &mut std::option::Option<std::boxed::Box<dyn vulkano::sync::GpuFuture>>) -> GameResult;
}