mod lib;
// mod entity;
mod sprite;
// mod physics;
mod game;
mod animation;

use game::*;
use animation::*;

fn main() {
    let mut game = Game::new();

    let walking_texture_order = Vec::from(
        [[0, 0], [1, 0], [2, 0],
        [0, 1], [1, 1], [2, 1],
        [0, 2], [1, 2], [2, 2]]
    );

    let standing_texture_order = Vec::from(
        [[0, 0], [1, 0], [2, 0],
        [0, 1], [1, 1], [2, 1],
        [0, 2], [1, 2], [2, 2]]
    );

    let pokeball_order = Vec::from(
        [[0, 0], [1, 0],
        [0, 1], [1, 1]]
    );

    let not_pokeball_order = Vec::from(
        [[0, 0], [1, 0],
        [0, 1], [1, 1]]
    );

    let walking_right = AnimationState::new("WalkingRight".to_string(), 0, walking_texture_order);
    let standing = AnimationState::new("Standing".to_string(), 1, standing_texture_order);
    let test_rule = StateChangeRule::new(standing.clone(), PhysicalInput::D, walking_right.clone());
    let animation_machine = AnimationStateMachine::new(Vec::from([walking_right, standing]), Vec::from([Vec::from([test_rule])]));

    let pokeball = AnimationState::new("WalkingRight".to_string(), 0, pokeball_order);
    let not_pokeball = AnimationState::new("Standing".to_string(), 1, not_pokeball_order);
    let test_rule_two = StateChangeRule::new(pokeball.clone(), PhysicalInput::D, not_pokeball.clone());
    let poke_animation_machine = AnimationStateMachine::new(Vec::from([pokeball, not_pokeball]), Vec::from([Vec::from([test_rule_two])]));


    game.add_sprite("rock".to_string(), [-1.0, -1.0], include_bytes!("rock.png"), [400,300], [2, 2], None);
    game.add_sprite("test".to_string(), [0.0, -1.0], include_bytes!("test.png"), [400,300], [2, 2], None);
    game.add_sprite("pokeball".to_string(), [-1.0, 0.0], include_bytes!("pokeball.png"), [400,300], [2, 2], Some(poke_animation_machine));
    game.add_sprite("background".to_string(), [0.0, 0.0], include_bytes!("background.png"), [400,300], [2, 2], None);
    game.add_sprite("Dan".to_string(), [0.0, 0.0], include_bytes!("small-man-walk-se.png"), [100,100], [3, 3], Some(animation_machine));
    
    
    game.run();
}