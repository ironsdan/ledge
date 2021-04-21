use vulkano::pipeline::blend::BlendFactor;
use vulkano::pipeline::blend::Blend;
use vulkano::pipeline::blend::BlendOp;
use crate::graphics::BlendMode;
use std::sync::Arc;
use crate::graphics::shader::ShaderHandle;
use crate::graphics::Drawable;
use crate::graphics::context::GraphicsContext;
use crate::graphics::buffer::*;
use vulkano::descriptor::descriptor_set::DescriptorSet;

pub trait Material {
    fn alpha_test(test_value: f32);
    fn blend_color_dest(blend_dest: BlendFactor);
    fn blend_alpha_dest(blend_dest: BlendFactor);
    fn blend_color_equation(equation: BlendOp);
    fn blend_alpha_equation(equation: BlendOp);
    fn blending(blend_mode: BlendMode);
    fn blend_color_src(blend_src: BlendFactor);
    fn blend_alpha_src(blend_src: BlendFactor);
}

pub struct ShaderMaterial {
    uniforms: Vec<usize>,
    descriptor: Option<Arc<dyn DescriptorSet>>,
    blend_mode: Blend,
    pub shader_program: Arc<dyn ShaderHandle>
}

impl ShaderMaterial {
    pub fn new(shader_program: Arc<dyn ShaderHandle>) -> Self {
        Self {
            uniforms: Vec::new(),
            descriptor: None,
            blend_mode: Blend::alpha_blending(),
            shader_program,
        }
    }

    // pub fn add_uniform(&mut self, buffer: ) {

    // }

    pub fn set_descriptor(&mut self, descriptor: Arc<dyn DescriptorSet>) {
        self.descriptor = Some(descriptor);
    }
}