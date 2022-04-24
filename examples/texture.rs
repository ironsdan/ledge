use ledge::conf;
use ledge::graphics;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};
use crate::graphics::camera::*;
use vulkano::pipeline::Pipeline;

fn main() {
    let (mut context, event_loop) =
        graphics::context::GraphicsContext::new(conf::Conf::new("Texture")); // Creating a new context.

    let image = graphics::image::Image::new(&context, "examples/images/pokeball.png");
    let params = graphics::DrawInfo::default();

    println!("{:?}", params.transform);
    println!("{:?}", params.transform.as_mat4());
    
    event_loop.run(move |event, _, control_flow| {
        let now = std::time::Instant::now();

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                }
                | WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(_) => {
                    context.recreate_swapchain();
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                graphics::begin_frame(&mut context, graphics::Color::black());

                let camera = OrthographicCamera::new(0.001, 1000.0);
                let cam_buf = vulkano::buffer::CpuAccessibleBuffer::from_data(
                    context.device.clone(), 
                    vulkano::buffer::BufferUsage::uniform_buffer(), 
                    false, 
                    camera.as_mvp(),
                ).unwrap();
            
                let shader = context.shaders[context.default_shader].clone();
                let set = vulkano::descriptor_set::PersistentDescriptorSet::new(
                    shader.layout()[0].clone(),
                    [vulkano::descriptor_set::WriteDescriptorSet::buffer(0, cam_buf)],
                ).unwrap();

                context.command_buffer.as_mut().unwrap().bind_descriptor_sets(
                    vulkano::pipeline::PipelineBindPoint::Graphics,
                    shader.pipeline().layout().clone(),
                    0,
                    set,
                );

                graphics::draw(&mut context, &image, params.clone());

                graphics::present(&mut context);

                print!("{:.2}\r", now.elapsed().as_secs_f32() * 1000.0);
            }
            _ => {}
        }
    });
}
