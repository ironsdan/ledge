use ledge_engine::interface::*;
use ledge_engine::asset::*;
use ledge_engine::graphics::sprite::{SpriteBatch};
use ledge_engine::graphics::Drawable;
use ledge_engine::graphics::BlendMode;
use ledge_engine::graphics::DrawInfo;
use ledge_engine::graphics::Transform;
use ledge_engine::graphics::Rect;
use winit::{
    event_loop::{ControlFlow},
    event::{Event, WindowEvent}
};

fn main() {
    let (mut interface, event_loop) = InterfaceBuilder::new("Example01", "Dan").build().unwrap();

    let texture = types::Texture::from_file_vulkano(include_bytes!("images/small-man-walk-se.png"), &interface.graphics_context);
    let mut sprite_batch = SpriteBatch::new(texture, &interface, BlendMode::Alpha);

    for i in 0..3 {
        for j in 0..3 {
            let mut draw_info = DrawInfo {
                texture_rect: Rect { x: i as f32 * 0.33, y: j as f32 * 0.33, w: 0.33, h: 0.33 },
                color: [0.0, 0.0, 0.0, 1.0],
                transform: Transform::default(),
            };
    
            draw_info.translate(-1.0 + (i as f32 /3.0) , -1.0 + (j as f32 /3.0), 0.0);
            draw_info.scale(0.33);

            sprite_batch.add(draw_info);
        }
    }

    let mut rotate = 0.0;

    event_loop.run(move |event, _, control_flow| {
        let interface = &mut interface;

        interface.process_event(&event);
        
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                },
                WindowEvent::Resized(_) => {
                    interface.graphics_context.recreate_swapchain = true;
                },
                _ => {},
            },
            Event::DeviceEvent { event, .. } => match event {
                _ => (),
            },
            Event::Resumed => {},
            Event::Suspended => {},
            Event::NewEvents(_) => {},
            Event::UserEvent(_) => {},
            Event::LoopDestroyed => {},
            Event::MainEventsCleared => { 
                const DESIRED_FPS: u32 = 60;
                interface.timer_state.tick();

                interface.graphics_context.create_command_buffer();

                while interface.timer_state.check_update_time(DESIRED_FPS) {}
                
                interface.graphics_context.begin_frame();
                render(&mut sprite_batch, &mut rotate);
                sprite_batch.draw(&mut interface.graphics_context);

                interface.graphics_context.present();
            },
            Event::RedrawRequested(_) => {},
            Event::RedrawEventsCleared => {},
        }
        
    });
}

pub fn render(sprite_batch: &mut SpriteBatch, rotate: &mut f32) {
    *rotate += 0.05;
    for sprite in sprite_batch.sprite_data.iter_mut() {
        sprite.rotate_value(*rotate);
    }
}