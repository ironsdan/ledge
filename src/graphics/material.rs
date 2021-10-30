use vulkano::pipeline::blend::BlendFactor;
use vulkano::pipeline::blend::Blend;
use vulkano::pipeline::blend::BlendOp;
use crate::graphics::BlendMode;
use std::sync::Arc;
use crate::graphics::shader::ShaderHandle;
use vulkano::descriptor_set::DescriptorSet;
use vulkano::buffer::BufferAccess;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::device::Device;

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

#[allow(dead_code)]
pub struct ShaderMaterial {
    pub uniforms: Vec<Arc<dyn BufferAccess + Send + Sync>>,
    descriptor: Option<Arc<dyn DescriptorSet>>,
    blend_mode: Blend,
    pub program: Arc<dyn ShaderHandle>
}

impl ShaderMaterial {
    pub fn new(shader_program: Arc<dyn ShaderHandle>) -> Self {
        Self {
            uniforms: Vec::new(),
            descriptor: None,
            blend_mode: Blend::alpha_blending(),
            program: shader_program,
        }
    }

    pub fn add_uniform<T>(&mut self, data: T, device: Arc<Device>) where
    T: 'static + Copy + Send + Sync,
    {
        let buffer = CpuAccessibleBuffer::from_data(
            device.clone(), 
            BufferUsage::all(), 
            false,
            data,
        ).unwrap();
        self.uniforms.push(buffer.clone());
    }

    pub fn set_descriptor(&mut self, descriptor: Arc<dyn DescriptorSet>) {
        self.descriptor = Some(descriptor);
    }
}