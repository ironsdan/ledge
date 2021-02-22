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
    input:: {keyboard::*, mouse::*},
    physics::*,
};

pub fn run<S: 'static>(mut interface: Interface, mut world: World, event_loop: EventLoop<()>, mut game_state: S) -> !
where
    S: EventHandler,
{    
    event_loop.run(move |event, _, control_flow| {
        let now = SystemTime::now();
        let interface = &mut interface;
        let world = &mut world;

        let mut sprite_system = SpriteMove {};
        let mut gravity_system = GravitySystem {};
        let mut movement_system = MovementSystem {};
        let mut position_system = PositionSystem {};

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

                
                movement_system.run((world.write_comp_storage::<RigidBody>(), now.elapsed().unwrap()));
                position_system.run((world.write_comp_storage::<Position>(), world.read_comp_storage::<RigidBody>(), now.elapsed().unwrap()));

                try_move_sprite(&interface.keyboard_context, world.write_comp_storage::<RigidBody>(), world.read_comp_storage::<DynamicObject>());
                // gravity_system.run((world.write_comp_storage::<Position>(), world.read_comp_storage::<RigidBody>()));
                sprite_system.run((world.write_comp_storage::<Sprite>(), world.read_comp_storage::<Position>()));
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



pub fn try_move_sprite<'a>(keyboard_context: &KeyboardContext, mut rigid_body: WriteStorage<'a, RigidBody>, dynamic: ReadStorage<'a, DynamicObject>) {
    let mut x = 0.0;
    let mut y = 0.0;
    
    let keys = keyboard_context.pressed_keys();

    if keys.contains(&KeyCode::W) {
        y -= 2.0;
    }
    if keys.contains(&KeyCode::A) {
        x -= 2.0;
    }
    if keys.contains(&KeyCode::S) {
        y += 2.0;
    }
    if keys.contains(&KeyCode::D) {
        x += 2.0;
    }
    
    for (rigid_body, _) in (&mut rigid_body, &dynamic).join() {
        rigid_body.desired_velocity.0 = x;
        rigid_body.desired_velocity.1 = y;
    }
}

pub fn try_move_sprite_mouse<'a>(mouse_context: &MouseContext, mut pos: WriteStorage<'a, Position>, dynamic: ReadStorage<'a, DynamicObject>) {    
    let mouse_pos = mouse_context.last_position;
    
    for (pos, _) in (&mut pos, &dynamic).join() {
        pos.0 = mouse_pos.0 as f32;
        pos.1 = mouse_pos.1 as f32;
    }
}