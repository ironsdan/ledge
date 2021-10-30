use ledge_engine::graphics;
use ledge_engine::conf;
use cgmath::{Deg, Rad, Angle};
use winit::{
    event_loop::{ControlFlow},
    event::{Event, WindowEvent}
};

fn main() {
    let (mut context, event_loop) = graphics::context::GraphicsContext::new(conf::Conf::new("Texture")); // Creating a new context.

    event_loop.run(move |event, _, control_flow| {
        let now = std::time::Instant::now();
        
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => { *control_flow = ControlFlow::Exit; },
                WindowEvent::Resized(_) => { context.recreate_swapchain = true; },
                _ => {},
            },
            Event::MainEventsCleared => { 
                graphics::clear(&mut context, graphics::Color::black());

                // Do some drawing.

                graphics::present(&mut context);

                print!("{:.2}\r", now.elapsed().as_secs_f32() * 1000.0);
            },
            _ => {}
        }
    });
}