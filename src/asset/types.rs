use vulkano::image::ImmutableImage;
use std::sync::Arc;
use vulkano::format::Format;
use vulkano::image::Dimensions;
use vulkano::image::MipmapsCount;
use crate::graphics::context::*;
use image::ImageFormat;

#[derive(Clone, PartialEq)]
pub struct Texture {
    pub vulkano_texture: Arc<vulkano::image::ImmutableImage<vulkano::format::Format>>,
    pub dimensions: (u32, u32),
}

impl Texture {
    pub fn new(texture: Arc<vulkano::image::ImmutableImage<vulkano::format::Format>>) -> Self {
        Self {
            vulkano_texture: texture.clone(),
            dimensions: (texture.dimensions().width(), texture.dimensions().height()),
        }
    }
    pub fn from_file_vulkano(file_contents: &[u8], context: &GraphicsContext) -> Self {
        let (texture, _) = {
            let image = image::load_from_memory_with_format(file_contents,
                ImageFormat::Png).unwrap().to_rgba8();
            let dimensions = image.dimensions();
            let image_data = image.into_raw().clone();
    
            ImmutableImage::from_iter(
                image_data.iter().cloned(),
                Dimensions::Dim2d { width: dimensions.0, height: dimensions.1 },
                MipmapsCount::One,
                Format::R8G8B8A8Srgb,
                context.queue.clone(),
            )
            .unwrap()
        };

        Self {
            vulkano_texture: texture.clone(),
            dimensions: (texture.dimensions().width(), texture.dimensions().height()),
        }
    }

    pub fn as_raw_vk_texture(&self) -> &Arc<vulkano::image::ImmutableImage<vulkano::format::Format>> {
        &self.vulkano_texture
    }
}