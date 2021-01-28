use std::collections::HashSet;
use winit::event::ModifiersState;
pub use winit::event::VirtualKeyCode as KeyCode;

pub struct KeyModifierList {}

pub struct KeyboardInterface {
    pressed_keys: HashSet<KeyCode>,
    key_modifiers: KeyModifierList,
    last_pressed: Option<KeyCode>,
    current_pressed: Option<KeyCode>,
}

impl KeyboardInterface {
    pub(crate) fn new() -> Self {
        Self {
            pressed_keys: HashSet::with_capacity(128),
            key_modifiers: KeyModifierList {},
            last_pressed: None,
            current_pressed: None,
        }
    }

    pub(crate) fn set_key(&mut self, key: KeyCode, pressed: bool) {
        if pressed {
            let _ = self.pressed_keys.insert(key);
            self.last_pressed = self.current_pressed;
            self.current_pressed = Some(key);
        } else {
            let _ = self.pressed_keys.remove(&key);
            self.current_pressed = None;
        }

        // self.set_key_modifier(key, pressed);
    }
}