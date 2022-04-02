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
                    context.recreate_swapchain = true;
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                graphics::begin_frame(&mut context, graphics::Color::black());

                graphics::draw(&mut context, &image, params.clone());

                graphics::present(&mut context);

                print!("{:.2}\r", now.elapsed().as_secs_f32() * 1000.0);
            }
            _ => {}
        }
    });
}
