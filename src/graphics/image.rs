use crate::asset::types::Texture;
use vulkano::sampler::Sampler;

use std::sync::Arc;

#[derive(Clone, Default)]
pub struct Image {
    pub texture: Option<Texture>,
    sampler: Option<Arc<Sampler>>,
    width: u32,
    height: u32,
}

impl Image {
    pub fn new(texture: Texture, sampler: Arc<Sampler>, width: u32, height: u32,) -> Self {
        Self {
            texture: Some(texture),
            sampler: Some(sampler),
            width,
            height,
        }
    }

    pub fn empty() -> Self {
        Self {
            texture: None,
            sampler: None,
            width: 0,
            height: 0,
        }
    }
}