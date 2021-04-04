use std::sync::Arc;
use vulkano::{
    framebuffer::{Subpass},
    pipeline::{GraphicsPipeline, GraphicsPipelineAbstract},
};
use vulkano::pipeline::shader::GraphicsEntryPointAbstract;
use vulkano::pipeline::shader::SpecializationConstants;

use crate::graphics::context::GraphicsContext;
use vulkano::pipeline::vertex::VertexDefinition;
use vulkano::pipeline::shader::EntryPointAbstract;

pub struct Shader<S, C> {
    pub entry_point: S,
    pub specialization_constants: C,
}

impl<S, C> Shader<S, C> {
    pub fn new(entry_point: S, specialization_constants: C) -> Self {
        Self {
            entry_point,
            specialization_constants,
        }
    }
}

// pub trait ShaderAbstract<S, C> {
//     fn entry_point(&self) -> S;
//     fn specialization_constants(&self) -> C;
// }

// This structure is to store multiple pipelines for different blend modes.
// pub struct PipelineObjectSet {
//     pipelines: HashMap<BlendMode, PipelineObject>,
// }

// impl PipelineObjectSet {
//     pub fn new(cap: usize) -> Self {
//         Self {
//             pipelines: HashMap::with_capacity(cap),
//         }
//     }

//     pub fn insert(&mut self, blend_mode: BlendMode, pipeline: PipelineObject) {
//         self.pipelines.insert(blend_mode, pipeline);
//     }

//     pub fn get(&self, blend_mode: &BlendMode) -> Option<&PipelineObject> {
//         self.pipelines.get(blend_mode)
//     }
// }

pub struct PipelineObject {}

impl PipelineObject {
    pub fn new<Vd, Vs, Vss, Fs, Fss>(context: &mut GraphicsContext, vertex_type: Vd, vertex_shader: Shader<Vs, Vss>, fragment_shader: Shader<Fs, Fss>) 
    -> Arc<dyn GraphicsPipelineAbstract + Send + Sync> 
    where
        Vd: VertexDefinition<Vs::InputDefinition> + 'static + Sync + Send,
        Vs: GraphicsEntryPointAbstract<SpecializationConstants = Vss>,
        <Vs as EntryPointAbstract>::PipelineLayout: Clone + 'static + Send + Sync,
        Vss: SpecializationConstants, 
        Fs: GraphicsEntryPointAbstract<SpecializationConstants = Fss>,
        <Fs as EntryPointAbstract>::PipelineLayout: Clone + 'static + Send + Sync,
        Fss: SpecializationConstants, 
    {
        Arc::new(
            GraphicsPipeline::start()
                .vertex_input::<Vd>(vertex_type)
                .vertex_shader(vertex_shader.entry_point, vertex_shader.specialization_constants)
                .point_list()
                .viewports_dynamic_scissors_irrelevant(1)
                .fragment_shader(fragment_shader.entry_point, fragment_shader.specialization_constants)
                .blend_alpha_blending()
                .render_pass(Subpass::from(context.render_pass.clone(), 0).unwrap())
                .build(context.device.clone())
                .unwrap()
        ) as Arc<dyn GraphicsPipelineAbstract + Send + Sync>
    }
}