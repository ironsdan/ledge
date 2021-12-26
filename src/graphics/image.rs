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

        println!("{} {}", dimensions.width(), dimensions.height());

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
            width,
            height,
        }
    }

    pub fn from_color(ctx: &GraphicsContext, color: Color) -> Self {
        let mut image_data: Vec<u8> = Vec::new();
        image_data.append(&mut color.as_u8_vec());
        let dimensions = ImageDimensions::Dim2d {
            width: 1,
            height: 1,
            array_layers: 1,
        };

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
        let x_r = self.width as f32 / 2.;
        let y_r = self.height as f32 / 2.;
        context.update_vertex_data(vec![
            Vertex {
                pos: [-1.0, -1.0, 0.0],
                uv: [0.0, 0.0],
                vert_color: [1.0, 1.0, 1.0, 1.0],
            },
            Vertex {
                pos: [-1.0, 1.0, 0.0],
                uv: [0.0, 1.0],
                vert_color: [1.0, 1.0, 1.0, 1.0],
            },
            Vertex {
                pos: [1.0, -1.0, 0.0],
                uv: [1.0, 0.0],
                vert_color: [1.0, 1.0, 1.0, 1.0],
            },
            Vertex {
                pos: [1.0, 1.0, 0.0],
                uv: [1.0, 1.0],
                vert_color: [1.0, 1.0, 1.0, 1.0],
            },
        ]);
        // Update instance data
        context.update_instance_properties(Arc::new(vec![info.into_instance_data()]));
        // Add texture to pipe data
        context
            .pipe_data
            .sampled_images
            .insert(0, (self.inner.clone(), context.samplers[0].clone()));
        // Set blend mode
        context.set_blend_mode(BlendMode::Alpha);
        // call context draw with none
        context.draw();
    }
}
