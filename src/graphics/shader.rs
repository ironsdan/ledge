use std::sync::Arc;
use std::collections::HashMap;

use vulkano::{
    descriptor_set::{
        layout::DescriptorSetLayout,
        DescriptorSet,
    },
    render_pass::{Subpass},
    pipeline::{
        shader::GraphicsEntryPoint,
        GraphicsPipeline, 
        vertex::VertexDefinition,
        blend::{
            AttachmentBlend,
            AttachmentsBlend,
            Blend,
            LogicOp,
            BlendOp,
            BlendFactor,
        }
    },
    buffer::BufferAccess,
    command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer},
};
use vulkano::pipeline::{PipelineBindPoint};
use vulkano::descriptor_set::persistent::PersistentDescriptorSet;
use vulkano::descriptor_set::layout::DescriptorDescTy;
// use vulkano::descriptor_set::layout::DescriptorSetLayout;

use crate::graphics::{
    context::GraphicsContext,
    PipelineData,
    BlendMode,
    error::*,
};


pub enum VertexOrder {
    LineList,
    LineStrip,
    PointList,
    TriangleFan,
    TriangleList,
    TriangleStrip,
}

pub enum ShaderType {
    Vertex,
    Fragment,
    TessellationControl,
    TessellationEval,
    Geometry,
    Default,
}

pub type ShaderId = usize;

pub struct ShaderProgram {
    pipelines: PipelineObjectSet,
    current_mode: BlendMode,
}

pub trait ShaderHandle {
    fn draw(&self, command_buffer: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>, slice: Arc<dyn BufferAccess + Send + Sync>, set: Arc<dyn DescriptorSet>, pipe_data: &PipelineData) -> Result<(), GraphicsError>;
    fn set_blend_mode(&mut self, mode: BlendMode) -> Result<(), GraphicsError>;
    fn blend_mode(&self) -> BlendMode;
    fn layout(&self) -> &[Arc<DescriptorSetLayout>];
    fn pipeline(&self) -> Arc<GraphicsPipeline>;
}

impl ShaderHandle for ShaderProgram {
    fn draw(&self, command_buffer: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>, slice: Arc<dyn BufferAccess + Send + Sync>, set: Arc<dyn DescriptorSet>, pipe_data: &PipelineData) -> Result<(), GraphicsError> {
        command_buffer.bind_pipeline_graphics(self.pipeline().clone());

        // let mut builder = PersistentDescriptorSet::start(self.layout().clone());
        // for i in 0..self.layout().num_bindings() {
        //     match self.layout().descriptor(i).unwrap().ty {
        //         DescriptorDescTy::UniformBuffer => {builder.add_buffer(pipe_data.descriptor_sets.get(&i).unwrap().clone()).unwrap();},
        //         _ => {panic!("Unsupported descriptor type in shader.")},
        //     }
        // }

        // let set = builder.build().unwrap();

        command_buffer.bind_vertex_buffers(0, slice);
        command_buffer.draw(pipe_data.vert_count, 1, 0, 0).unwrap();
        Ok(())
    }

    fn set_blend_mode(&mut self, mode: BlendMode) -> Result<(), GraphicsError> {
        let _ = self.pipelines.mode(&mode)?;
        self.current_mode = mode;
        Ok(())
    }

    fn blend_mode(&self) -> BlendMode {
        self.current_mode
    }

    fn layout(&self) -> &[Arc<DescriptorSetLayout>] {
        self.pipelines.get(&self.current_mode).unwrap().layout().descriptor_set_layouts()
    }

    fn pipeline(&self) -> Arc<GraphicsPipeline> {
        self.pipelines.get(&self.current_mode).unwrap().clone()
    }
}

impl ShaderProgram {
    pub fn new<Vd>(
        context: &mut GraphicsContext, 
        vertex_type: Vd, 
        vertex_order: VertexOrder, 
        vertex_shader: GraphicsEntryPoint, 
        fragment_shader: GraphicsEntryPoint, 
        blend: BlendMode
    ) -> Self 
    where
        Vd: VertexDefinition + 'static + Sync + Send,
    {
        let po = new_pipeline(
            context, 
            vertex_type, 
            vertex_order,
            vertex_shader, 
            fragment_shader, 
            blend,
        );

        let mut pos = PipelineObjectSet::new(16);
        pos.insert(blend, po);

        Self {
            pipelines: pos,
            current_mode: blend,
        }
    }

    pub fn from_pipeline(mode: BlendMode, pipeline: Arc<GraphicsPipeline>) -> Self {
        let mut pipeline_os = PipelineObjectSet::new(16);
        pipeline_os.insert(mode, pipeline);
        Self {
            pipelines: pipeline_os,
            current_mode: mode,
        }
    }
}


