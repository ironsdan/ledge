// use crate::entity::*;

// pub struct InputHelper {
//     player: Entity
// }

// impl InputHelper {
//     pub fn new(player: Entity) -> Self {
//         Self{player}
//     }
//     pub fn execute_input(&self, mut input: winit_input_helper::WinitInputHelper, event: &winit::event::Event<()>) {
//         if input.update(&event) {
//             let key_w_released = input.key_released(winit::event::VirtualKeyCode::W);
//             let key_w_pressed = input.key_pressed(winit::event::VirtualKeyCode::W);
//             let key_a = input.key_held(winit::event::VirtualKeyCode::A);
//             let key_d = input.key_held(winit::event::VirtualKeyCode::D);
//         }
//     }
// }

// pub enum MovementInput {
//     UpPress,
//     UpRelease,
//     DownPress,
//     DownRelease,
//     Left,
//     Right
// }