struct Input {
}

// player_ref.borrow_mut().horizontal_move = false;
// if input.update(&event) {
//     let key_w_released = input.key_released(winit::event::VirtualKeyCode::W);
//     let key_w_pressed = input.key_pressed(winit::event::VirtualKeyCode::W);
//     let key_a = input.key_held(winit::event::VirtualKeyCode::A);
//     let key_d = input.key_held(winit::event::VirtualKeyCode::D);

//     if key_w_pressed {
//         player_ref.borrow_mut().take_input(MovementInput::UpPress);
//     }
//     if key_w_released {
//         player_ref.borrow_mut().take_input(MovementInput::UpRelease);
//     }
//     if key_a {
//         player_ref.borrow_mut().take_input(MovementInput::Left);
//     }
//     if key_d {
//         player_ref.borrow_mut().take_input(MovementInput::Right);
//     }
// }

pub enum MovementInput {
    UpPress,
    UpRelease,
    DownPress,
    DownRelease,
    Left,
    Right
}