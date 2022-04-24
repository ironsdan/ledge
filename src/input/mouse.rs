pub struct MouseContext {
    pub last_position: (f64, f64),
    pub current_pressed: Option<MouseButton>,
}

impl Default for MouseContext {
    fn default() -> Self {
        Self::new()
    }
}

impl MouseContext {
    pub fn new() -> Self {
        Self {
            last_position: (0.1, 0.1),
            current_pressed: None,
        }
    }

    pub fn set_last_position(&mut self, position: (f64, f64)) {
        self.last_position = position;
    }

    pub fn set_button(&mut self, button: MouseButton, pressed: bool) {
        if pressed {
            self.current_pressed = Some(button);
        } else {
            self.current_pressed = None;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MouseButton {
    Middle,
    Right,
    Left,
    Misc(u16),
}

pub enum MouseCursor {
    Default,
    Crosshair,
    Hand,
    Arrow,
    Move,
    Text,
    Wait,
    Help,
    Progress,
    NotAllowed,
    ContextMenu,
    Cell,
    VerticalText,
    Alias,
    Copy,
    NoDrop,
    Grab,
    Grabbing,
    AllScroll,
    ZoomIn,
    ZoomOut,
    EResize,
    NResize,
    NeResize,
    NwResize,
    SResize,
    SeResize,
    SwResize,
    WResize,
    EwResize,
    NsResize,
    NeswResize,
    NwseResize,
    ColResize,
    RowResize,
}
