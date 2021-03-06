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
use crate::asset::storage::AssetStorage;
use crate::asset::types::Texture;
use crate::graphics::context::GraphicsContext;
use crate::ecs::component::Component;
use crate::ecs::storage::VecStorage;
// use std::marker::PhantomData;
use crate::ecs::World;
use graphics::{Vertex, DrawInfo};

// use vulkano::image::ImmutableImage;
// use vulkano::format::Format;
// use vulkano::image::Dimensions;
// use vulkano::image::MipmapsCount;
// use crate::graphics::context::*;
// use image::ImageFormat;
use crate::graphics::InstanceData;
use crate::graphics::image::Image;

#[derive(Clone)]
pub struct SpriteBatch {
    pub image: Image,
    sprite_data: Vec<DrawInfo>,
}

impl Component for SpriteBatch {
    type Storage = VecStorage<Self>;
}

impl Default for SpriteBatch {
    fn default() -> Self {
        Self {
            image: Image::default(),
            sprite_data: Vec::new(),
        }
    }
}

impl SpriteBatch {
    pub fn new(image: Image) -> Self {
        Self {
            image,
            sprite_data: Vec::new(),
        }
    }

    pub fn add(&mut self, draw_info: DrawInfo) {
        self.sprite_data.push(draw_info);
    }

    pub fn load_asset(&self, world: &World, graphics_context: &mut GraphicsContext) {
        let texture_assets = world.fetch::<AssetStorage<Texture>>();
        let texture = texture_assets.get(&self.image.texture_handle).unwrap().as_raw_vk_texture();

        let layout = graphics_context.pipeline.descriptor_set_layout(0).unwrap();
        graphics_context.frame_data.uniform_descriptor_set = Some(Arc::new(
            PersistentDescriptorSet::start(layout.clone())
                .add_buffer(graphics_context.mvp_buffer.clone()).unwrap()
                .add_sampled_image(texture.clone(), graphics_context.sampler.clone()).unwrap()
                .build()
                .unwrap(),
        ));
    }

    pub fn flush(&mut self, graphics_context: &mut GraphicsContext) {
        graphics_context.frame_data.vbuf = Some(graphics_context.vertex_buffer_pool.chunk(vec![
            Vertex {
                a_pos: [0.0, 0.0],
                a_uv: [0.0, 0.0],
                a_vert_color: [0.0, 0.0, 0.0, 1.0],
            },
            Vertex {
                a_pos: [0.0, 1.0],
                a_uv: [0.0, 1.0],
                a_vert_color: [0.0, 0.0, 0.0, 1.0],
            },
            Vertex {
                a_pos: [1.0, 0.0],
                a_uv: [1.0, 0.0],
                a_vert_color: [0.0, 0.0, 0.0, 1.0],
            },
            Vertex {
                a_pos: [1.0, 1.0],
                a_uv: [1.0, 1.0],
                a_vert_color: [0.0, 0.0, 0.0, 1.0],
            },
        ]).unwrap());
        
        let mut instance_data = Vec::new();

        for sprite_data in self.sprite_data.iter() {
            instance_data.push(sprite_data.into_instance_data());
        }

        graphics_context.frame_data.instance_data = Some(graphics_context.instance_buffer_pool.chunk(instance_data).unwrap());
    }
}

impl Drawable for SpriteBatch {
    fn draw(&mut self, graphics_context: &mut GraphicsContext) {
        self.flush(graphics_context);

        graphics_context.draw();
    }
}