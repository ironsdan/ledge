use crate::graphics::*;

pub struct SpriteBatch {
    image: image::Image,
    sprites: Vec<DrawInfo>,
    // blend_mode: Option<BlendMode>,
}

impl SpriteBatch {
    pub fn new(image: image::Image) -> Self {
        Self {
            image,
            sprites: vec![],
            // blend_mode: None,
        }
    }

    pub fn add(&mut self, info: DrawInfo) -> usize {
        self.sprites.push(info);
        self.sprites.len()
        // SpriteIdx(self.sprites.len() - 1)
    }
}

impl Drawable for SpriteBatch {
    fn draw(&self, context: &mut GraphicsContext, _info: DrawInfo) {
        context.update_vertex_data(vec![
            Vertex {
                pos: [-0.5, -0.5, 0.0],
                uv: [0.0, 0.0],
                vert_color: [1.0, 1.0, 1.0, 1.0],
            },
            Vertex {
                pos: [-0.5, 0.5, 0.0],
                uv: [0.0, 1.0],
                vert_color: [1.0, 1.0, 1.0, 1.0],
            },
            Vertex {
                pos: [0.5, -0.5, 0.0],
                uv: [1.0, 0.0],
                vert_color: [1.0, 1.0, 1.0, 1.0],
            },
            Vertex {
                pos: [0.5, 0.5, 0.0],
                uv: [1.0, 1.0],
                vert_color: [1.0, 1.0, 1.0, 1.0],
            },
        ]);
        // Update instance data
        let mut instance_data = Vec::new();
        for info in self.sprites.clone() {
            instance_data.push(info.into_instance_data());
        }

        context.update_instance_properties(Arc::new(instance_data));
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
