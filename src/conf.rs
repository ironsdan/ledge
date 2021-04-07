#[allow(unused)]
pub struct WindowMode {
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) maximized: bool,
    fullscreen_type: FullscreenType,
    borderless: bool,
    pub(crate) min_width: f32,
    max_width: f32,
    pub(crate) min_height: f32,
    max_height: f32,
    pub(crate) resizable: bool,
}

impl WindowMode {
    pub fn default() -> Self {
        Self {
            width: 800.0,
            height: 600.0,
            maximized: false,
            fullscreen_type: FullscreenType::Windowed,
            borderless: false,
            min_width: 0.0,
            min_height: 0.0,
            max_width: 0.0,
            max_height: 0.0,
            resizable: true,
        }
    }
}

#[allow(unused)]
pub struct WindowSetup {
    pub(crate) title: String,
    // samples:
    vsync: bool,
    icon: String,
    srgb: bool,
}

impl WindowSetup {
    pub fn default() -> Self {
        Self {
            title: "Empty Title".to_string(),
            vsync: false,
            icon: "".to_string(),
            srgb: true,
        }
    }
}

#[allow(unused)]
enum FullscreenType {
    Windowed,
    TFullScreen,
    WFullScreen,
}

pub struct Conf {
    pub(crate) window_mode: WindowMode,
    pub(crate) window_setup: WindowSetup,
}

impl Conf {
    pub fn new(title: &str) -> Self {
        let mut conf = Self::default();
        conf.window_setup.title = title.to_string();
        conf
    }

    pub fn default() -> Self {
        Self {
            window_mode: WindowMode::default(),
            window_setup: WindowSetup::default(),
        }
    }
}