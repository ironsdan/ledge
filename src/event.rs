// use std::time::{Duration, SystemTime};
use crate::{interface::*};
use std::time;
use std::thread;
use vulkano::sync::GpuFuture;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use anyhow::Result;

pub fn run<S: 'static>(mut interface: Interface, event_loop: EventLoop<()>) -> !
where
    S: EventHandler,
{
    let mut game_state = S::start(&mut interface);

    event_loop.run(move |event, _, control_flow| {
        let interface = &mut interface;

        interface.process_event(&event);

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(size) => {
                    interface.renderer.recreate_swapchain = true;
                    game_state.resize(size.width, size.height).unwrap();
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
                let start = time::Instant::now();

                // 

                let upda = time::Instant::now();
                if let Err(e) = game_state.update(interface) {
                    println!("Error on EventHandler::update(): {:?}", e);
                }

                let update_time = 1000. * upda.elapsed().as_secs_f32();

                let draw = time::Instant::now();

                let mut future = interface.renderer.begin_frame().unwrap();

                future = game_state.draw(interface, future).unwrap();

                interface.renderer.end_frame(future);

                let draw_time = 1000. * draw.elapsed().as_secs_f32();

                if start.elapsed().as_secs_f32() < 0.016 {
                    let diff = 0.016 - start.elapsed().as_secs_f32();
                    thread::sleep(time::Duration::from_secs_f32(diff));
                }

                let frame_time = 1000. * start.elapsed().as_secs_f32();

                print!(
                    "frame time: {:.2}ms u: {:.2}ms d: {:.2}ms i: {:.2}ms\r",
                    frame_time,
                    update_time,
                    draw_time,
                    frame_time - update_time - draw_time
                );
            }
            Event::RedrawRequested(_) => {}
            Event::RedrawEventsCleared => {}
        }
    });
}

pub trait EventHandler {
    fn start(interface: &mut Interface) -> Self;
    fn update(&mut self, interface: &mut Interface) -> Result<()>;
    fn draw(&mut self, interface: &mut Interface, future: Box<dyn GpuFuture>) -> Result<Box<dyn GpuFuture>>;
    fn resize(&mut self, width: u32, height: u32) -> Result<()>;
}
