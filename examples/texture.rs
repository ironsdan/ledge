use ledge_engine::conf;
use ledge_engine::graphics;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};

fn main() {
    let (mut context, event_loop) =
        graphics::context::GraphicsContext::new(conf::Conf::new("Texture")); // Creating a new context.

    let image = graphics::image::Image::new(&context, "examples/images/pokeball.png");
    let mut params = graphics::DrawInfo::default();
    
    // params.translate(0., 0., 10.);
    // params.dest((8.-8.)/8., (8.-8.)/8., 0.);
    params.scale(1./16.);
    params.dest(-15./16., -15./16., 0.);
    // params.rotate(0., 0., 20.);
    // params.nonuniform_scale(1.,2.,1.);
    
    println!("{:?}", params);

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
                    context.recreate_swapchain = true;
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                graphics::begin_frame(&mut context, graphics::Color::grey());

                graphics::draw(&mut context, &image, params.clone());

                graphics::present(&mut context);

                print!("{:.2}\r", now.elapsed().as_secs_f32() * 1000.0);
            }
            _ => {}
        }
    });
}
