use crate::interface::*;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::event::{Event, WindowEvent};
use crate::error::*;
use crate::sprite::*;
use std::time::{Duration, SystemTime};
use std::thread::sleep;

pub fn run<S: 'static>(mut interface: Interface, event_loop: EventLoop<()>, mut game_state: S) -> !
where
    S: EventHandler,
{

    let dan = interface.graphics_ctx.create_sprite("Dan".to_string(), [0.0, 0.0], include_bytes!("images/small-man-walk-se.png"), [50, 100], [3, 3], None);
    let rock = interface.graphics_ctx.create_sprite("rock".to_string(), [-1.0, -1.0], include_bytes!("images/rock.png"), [400, 300], [1, 1], None);
    let pokeball = interface.graphics_ctx.create_sprite("pokeball".to_string(), [0.0, 0.0], include_bytes!("images/pokeball.png"), [400, 300], [1, 1], None);
    let background = interface.graphics_ctx.create_sprite("background".to_string(), [0.0, -1.0], include_bytes!("images/background.png"), [400, 300], [1, 1], None);
    let test = interface.graphics_ctx.create_sprite("test".to_string(), [-1.0, 0.0], include_bytes!("images/test.png"), [400, 300], [1, 1], None);
    
    game_state.update_world(rock);
    game_state.update_world(pokeball);
    game_state.update_world(background);
    game_state.update_world(test);
    game_state.update_world(dan);

    event_loop.run(move |event, _, control_flow| {
        let now = SystemTime::now();
        let interface = &mut interface;

        interface.process_event(&event);
        
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
                interface.graphics_ctx.recreate_swapchain = true;
            }
            Event::MainEventsCleared => {
                // if let Err(e) = game_state.update(interface) {
                //     println!("Error on EventHandler::update(): {:?}", e);
                // }
            }
            Event::RedrawRequested(_) => {
                if let Err(e) = game_state.draw(interface) {
                    println!("Error on EventHandler::update(): {:?}", e);
                }
                sleep(Duration::from_millis(16 - now.elapsed().unwrap().as_secs_f64() as u64));
            }
            Event::RedrawEventsCleared => {
                // print!("Cleared: ");
            }
            _ => {
                // print!("Other: ");
            },
        }
        
        println!("{:?}", now.elapsed().unwrap());
    });
}

pub trait EventHandler {
    fn update_world(&mut self, sprite: Sprite);
    fn update(&mut self, interface: &mut Interface) -> GameResult;
    fn draw(&self, interface: &mut Interface) -> GameResult;

    // fn mouse_button_down_event(&mut self, interface: &mut Interface, button: MouseButton, x: f32, y: f32);
    // fn mouse_button_up_event();
    // fn mouse_motion_event();
    // fn mouse_wheel_event();
}