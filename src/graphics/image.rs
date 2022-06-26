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
use vulkano::descriptor_set::PersistentDescriptorSet;
use vulkano::format::Format;
use vulkano::image::{view::ImageView, ImageDimensions, ImmutableImage, MipmapsCount};
use vulkano::command_buffer::CommandBufferUsage;
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::pipeline::{Pipeline, PipelineBindPoint};

#[derive(Clone)]
#[allow(unused)]
pub struct Image {
    inner: Arc<ImageView<ImmutableImage>>,
    width: u32,
    height: u32,
    sampler: Arc<Sampler>,
}

impl Image {
    pub fn new<P: AsRef<path::Path>>(queue: Arc<Queue>, sampler: Arc<Sampler>, path: P) -> Self {
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
            queue.clone(),
        )
        .unwrap();
        let image_view = ImageView::new_default(image).unwrap();

        Self {
            inner: image_view,
            width,
            height,
            sampler,
        }
    }

    // pub fn with_size(queue: Arc<Queue>, w: usize, h: usize) -> Self {
    //     Self::with_size_color(queue, w, h, Color::black())
    // }

    // pub fn with_size_color(queue: Arc<Queue>, w: usize, h: usize, color: Color) -> Self {
    //     let mut v: Vec<u8> = Vec::new();
    //     for _ in 0..w {
    //         for _ in 0..h {
    //             v.append(&mut color.as_u8_vec());
    //         }
    //     }

    //     Self::from_u8(queue, w as u32, h as u32, v)
    // }

    // pub fn from_u8(queue: Arc<Queue>, w: u32, h: u32, v: Vec<u8>) -> Self {
    //     let dimensions = ImageDimensions::Dim2d {
    //         width: w,
    //         height: h,
    //         array_layers: 1,
    //     };

    //     let (image, _) = ImmutableImage::from_iter(
    //         v.iter().cloned(),
    //         dimensions,
    //         MipmapsCount::One,
    //         Format::R8G8B8A8_UNORM,
    //         queue.clone(),
    //     )
    //     .unwrap();
    //     let image_view = ImageView::new_default(image).unwrap();

    //     Self {
    //         inner: image_view,
    //         width: w,
    //         height: h,
    //         sampler: None,
    //     }
    // }

    // pub fn from_color(queue: Arc<Queue>, color: Color) -> Self {
    //     let image_data: Vec<u8> = color.as_u8_vec();
    //     let dimensions = ImageDimensions::Dim2d {
    //         width: 1,
    //         height: 1,
    //         array_layers: 1,
    //     };

    //     let (image, _) = ImmutableImage::from_iter(
    //         image_data.iter().cloned(),
    //         dimensions,
    //         MipmapsCount::One,
    //         Format::R8G8B8A8_UNORM,
    //         queue.clone(),
    //     )
    //     .unwrap();
    //     let image_view = ImageView::new_default(image).unwrap();

    //     Self {
    //         inner: image_view,
    //         width: 1,
    //         height: 1,
    //         sampler: None,
    //     }
    // }

    pub fn inner(&self) -> &Arc<ImageView<ImmutableImage>> {
        &self.inner
    }
}

impl Drawable for Image {
    fn draw(&self, queue: Arc<Queue>, shader_handle: &Box<dyn ShaderHandle>, info: DrawInfo) -> Result<SecondaryAutoCommandBuffer> {
        let mut builder = AutoCommandBufferBuilder::secondary_graphics(
            queue.device().clone(),
            queue.family(),
            CommandBufferUsage::MultipleSubmit,
            shader_handle.pipeline().subpass().clone(),
        )?;

        let vertex_count = QUAD_VERTICES.len() as u32;
        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            queue.device().clone(),
            BufferUsage::all(),
            false,
            QUAD_VERTICES.to_vec(),
        ).unwrap();

        let instances: Vec<InstanceData> = vec![info.into()];
        let instance_count = instances.len() as u32;
        let instance_buffer = CpuAccessibleBuffer::from_iter(
            queue.device().clone(),
            BufferUsage::all(),
            false,
            instances,
        ).unwrap();

        let layout = shader_handle.layout()[1].clone();

        let set = PersistentDescriptorSet::new(
            layout.clone(),
            [WriteDescriptorSet::image_view_sampler(
                0,
                self.inner.clone(),
                self.sampler.clone(),
            )],
        ).unwrap();

        let layout = shader_handle.layout()[0].clone();

        let mvp_buffer = CpuAccessibleBuffer::from_iter(
            queue.device().clone(),
            BufferUsage::all(),
            false,
            [
                    [1.0,0.0,0.0,0.0],
                    [0.0,1.0,0.0,0.0],
                    [0.0,0.0,1.0,0.0],
                    [0.0,0.0,0.0,1.0],
                ],
        ).unwrap();

        let cam_set = PersistentDescriptorSet::new(
            layout.clone(),
            [WriteDescriptorSet::buffer(
                0,
                mvp_buffer,
            )],
        ).unwrap();

        builder
            .bind_pipeline_graphics(shader_handle.pipeline().clone())
            .set_viewport(0, vec![Viewport {
                origin: [0.0, 0.0],
                dimensions: [800 as f32, 600 as f32],
                depth_range: 0.0..1.0,
            }])
            .bind_vertex_buffers(0, (vertex_buffer, instance_buffer))
            .bind_descriptor_sets(
                PipelineBindPoint::Graphics,
                shader_handle.pipeline().layout().clone(),
                0,
                (cam_set, set),
            )
            .draw(
                vertex_count, 
                instance_count, 
                0, 
                0, 
                )
            .unwrap();
        
        let commands = builder.build()?;

        Ok(commands)
    }
}
