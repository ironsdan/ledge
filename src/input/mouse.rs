pub struct MouseContext {
    pub(crate) last_position: (f64, f64),
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
        }
    }

    pub fn set_last_position(&mut self, position: (f64, f64)) {
        self.last_position = position;
    }
}

pub enum MouseButtons {
    Middle,
    Right,
    Left,
    Misc(u8),
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
