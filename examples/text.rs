use ledge::conf;
use ledge::graphics;
use ledge::graphics::text::*;
use ledge::graphics::Color;
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};
use std::sync::Arc;
use ledge::graphics::camera::*;
use vulkano::pipeline::Pipeline;


fn main() {
    let (mut ctx, event_loop) =
        graphics::context::GraphicsContext::new(conf::Conf::new("Text")); // Creating a new context.

    let image = graphics::image::Image::new(&ctx, "examples/images/font.png");
    let font = Arc::new(Font::new(image, 13, 6));

    let mut dom = DocumentContext::new();

    let hello_world = Box::new(
        Text::with_font(font.clone())
            .text("HELLO\nWORLD".to_string())
    );

    let foo_bar = Box::new(
        Text::with_font(font.clone())
            .text("FOO\nBAR".to_string())
    );

    dom.insert("root", "container", Box::new(Div::new()));
    dom.insert("container", "text", hello_world);
    dom.insert("root", "text2", foo_bar);

    dom.select_mut("container").style.position = (-1.0, -1.0, 0.0);
    dom.select_mut("container").style.background_color = Color::grey();
    dom.select_mut("container").style.width = 2.0;

    dom.select_mut("text").style.font_size = 104;
    dom.select_mut("text").style.letter_spacing = 100;
    dom.select_mut("text").style.line_height = 16;

    event_loop.run(move |event, _, control_flow| {
        let now = std::time::Instant::now();

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(_) => {
                    ctx.recreate_swapchain();
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                graphics::begin_frame(&mut ctx, graphics::Color::black());

                let camera = OrthographicCamera::new(0.001, 1000.0);
                let cam_buf = vulkano::buffer::CpuAccessibleBuffer::from_data(
                    ctx.device.clone(), 
                    vulkano::buffer::BufferUsage::uniform_buffer(), 
                    false, 
                    camera.as_mvp(),
                ).unwrap();
            
                let shader = ctx.shaders[ctx.default_shader].clone();
                let set = vulkano::descriptor_set::PersistentDescriptorSet::new(
                    shader.layout()[0].clone(),
                    &[vulkano::descriptor_set::WriteDescriptorSet::buffer(0, cam_buf)],
                ).unwrap();

                ctx.command_buffer.as_mut().unwrap().bind_descriptor_sets(
                    vulkano::pipeline::PipelineBindPoint::Graphics,
                    shader.pipeline().layout().clone(),
                    0,
                    set,
                );

                graphics::draw(&mut ctx, &dom, graphics::DrawInfo::default());

                graphics::present(&mut ctx);

                print!("{:.2}\r", now.elapsed().as_secs_f32() * 1000.0);
            }
            _ => {}
        }
    });
}