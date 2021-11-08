use std::collections::HashMap;
use crate::graphics::DrawInfo;

pub enum Transition {
    Linear,
    EaseIn,
    EaseInOut,
    EaseOut,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    // Bezier,
    // EaseInOut3Point,
}

pub struct AnimationDef<T> {
    type_map: HashMap<T, (usize, usize)>,
    default: T,
}

// Wrapper around drawinfo that implements Into<DrawInfo> but keeps track of frames.
// Possibly add a trait that has the DrawInfo functions so this can be a drop in replacement for drawinfo
// into spritebatch etc.
pub struct Animation<T> {
    width: usize,
    height: usize,
    inner: DrawInfo, // Selects the frame.
    type_map: HashMap<T, (usize, usize)>, // Animation type T to (start frame, frame count)
    state: T,
}

impl<T> Animation<T> {
    pub fn new(w: usize, h: usize, def: AnimationDef<T>) -> Self {
        Self {
            width: w,
            height: h,
            inner: DrawInfo::default(),
            type_map: def.type_map,
            state: def.default,
        }
    }

    pub fn tick(&mut self) {
        
    }

    pub fn set_state(&mut self, s: T) {
        self.state = s
    }

    pub fn get_state(&self) -> &T {
        &self.state
    }
}

impl<T> Into<DrawInfo> for Animation<T> {
    fn into(self) -> DrawInfo {
        self.inner
    }
}