mod lib;
mod entity;
mod sprite;
// mod physics;
mod graphics;
mod animation;
mod input;

use graphics::*;
use animation::*;

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new().build().unwrap();


    // let walking_texture_order = Vec::from(
    //     [[0, 0], [1, 0], [2, 0],
    //     [0, 1], [1, 1], [2, 1],
    //     [0, 2], [1, 2]]
    // );

    // let standing_texture_order = Vec::from(
    //     [[0, 0], [1, 0], [2, 0],
    //     [0, 1], [1, 1], [2, 1],
    //     [0, 2], [1, 2]]
    // );

    // let timings_walking = Vec::from([200.0, 200.0, 200.0, 200.0, 200.0, 200.0, 200.0, 200.0]);
    // let timings_standing = Vec::from([200.0, 200.0, 200.0, 200.0, 200.0, 200.0, 200.0, 200.0]);

    // let walking_right = AnimationState::new("WalkingRight".to_string(), 0, walking_texture_order, timings_walking);
    // let standing = AnimationState::new("Standing".to_string(), 1, standing_texture_order, timings_standing);
    // let test_rule = StateChangeRule::new(standing.clone(), PhysicalInput::D, walking_right.clone());
    // let animation_machine = AnimationStateMachine::new(Vec::from([walking_right, standing]), Vec::from([Vec::from([test_rule])]));

    // ctx.add_sprite("rock".to_string(), [-1.0, -1.0], include_bytes!("rock.png"), [400,300], [2, 2], None);
    // ctx.add_sprite("test".to_string(), [0.0, -1.0], include_bytes!("test.png"), [400,300], [2, 2], None);
    // ctx.add_sprite("pokeball".to_string(), [-1.0, 0.0], include_bytes!("pokeball.png"), [400,300], [2, 2], None);
    // ctx.add_sprite("background".to_string(), [0.0, 0.0], include_bytes!("background.png"), [400,300], [2, 2], None);
    // ctx.add_physics_object("Dan".to_string(), [0.0, 0.0], include_bytes!("small-man-walk-se.png"), [100,100], [3, 3], Some(animation_machine));
    
    // ctx.run();
}