// This structure is to store multiple pipelines for different blend modes.
pub struct PipelineObjectSet {
    pipelines: HashMap<BlendMode, Arc<GraphicsPipeline>>,
}

impl PipelineObjectSet {
    pub fn new(cap: usize) -> Self {
        Self {
            pipelines: HashMap::with_capacity(cap),
        }
    }

    pub fn insert(&mut self, blend_mode: BlendMode, pipeline: Arc<GraphicsPipeline>) {
        self.pipelines.insert(blend_mode, pipeline);
    }

    pub fn get(&self, blend_mode: &BlendMode) -> Option<&Arc<GraphicsPipeline>> {
        self.pipelines.get(blend_mode)
    }

    pub fn mode(&self, mode: &BlendMode) -> Result<&GraphicsPipeline, GraphicsError> {
        match self.pipelines.get(&mode) {
            Some(po) => Ok(po),
            None => Err(GraphicsError::PipelineError(
                "Couldn't find a pipeline for the specified shader and BlendMode".into(),
            )),
        }
    }
}

pub fn new_pipeline<Vd>(
    context: &mut GraphicsContext, 
    vertex_type: Vd, 
    vertex_order: VertexOrder,
    vertex_shader: GraphicsEntryPoint, 
    fragment_shader: GraphicsEntryPoint, 
    blend: BlendMode) -> Arc<GraphicsPipeline>
where
    Vd: VertexDefinition + 'static + Sync + Send,
{
    let mut pipeline =
        GraphicsPipeline::start()
            .vertex_input::<Vd>(vertex_type)
            .vertex_shader(vertex_shader, ())
            .viewports_dynamic_scissors_irrelevant(1)
            .fragment_shader(fragment_shader, ())
            .blend_collective(blend.into())
            .render_pass(Subpass::from(context.render_pass.clone(), 0).unwrap());
    
    pipeline = match vertex_order {
        VertexOrder::LineList => pipeline.line_list(),
        VertexOrder::LineStrip => pipeline.line_strip(),
        VertexOrder::PointList => pipeline.point_list(),
        VertexOrder::TriangleFan => pipeline.triangle_fan(),
        VertexOrder::TriangleList => pipeline.triangle_list(),
        VertexOrder::TriangleStrip => pipeline.triangle_strip(),
    };
        
    Arc::new(
        pipeline.build(context.device.clone())
                .unwrap()
    )
}

impl From<BlendMode> for Blend {
    fn from(blend_mode: BlendMode) -> Self {
        let mut logic_op: Option<LogicOp> = None;
        let mut attachments: AttachmentBlend = AttachmentBlend::ignore_source();
        let blend_constants: Option<[f32; 4]> = Some([1.0, 1.0, 1.0, 1.0]); // TODO implement these.

        match blend_mode {
            BlendMode::Add => {
                attachments = AttachmentBlend {
                    enabled: true,
                    color_op: BlendOp::Add,
                    color_source: BlendFactor::One,
                    color_destination: BlendFactor::One,
                    alpha_op: BlendOp::Add,
                    alpha_source: BlendFactor::One,
                    alpha_destination: BlendFactor::One,
                    mask_red: true,
                    mask_green: true,
                    mask_blue: true,
                    mask_alpha: true,
                };
            },
            BlendMode::Subtract => {
                attachments = AttachmentBlend {
                    enabled: true,
                    color_op: BlendOp::Subtract,
                    color_source: BlendFactor::One,
                    color_destination: BlendFactor::One,
                    alpha_op: BlendOp::Subtract,
                    alpha_source: BlendFactor::One,
                    alpha_destination: BlendFactor::One,
                    mask_red: true,
                    mask_green: true,
                    mask_blue: true,
                    mask_alpha: true,
                };
            },
            BlendMode::Alpha => {
                attachments = AttachmentBlend::alpha_blending();
            },
            BlendMode::Invert => {
                logic_op = Some(LogicOp::Invert);
            },
        };

        return Blend {
            logic_op,
            attachments: AttachmentsBlend::Collective(attachments),
            blend_constants,
        }
    }
}

impl From<BlendMode> for AttachmentBlend {
    fn from(blend_mode: BlendMode) -> AttachmentBlend {
        let blend: Blend = blend_mode.into();
        match blend.attachments {
            AttachmentsBlend::Collective(attachment) => {
                attachment
            },
            _ => {
                AttachmentBlend::pass_through() // TODO Fix so it cannot fail.
            }
        }
    }
}