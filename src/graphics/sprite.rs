// use crate::lib::*;
use std::sync::Arc;
// use crate::graphics::animation::*;
// use crate::interface::Interface;
use crate::graphics::Drawable;
use crate::graphics;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
// use vulkano::image::ImmutableImage;
// use vulkano::format::Format;
// use vulkano::descriptor::descriptor_set::PersistentDescriptorSetImg;
// use vulkano::descriptor::descriptor_set::PersistentDescriptorSetSampler;
use crate::asset::handle::Handle;
use crate::asset::handle::HandleId;
use crate::asset::types::Texture;
// use crate::asset::storage::AssetStorage;
use crate::graphics::context::GraphicsContext;
use crate::ecs::component::Component;
use crate::ecs::storage::VecStorage;
use std::marker::PhantomData;
use crate::ecs::World;
use graphics::{Vertex, DrawSettings};

use vulkano::image::ImmutableImage;
use vulkano::format::Format;
use vulkano::image::Dimensions;
use vulkano::image::MipmapsCount;
use crate::graphics::context::*;
use image::ImageFormat;

#[derive(Clone, PartialEq)]
pub struct SpriteBatch {
    pub texture_handle: Handle<Texture>,
    sprite_data: Vec<graphics::DrawSettings>,
}

impl Component for SpriteBatch {
    type Storage = VecStorage<Self>;
}

impl Default for SpriteBatch {
    fn default() -> Self {
        Self {
            texture_handle: Handle { id: HandleId::default(), marker: PhantomData },
            sprite_data: Vec::new(),
        }
    }
}

impl SpriteBatch {
    pub fn new(handle: Handle<Texture>) -> Self {
        Self {
            texture_handle: handle,
            sprite_data: Vec::new(),
        }
    }

    pub fn flush(&mut self, graphics_context: &mut GraphicsContext) {

        let sprite_data = graphics::vs::ty::instance_data {
            transform: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]],
        };
        let instance_buffer = graphics_context.instance_pool.next(sprite_data).unwrap();

        let layout = graphics_context.pipeline.descriptor_set_layout(0).unwrap();

        let (texture, _) = {
            let image = image::load_from_memory_with_format(include_bytes!("../images/SweaterGuy.png"),
                ImageFormat::Png).unwrap().to_rgba8();
            let dimensions = image.dimensions();
            let image_data = image.into_raw().clone();
    
            ImmutableImage::from_iter(
                image_data.iter().cloned(),
                Dimensions::Dim2d { width: dimensions.0, height: dimensions.1 },
                MipmapsCount::One,
                Format::R8G8B8A8Srgb,
                graphics_context.queue.clone(),
            )
            .unwrap()
        };
        
        graphics_context.frame_data.instance_descriptor_set = Some(Arc::new(
            PersistentDescriptorSet::start(layout.clone())
                .add_buffer(instance_buffer).unwrap()
                .add_sampled_image(texture, graphics_context.sampler.clone()).unwrap()
                .build()
                .unwrap(),
        ));

        graphics_context.frame_data.vbuf = Some(graphics_context.vertex_buffer_pool.chunk(vec![
            Vertex {
                a_pos: [0.0, 0.0],
            },
            Vertex {
                a_pos: [0.0, 0.0 + 0.1],
            },
            Vertex {
                a_pos: [0.0 + 0.1, 0.0],
            },
            Vertex {
                a_pos: [0.0 + 0.1, 0.0 + 0.1],
            },
        ]).unwrap());
    }
}

impl Drawable for SpriteBatch {
    fn draw(&mut self, graphics_context: &mut GraphicsContext) {
        self.flush(graphics_context);

        graphics_context.draw();
    }
}