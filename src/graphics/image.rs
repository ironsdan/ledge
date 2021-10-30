// use crate::asset::types::Texture;
// use vulkano::sampler::Sampler;
// use image;
use crate::graphics::GraphicsContext;
use crate::graphics::Drawable;
use crate::graphics::DrawInfo;
use std::io::Read;
use std::io::Cursor;
use std::fs;
use std::path;
use vulkano::image::{
    view::ImageView, ImageDimensions, ImmutableImage, MipmapsCount,
};
use vulkano::format::Format;
use std::sync::Arc;

// #[derive(Clone, Default)]
pub struct Image {
    inner: Arc<ImageView<std::sync::Arc<ImmutableImage>>>,
    width: u32,
    height: u32,
}

impl Image {
    pub fn new<P: AsRef<path::Path>>(ctx: &GraphicsContext, path: P) -> Self {
        let mut png_bytes = Vec::new();
        fs::File::open(path).unwrap().read(&mut png_bytes).unwrap();

        let cursor = Cursor::new(png_bytes);
        let decoder = png::Decoder::new(cursor);
        let mut reader = decoder.read_info().unwrap();
        let info = reader.0;
        let dimensions = ImageDimensions::Dim2d {
            width: info.width,
            height: info.height,
            array_layers: 1,
        };
        let mut image_data = Vec::new();
        image_data.resize((info.width * info.height * 4) as usize, 0);
        reader.1.next_frame(&mut image_data).unwrap();

        let (image, _) = ImmutableImage::from_iter(
            image_data.iter().cloned(),
            dimensions,
            MipmapsCount::One,
            Format::R8G8B8A8_SRGB,
            ctx.queue.clone(),
        )
        .unwrap();
        let image_view = ImageView::new(image).unwrap();
        
        Self {
            inner: image_view,
            width: info.width,
            height: info.height,
        }
    }
}

impl Drawable for Image {
    fn draw(&self, context: &mut GraphicsContext, info: DrawInfo) {
        // context.pipe_data.vert_buf = Arc::new(context.buffer_from(context.quad_vertex_buffer).unwrap());
        // context.pipe_data.blend_mode
        // context.tex
        // draw with none.
    }
}