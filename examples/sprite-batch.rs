use ledge_engine::conf;
use ledge_engine::graphics;
// use cgmath::{Deg, Rad, Angle};
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};

fn main() {
    let (mut context, event_loop) =
        graphics::context::GraphicsContext::new(conf::Conf::new("Texture")); // Creating a new context.

    let image = graphics::image::Image::new(&context, "examples/images/pokeball.png");
    let mut batch = graphics::sprite::SpriteBatch::new(image);
    let mut params = graphics::DrawInfo::default();
    params.translate(0.5, 0.5, 6.0);
    // params.scale(0.01);
    batch.add(params);
    let mut params = graphics::DrawInfo::default();
    params.translate(-0.5, 0.5, 6.0);
    // params.scale(0.01);
    batch.add(params);
    let mut params = graphics::DrawInfo::default();
    params.translate(0.5, -0.5, 6.0);
    // params.scale(0.01);
    batch.add(params);
    let mut params = graphics::DrawInfo::default();
    params.translate(-0.5, -0.5, 6.0);
    // params.scale(0.01);
    batch.add(params);

    event_loop.run(move |event, _, control_flow| {
        let now = std::time::Instant::now();

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(_) => {
                    context.recreate_swapchain = true;
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                graphics::clear(&mut context, graphics::Color::black());

                graphics::draw(&mut context, &batch, graphics::DrawInfo::default());

                graphics::present(&mut context);

                print!("{:.2}\r", now.elapsed().as_secs_f32() * 1000.0);
            }
            _ => {}
        }
    });
}
