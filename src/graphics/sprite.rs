// use crate::lib::*;
// use std::sync::Arc;
// use crate::graphics::animation::*;
// use crate::interface::Interface;
use crate::graphics::Drawable;
use crate::graphics;
// use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
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
// use crate::ecs::World;

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

        
        graphics_context.command_buffer.unwrap().update_buffer();
    }
}

impl Drawable for SpriteBatch {
    fn draw(&mut self, graphics_context: &mut GraphicsContext) {
        self.flush(graphics_context);


    }
}