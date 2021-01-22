mod lib;
mod entity;
mod sprite;
mod physics;
mod game;

use game::*;

fn main() {
    let mut game = Game::new();
    game.add_sprite("rock".to_string(), [-1.0, -1.0], include_bytes!("rock.png"), [400,300], [2, 2]);
    game.add_sprite("test".to_string(), [0.0, -1.0], include_bytes!("test.png"), [400,300], [2, 2]);
    game.add_sprite("pokeball".to_string(), [-1.0, 0.0], include_bytes!("pokeball.png"), [400,300], [2, 2]);
    game.add_sprite("background".to_string(), [0.0, 0.0], include_bytes!("background.png"), [400,300], [2, 2]);
    game.add_sprite("Dan".to_string(), [0.0, 0.0], include_bytes!("small-man-walk-se.png"), [100,100], [3, 3]);
    game.run();
}