// use std::time::{Duration, SystemTime};
use crate::{error::*, interface::*, graphics};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use crate::graphics::camera::*;
use vulkano::pipeline::Pipeline;
use std::time;

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
                WindowEvent::Resized(size) => {
                    interface.graphics_context.recreate_swapchain = true;
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

                graphics::begin_frame(&mut interface.graphics_context, graphics::Color::black());

                let camera = OrthographicCamera::new(0.001, 1000.0);
                let cam_buf = vulkano::buffer::CpuAccessibleBuffer::from_data(
                    interface.graphics_context.device.clone(), 
                    vulkano::buffer::BufferUsage::uniform_buffer(), 
                    false, 
                    camera.as_mvp(),
                ).unwrap();
            
                let shader = interface.graphics_context.shaders[interface.graphics_context.default_shader].clone();
                let set = vulkano::descriptor_set::PersistentDescriptorSet::new(
                    shader.layout()[0].clone(),
                    [vulkano::descriptor_set::WriteDescriptorSet::buffer(0, cam_buf)],
                ).unwrap();

                interface.graphics_context.command_buffer.as_mut().unwrap().bind_descriptor_sets(
                    vulkano::pipeline::PipelineBindPoint::Graphics,
                    shader.pipeline().layout().clone(),
                    0,
                    set,
                );

                interface.timer_state.tick();

                let upda = time::Instant::now();
                if let Err(e) = game_state.update(interface) {
                    println!("Error on EventHandler::update(): {:?}", e);
                }

                let update_time = 1000.*upda.elapsed().as_secs_f32();

                let draw = time::Instant::now();
                if let Err(e) = game_state.draw(interface) {
                    println!("Error on EventHandler::draw(): {:?}", e);
                }

                let draw_time = 1000.*draw.elapsed().as_secs_f32();

                let pres = time::Instant::now();
                graphics::present(&mut interface.graphics_context);

                let present_time = 1000.*pres.elapsed().as_secs_f32();

                let frame_time = 1000.*start.elapsed().as_secs_f32();

                print!("frame time: {:.2}ms u: {:.2}ms d: {:.2}ms p: {:.2}ms i: {:.2}ms\r", 
                frame_time, update_time, draw_time, present_time, 
                frame_time - update_time - draw_time - present_time);
            }
            Event::RedrawRequested(_) => {}
            Event::RedrawEventsCleared => {}
        }
    });
}

pub trait EventHandler {
    fn update(&mut self, interface: &mut Interface) -> GameResult;
    fn draw(&mut self, interface: &mut Interface) -> GameResult;
    fn resize(&mut self, width: u32, height: u32) -> GameResult;
}
