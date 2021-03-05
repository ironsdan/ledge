use crate::graphics::*;
use crate::asset::handle::Handle;
use crate::asset::types::Texture;
use vulkano::sampler::Sampler;

use std::sync::Arc;

#[derive(Clone, Default)]
pub struct Image {
    pub texture_handle: Handle<Texture>,
    sampler: Option<Arc<Sampler>>,
    blend_mode: Option<BlendMode>,
    width: u32,
    height: u32,
}

impl Image {
    pub fn new(texture_handle: Handle<Texture>, sampler: Arc<Sampler>, blend_mode: BlendMode, width: u32, height: u32,) -> Self {
        Self {
            texture_handle,
            sampler: Some(sampler),
            blend_mode: Some(blend_mode),
            width,
            height,
        }
    }
}