use std::sync::Arc;
use std::collections::HashMap;

use vulkano::{
    descriptor_set::{
        DescriptorSet, 
        layout::DescriptorSetLayout,
    },
    render_pass::{Subpass},
    pipeline::{
        shader::GraphicsEntryPoint,
        GraphicsPipeline, 
        // GraphicsPipelineAbstract,
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
};

use crate::graphics::{
    context::GraphicsContext,
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

// pub struct Shader {
//     pub shader_type: ShaderType,
//     pub entry_point: GraphicsEntryPoint,
//     pub specialization_constants: ,
// }

// impl<S, C> Shader<S, C> {
//     pub fn new(entry_point: S, specialization_constants: C) -> Self {
//         Self {
//             shader_type: ShaderType::Default,
//             entry_point,
//             specialization_constants,
//         }
//     }
// }

pub struct ShaderProgram {
    pipelines: PipelineObjectSet,
    current_mode: BlendMode,
}

pub trait ShaderHandle {
    fn draw(&self, context: &mut GraphicsContext, slice: Arc<dyn BufferAccess + Send + Sync>, descriptor: Arc<dyn DescriptorSet + Send + Sync>) -> Result<(), GraphicsError>;
    fn set_blend_mode(&mut self, mode: BlendMode) -> Result<(), GraphicsError>;
    fn blend_mode(&self) -> BlendMode;
    fn layout(&self) -> Arc<DescriptorSetLayout>;
    fn pipeline(&self) -> Arc<GraphicsPipeline>;
}

impl ShaderHandle for ShaderProgram {
    fn draw(&self, context: &mut GraphicsContext, slice: Arc<dyn BufferAccess + Send + Sync>, descriptor: Arc<dyn DescriptorSet + Send + Sync>) -> Result<(), GraphicsError> {
        // let po = self.pipelines.mode(&self.current_mode)?;
        // context.command_buffer.as_mut().unwrap().draw(
        //     po.pipeline.clone(),
        //     &context.dynamic_state,
        //     vec![Arc::new(slice.clone())],
        //     descriptor.clone(),
        // ).unwrap(); // TODO fix to return useful error.
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

    fn layout(&self) -> Arc<DescriptorSetLayout> {
        self.pipelines.get(&self.current_mode).unwrap().descriptor_set_layout()
    }

    fn pipeline(&self) -> Arc<GraphicsPipeline> {
        self.pipelines.get(&self.current_mode).unwrap().inner()
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
        let po = PipelineObject::new(
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

    pub fn from_pipeline(mode: BlendMode, pipeline: PipelineObject) -> Self {
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
    pipelines: HashMap<BlendMode, PipelineObject>,
}

impl PipelineObjectSet {
    pub fn new(cap: usize) -> Self {
        Self {
            pipelines: HashMap::with_capacity(cap),
        }
    }

    pub fn insert(&mut self, blend_mode: BlendMode, pipeline: PipelineObject) {
        self.pipelines.insert(blend_mode, pipeline);
    }

    pub fn get(&self, blend_mode: &BlendMode) -> Option<&PipelineObject> {
        self.pipelines.get(blend_mode)
    }

    pub fn mode(&self, mode: &BlendMode) -> Result<&PipelineObject, GraphicsError> {
        match self.pipelines.get(&mode) {
            Some(po) => Ok(po),
            None => Err(GraphicsError::PipelineError(
                "Couldn't find a pipeline for the specified shader and BlendMode".into(),
            )),
        }
    }
}

pub struct PipelineObject {
    // pub pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
    pub pipeline: Arc<GraphicsPipeline>,
}

impl PipelineObject {
    pub fn new<Vd>(
        context: &mut GraphicsContext, 
        vertex_type: Vd, 
        vertex_order: VertexOrder,
        vertex_shader: GraphicsEntryPoint, 
        fragment_shader: GraphicsEntryPoint, 
        blend: BlendMode) -> Self 
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
            
        let pipeline = Arc::new(
            pipeline.build(context.device.clone())
                    .unwrap()
        );
        // as Arc<dyn GraphicsPipelineAbstract + Send + Sync>;

        Self {
            pipeline,
        }
    }

    pub fn descriptor_set_layout(&self) -> Arc<DescriptorSetLayout> {
        self.pipeline.layout().descriptor_set_layouts()[0].clone()
    }

    pub fn inner(&self) -> Arc<GraphicsPipeline> {
        self.pipeline.clone()
    }
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