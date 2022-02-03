use crate::graphics::*;

pub struct SpriteBatch {
    image: image::Image,
    sprites: Arc<Vec<InstanceData>>,
    // blend_mode: Option<BlendMode>,
}

impl SpriteBatch {
    pub fn new(image: image::Image) -> Self {
        Self {
            image,
            sprites: Arc::new(Vec::new()),
            // blend_mode: None,
        }
    }

    pub fn insert(&mut self, info: DrawInfo) -> usize {
        Arc::get_mut(&mut self.sprites).unwrap().push(info.into());
        self.sprites.len()
    }

    pub fn remove(&mut self, idx: usize) {
        Arc::get_mut(&mut self.sprites).unwrap().remove(idx);
    }

    pub fn clear(&mut self) {
        Arc::get_mut(&mut self.sprites).unwrap().clear();
    }
}

impl Drawable for SpriteBatch {
    fn draw(&self, context: &mut GraphicsContext, _info: DrawInfo) {
        context.update_vertex_data(QUAD_VERTICES.to_vec());

        context.update_instance_properties(self.sprites.clone());
        
        // Add texture to pipe data
        context
            .pipe_data
            .sampled_images
            .insert(0, (self.image.inner().clone(), context.samplers[0].clone()));
        
        // Set blend mode
        context.set_blend_mode(BlendMode::Alpha);

        // call context draw with none
        context.draw();
    }
}
