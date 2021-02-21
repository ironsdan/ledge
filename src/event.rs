use std::time::{Duration, SystemTime};
use std::thread::sleep;
use winit::{
    event_loop::{ControlFlow, EventLoop},
    event::{Event, WindowEvent}
};
use crate::{
    ecs::{
        World,
        system::System,
        component::Component,
        storage::{VecStorage, NullStorage, WriteStorage, ReadStorage},
        join::Joinable,
    },
    graphics::sprite::Sprite,
    error::*,
    interface::*,
    input::keyboard::*,
};

pub fn run<S: 'static>(mut interface: Interface, mut world: World, event_loop: EventLoop<()>, mut game_state: S) -> !
where
    S: EventHandler,
{    
    event_loop.run(move |event, _, control_flow| {
        let now = SystemTime::now();
        let interface = &mut interface;
        let world = &mut world;

        let mut pos_system = PosWrite {};
        let mut sprite_system = SpriteMove {};

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
                if let Err(e) = game_state.update(interface) {
                    println!("Error on EventHandler::update(): {:?}", e);
                }
                sleep(Duration::from_millis(16 - now.elapsed().unwrap().as_secs_f64() as u64));
                // println!("{:?}", 1.0/now.elapsed().unwrap().as_secs_f64());

                try_move_sprite(&interface.keyboard_context, world.write_comp_storage::<Pos>(), world.read_comp_storage::<DynamicObject>());
                // pos_system.run((world.write_comp_storage::<Pos>(), world.read_comp_storage::<DynamicObject>()));
                sprite_system.run((world.write_comp_storage::<Sprite>(), world.read_comp_storage::<Pos>()));
            },
            Event::RedrawRequested(_) => {
                if let Err(e) = game_state.draw(interface, world) {
                    println!("Error on EventHandler::update(): {:?}", e);
                }
                sleep(Duration::from_millis(16 - now.elapsed().unwrap().as_secs_f64() as u64));
                // println!("{:?}", 1.0/now.elapsed().unwrap().as_secs_f64());
            },
            Event::RedrawEventsCleared => {
                sleep(Duration::from_millis(16 - now.elapsed().unwrap().as_secs_f64() as u64));
                // println!("{:?}", 1.0/now.elapsed().unwrap().as_secs_f64());
            },
        }
    });
}

pub trait EventHandler {
    fn update(&mut self, interface: &mut Interface) -> GameResult;
    fn draw(&mut self, interface: &mut Interface, world: &mut World) -> GameResult;

    // fn mouse_button_down_event(&mut self, interface: &mut Interface, button: MouseButton, x: f32, y: f32);
    // fn mouse_button_up_event();
    // fn mouse_motion_event();
    // fn mouse_wheel_event();
}

#[derive(Default)]
pub struct Pos {
    pub test: (f32, f32),
}

impl Component for Pos {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct DynamicObject {}

impl Component for DynamicObject {
    type Storage = NullStorage<Self>;
}

struct PosWrite {}

impl<'a> System<'a> for PosWrite {
    type SystemData = (WriteStorage<'a, Pos>, ReadStorage<'a, DynamicObject>);

    fn run(&mut self, (mut pos, moveable): Self::SystemData) {
        for (pos, _) in (&mut pos, &moveable).join() {
            if pos.test.0 < 0.0 {
                pos.test.0 += 0.01;
            }
            if pos.test.1 < 0.0 {
                pos.test.1 += 0.01;
            }
        }
    }
}

struct SpriteMove {}

impl<'a> System<'a> for SpriteMove {
    type SystemData = (WriteStorage<'a, Sprite>, ReadStorage<'a, Pos>);

    fn run(&mut self, (mut sprite, pos): Self::SystemData) {
        for (sprite, pos) in (&mut sprite, &pos).join() {
            sprite.update_rect([pos.test.0 as f32, pos.test.1 as f32]);
        }
    }
}

pub fn try_move_sprite<'a>(keyboard_context: &KeyboardInterface, mut pos: WriteStorage<'a, Pos>, dynamic: ReadStorage<'a, DynamicObject>) {
    let mut x = 0.0;
    let mut y = 0.0;
    
    let keys = keyboard_context.pressed_keys();

    if keys.contains(&KeyCode::W) {
        y -= 0.01;
    }
    if keys.contains(&KeyCode::A) {
        x -= 0.01;
    }
    if keys.contains(&KeyCode::S) {
        y += 0.01;
    }
    if keys.contains(&KeyCode::D) {
        x += 0.01;
    }
    
    for (pos, dynamic) in (&mut pos, &dynamic).join() {
        pos.test.0 += x;
        pos.test.1 += y;
        // sprite.update_rect([pos.test.0 as f32, pos.test.1 as f32]);
    }
}