// use crate::asset::types::Texture;
// use vulkano::sampler::Sampler;
// use image;
// use crate::graphics::GraphicsContext;
// use crate::graphics::Drawable;
// use crate::graphics::DrawInfo;
use crate::graphics::*;
use std::fs;
use std::io::Cursor;
use std::io::Read;
use std::path;
use std::sync::Arc;
use vulkano::format::Format;
use vulkano::image::{view::ImageView, ImageDimensions, ImmutableImage, MipmapsCount};

#[derive(Clone)]
#[allow(unused)]
pub struct Image {
    inner: Arc<ImageView<ImmutableImage>>,
    width: u32,
    height: u32,
}

impl Image {
    pub fn new<P: AsRef<path::Path>>(ctx: &GraphicsContext, path: P) -> Self {
        let mut png_bytes = Vec::new();

        fs::File::open(path)
            .unwrap()
            .read_to_end(&mut png_bytes)
            .unwrap();

        let cursor = Cursor::new(png_bytes);
        let decoder = png::Decoder::new(cursor);
        let mut reader = decoder.read_info().unwrap();
        let width = reader.info().width;
        let height = reader.info().height;
        let dimensions = ImageDimensions::Dim2d {
            width: width,
            height: height,
            array_layers: 1,
        };
        let mut image_data = Vec::new();
        image_data.resize((width * height * 8) as usize, 0);
        reader.next_frame(&mut image_data).unwrap();

        let (image, _) = ImmutableImage::from_iter(
            image_data.iter().cloned(),
            dimensions,
            MipmapsCount::One,
            Format::R8G8B8A8_UNORM,
            ctx.queue.clone(),
        )
        .unwrap();
        let image_view = ImageView::new_default(image).unwrap();

        Self {
            inner: image_view,
            width,
            height,
        }
    }

    pub fn with_size(ctx: &GraphicsContext, w: usize, h: usize) -> Self {
        Self::with_size_color(ctx, w, h, Color::black())
    }

    pub fn with_size_color(ctx: &GraphicsContext, w: usize, h: usize, color: Color) -> Self {
        let mut v: Vec<u8> = Vec::new();
        for _ in 0..w {
            for _ in 0..h {
                v.append(&mut color.as_u8_vec());
            }
        }

        Self::from_u8(ctx, w as u32, h as u32, v)
    }

    pub fn from_u8(ctx: &GraphicsContext, w: u32, h: u32, v: Vec<u8>) -> Self {
        let dimensions = ImageDimensions::Dim2d {
            width: w,
            height: h,
            array_layers: 1,
        };

        let (image, _) = ImmutableImage::from_iter(
            v.iter().cloned(),
            dimensions,
            MipmapsCount::One,
            Format::R8G8B8A8_UNORM,
            ctx.queue.clone(),
        )
        .unwrap();
        let image_view = ImageView::new_default(image).unwrap();

        Self {
            inner: image_view,
            width: w,
            height: h,
        }
    }

    pub fn from_color(ctx: &GraphicsContext, color: Color) -> Self {
        let image_data: Vec<u8> = color.as_u8_vec();
        let dimensions = ImageDimensions::Dim2d {
            width: 1,
            height: 1,
            array_layers: 1,
        };

        let (image, _) = ImmutableImage::from_iter(
            image_data.iter().cloned(),
            dimensions,
            MipmapsCount::One,
            Format::R8G8B8A8_UNORM,
            ctx.queue.clone(),
        )
        .unwrap();
        let image_view = ImageView::new_default(image).unwrap();

        Self {
            inner: image_view,
            width: 1,
            height: 1,
        }
    }

    pub fn inner(&self) -> &Arc<ImageView<ImmutableImage>> {
        &self.inner
    }
}

impl Drawable for Image {
    fn draw(&self, context: &mut GraphicsContext, info: DrawInfo) {
        // Add quad vertex to pipe data
        context.update_vertex_data(QUAD_VERTICES.to_vec());
        // Update instance data
        context.update_instance_properties(Arc::new(vec![info.into()]));
        // Add texture to pipe data
        context
            .pipe_data
            .sampled_image(0, self.inner.clone(), context.samplers[0].clone());
        // Set blend mode
        context.set_blend_mode(BlendMode::Alpha);
        // call context draw with none
        context.draw();
    }
}
