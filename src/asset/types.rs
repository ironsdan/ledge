use vulkano::image::ImmutableImage;
use std::sync::Arc;

#[derive(Clone, PartialEq)]
pub struct Texture {
    pub vulkano_texture: Arc<vulkano::image::ImmutableImage<vulkano::format::Format>>,
}

impl Texture {
    pub fn new(texture: Arc<vulkano::image::ImmutableImage<vulkano::format::Format>>) -> Self {
        Self {
            vulkano_texture: texture.clone(),
        }
    }
}