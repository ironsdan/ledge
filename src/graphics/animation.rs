use std::collections::HashMap;
use crate::graphics::Drawable;
use crate::graphics::DrawInfo;
use crate::graphics::GraphicsContext;
use crate::graphics::image::Image;

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
    pub type_map: HashMap<T, (usize, usize)>,
    pub default: T,
    pub frame: usize,
}

// Wrapper around drawinfo that implements Into<DrawInfo> but keeps track of frames.
// Possibly add a trait that has the DrawInfo functions so this can be a drop in replacement for drawinfo
// into spritebatch etc.
#[derive(Clone)]
pub struct Animation<T> {
    width: usize,
    height: usize,
    inner: DrawInfo, // Selects the frame.
    type_map: HashMap<T, (usize, usize)>, // Animation type T to (start frame, frame count)
    state: T,
    frame: usize,
}

impl<T: std::cmp::Eq + std::hash::Hash> Animation<T> {
    pub fn new(x: usize, y:usize, def: AnimationDef<T>) -> Self {
        let mut info = DrawInfo::default();
        info.translate(0., 0., 6.);

        info.tex_rect.w = 1. / x as f32;
        info.tex_rect.h = 1. / y as f32;

        Self {
            width: x,
            height: y,
            inner: info,
            type_map: def.type_map,
            state: def.default,
            frame: def.frame,
        }
    }

    pub fn tick(&mut self) {
        if self.frame >= self.state_info().0 + self.state_info().1 {
            self.frame = self.state_info().0;
        } else {
            self.frame+=1;
        }
        
        self.update();
    }

    pub fn set_state(&mut self, s: T) {
        self.state = s;
        self.frame = self.state_info().0;
        self.update();
    }

    fn update(&mut self) {
        self.inner.tex_rect.x = self.inner.tex_rect.w * (self.frame % self.width) as f32;
        self.inner.tex_rect.y = self.inner.tex_rect.h * (self.frame / self.width) as f32;
    }

    pub fn state_info(&self) -> (usize, usize) {
        *self.type_map.get(&self.state).unwrap()
    }

    pub fn inner(&self) -> DrawInfo {
        self.inner.clone()
    }
}

impl<T> Into<DrawInfo> for Animation<T> {
    fn into(self) -> DrawInfo {
        self.inner
    }
}