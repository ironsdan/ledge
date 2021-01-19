mod lib;
mod entity;
mod sprite;
mod physics;
mod game;

use game::*;

fn main() {
    let game = Game::initialize_vulkan();
    game.run();
}