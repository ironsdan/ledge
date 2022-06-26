use vulkano::image::ImageViewAbstract;
use vulkano::device::Queue;
use vulkano::sync::GpuFuture;
use crate::graphics::camera::Camera;
use crate::graphics::shader::{Shader, ShaderId, ShaderHandle, ShaderProgram};

use vulkano::{
    render_pass::{Framebuffer, FramebufferCreateInfo},
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, SubpassContents,
    },
};

use std::sync::Arc;

use anyhow::*;

use vulkano::pipeline::graphics::vertex_input::VertexDefinition;
use crate::graphics::BlendMode;

pub mod frame;

pub struct RenderPass {
    queue: Arc<Queue>,
    shaders: Vec<Box<dyn ShaderHandle>>,
    pub render_pass: Arc<vulkano::render_pass::RenderPass>,
}

impl RenderPass {
    pub fn new(queue: Arc<Queue>, render_pass: Arc<vulkano::render_pass::RenderPass>) -> Result<RenderPass> {
        Ok(Self {
            queue: queue.clone(),
            shaders: Vec::new(),
            render_pass,
        })
    }

    pub fn register_shader<Vd: VertexDefinition + 'static + Sync + Send>(&mut self, shader: Arc<Shader>, v_type: Vd) -> Result<ShaderId> {
        self.shaders.push(
            Box::new(
                ShaderProgram::new(
                    self.queue.device().clone(),
                    self.render_pass.clone(),
                    v_type,
                    shader.topology,
                    shader.vertex.clone(),
                    shader.fragment.clone(),
                    BlendMode::Alpha,
                )
            )
        );

        Ok(self.shaders.len()-1)
    }

    pub fn frame(&mut self,
        clear_color: [f32; 4],
        before_future: Box<dyn GpuFuture + 'static>,
        final_image: Arc<dyn ImageViewAbstract + 'static>,
        _camera: Arc<dyn Camera>,
    ) -> Result<frame::Frame>
    {
        let _img_dims = final_image.image().dimensions().width_height();

        let framebuffer = Framebuffer::new(
           self.render_pass.clone(),
            FramebufferCreateInfo {
                attachments: vec![final_image],
                ..Default::default()
            },
        )?;

        let mut command_buffer = AutoCommandBufferBuilder::primary(
            self.queue.device().clone(),
            self.queue.family(),
            CommandBufferUsage::OneTimeSubmit,
        )?;

        command_buffer.begin_render_pass(
            framebuffer.clone(),
            SubpassContents::SecondaryCommandBuffers,
            vec![clear_color.into()],
        )?;

        // if render_pass.subpasses().len() > 16 {
        //     return Err(i)
        // }

        let num_pass = self.render_pass.subpasses().len() as u8;

        Ok(frame::Frame {
            pipelines: &self.shaders,
            before_main_cb_future: Some(before_future),
            // framebuffer,
            queue: self.queue.clone(),
            num_pass,
            cur_pass: 0,
            command_buffer: Some(command_buffer),
            // camera,
        })
    }
}