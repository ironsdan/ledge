use crate::graphics::*;

pub struct SpriteBatch {
    image: image::Image,
    sprites: Vec<InstanceData>,
    // blend_mode: Option<BlendMode>,
}

impl SpriteBatch {
    pub fn new(image: image::Image) -> Self {
        Self {
            image,
            sprites: Vec::new(),
            // blend_mode: None,
        }
    }

    pub fn insert(&mut self, info: DrawInfo) -> usize {
        self.sprites.push(info.into());
        self.sprites.len()
    }

    pub fn remove(&mut self, idx: usize) {
        self.sprites.remove(idx);
    }

    pub fn clear(&mut self) {
        self.sprites.clear();
    }

    pub fn count(&self) -> usize {
        self.sprites.len()
    }
}

impl Drawable for SpriteBatch {
    fn draw(&self, context: &mut GraphicsContext, _info: DrawInfo) {
        context.draw(Box::new(DefaultPipelineData::new(context.device.clone())
        .vertex_buffer(QUAD_VERTICES.to_vec())
        .instance_buffer(self.sprites.clone())
        .sampled_image(
            0,
            self.image.inner().clone(), 
            context.samplers[0].clone(),
        )));
    }
}
