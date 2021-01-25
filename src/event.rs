use crate::interface::*;
use crate::graphics::context::*;
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;
use winit::event::{Event, WindowEvent};
use vulkano::image::{Dimensions, ImmutableImage};
use vulkano::format::Format;
use vulkano::sync::GpuFuture;

pub fn run<S: 'static>(mut ctx: Interface, event_loop: EventLoop<()>, mut state: S) -> !
where
    S: EventHandler,
{
    let (_, tex_future) = {
        ImmutableImage::from_iter(
            [0,0,0,0].to_vec().iter().cloned(),
            Dimensions::Dim2d {width: 1, height: 1},
            Format::R8G8B8A8Srgb,
            ctx.graphics_ctx.vulkan_instance.queue.clone(),
        ).unwrap()
    };

    let mut recreate_swapchain = false;
    let mut previous_frame_end = Some(tex_future.boxed());

    let input = WinitInputHelper::new();

    let mut frame_num = 0;
    event_loop.run(move |event, _, control_flow| {

        // Execute Input processing for the context.
        
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
                recreate_swapchain = true;
            }
            Event::MainEventsCleared => {
                // self.collision_world.step(timestep);
            }
            Event::RedrawRequested(_) => {
                // ctx.draw(&mut previous_frame_end, &mut recreate_swapchain, &mut frame_num);
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
    fn update() -> GameResult;
    fn draw() -> GameResult;
}