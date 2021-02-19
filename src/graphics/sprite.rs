use crate::lib::*;
use std::sync::Arc;
use crate::graphics::animation::*;
use crate::interface::Interface;
use crate::graphics::Drawable;
use crate::graphics::DrawSettings;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::image::ImmutableImage;
use vulkano::format::Format;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSetImg;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSetSampler;
use crate::asset::handle::Handle;
use crate::asset::types::Texture;
use crate::asset::storage::AssetStorage;
use vulkano::sampler::Sampler;
use vulkano::descriptor::descriptor_set::UnsafeDescriptorSetLayout;

#[derive(Clone, PartialEq)]
pub struct Sprite {
    pub name: String,
    pub texture_handle: Handle<Texture>,
    pub rect: Rect,
    pub size: [u32; 2],
    pub screen_size: [f32; 2],
    pub matrix_dims: [u32; 2],
    pub animation_machine: Option<AnimationStateMachine>,
    pub set: Option<std::sync::Arc<PersistentDescriptorSet<(((), PersistentDescriptorSetImg<std::sync::Arc<ImmutableImage<Format>>>), PersistentDescriptorSetSampler)>>>,
}

impl Sprite {
    pub fn new(name: String, handle: Handle<Texture>, pos: [f32; 2], size: [u32; 2], matrix_dims: [u32; 2], animation_machine: Option<AnimationStateMachine>) -> Self {
        let screen_size = convert_to_screen_space(size, [800, 600]); // Converts the given array from pixels to screen space size (0.0-2.0).

        let texture_coord = [ // Adjust the size of the "viewport" of the sprite to only be one animation frame.
            [           0.0,                          0.0            ],
            [           0.0,              1.0 / matrix_dims[1] as f32],
            [1.0 / matrix_dims[0] as f32,             0.0            ],
            [1.0 / matrix_dims[0] as f32, 1.0 / matrix_dims[1] as f32],
        ];

        let rect = Rect::new(screen_size[0], screen_size[1], pos, texture_coord); // Create the Rect that is associated with the texture for drawing.
        Self {
            name: name,
            texture_handle: handle,
            rect: rect,
            size: size,
            screen_size: screen_size,
            matrix_dims: matrix_dims,
            animation_machine: animation_machine,
            set: None,
        }
    }

    pub fn create_set(&mut self, texture: Arc<vulkano::image::ImmutableImage<vulkano::format::Format>>, sampler: &std::sync::Arc<Sampler>, layout: &std::sync::Arc<UnsafeDescriptorSetLayout>) {
        let set = Arc::new(
            PersistentDescriptorSet::start(layout.clone())
                .add_sampled_image(texture.clone(), sampler.clone())
                .unwrap()
                .build()
                .unwrap(),
        );

        self.set = Some(set);
    }

    pub fn update_animation_frame(&mut self) {
        let animation_machine = self.animation_machine.as_mut();
        match animation_machine {
            Some(machine) => {
                let new_position = machine.current_state.update();
                self.update_animation_position(new_position);
            }
            None => {}
        }
    }

    pub fn update_rect(&mut self, position: [f32; 2]) {
        self.rect.update(position);
    }

    pub fn update_size(&mut self, size: [f32; 2]) {
        self.screen_size = size;
        self.rect.update_size(size);
    }

    pub fn update_animation_position(&mut self, animation_state: [u8;2]) {
        let translation_factor = [1.0 / self.matrix_dims[0] as f32, 1.0 / self.matrix_dims[1] as f32]; // Get the base size of one animation frame.
        let texture_coord = [
            [translation_factor[0]*(animation_state[0] as f32),                         translation_factor[0]*(animation_state[1] as f32)                        ],
            [translation_factor[0]*(animation_state[0] as f32),                         translation_factor[1]*(animation_state[1] as f32) + translation_factor[1]],
            [translation_factor[0]*(animation_state[0] as f32) + translation_factor[1], translation_factor[0]*(animation_state[1] as f32)                        ],
            [translation_factor[0]*(animation_state[0] as f32) + translation_factor[1], translation_factor[1]*(animation_state[1] as f32) + translation_factor[1]],
        ];
        for i in 0..self.rect.vertices.len() {
            self.rect.vertices[i].tex_coords = texture_coord[i];
        }
    }
}

impl Drawable for Sprite {
    fn draw(&mut self, interface: &mut Interface, draw_settings: DrawSettings) {
        let sampler = &interface.graphics_ctx.sampler;
        let layout = interface.graphics_ctx.pipeline.descriptor_set_layout(0).unwrap();
        self.create_set(interface.resources.fetch::<AssetStorage<Texture>>().get(self.texture_handle.clone()).unwrap().vulkano_texture.clone(), sampler, layout);
        interface.graphics_ctx.draw(self);
    }

    fn name(&self) -> &str {
        &self.name
    }
}