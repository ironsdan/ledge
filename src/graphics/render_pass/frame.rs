use vulkano::{
    command_buffer::{
        AutoCommandBufferBuilder, PrimaryAutoCommandBuffer,
    },
    // render_pass::{Framebuffer},
    sync::{GpuFuture},
    device::Queue,
};

use anyhow::Result;
use std::sync::Arc;

use crate::graphics::shader::*;
use crate::graphics::{Drawable, DrawInfo};
// use crate::graphics::camera::Camera2D;

pub struct Frame<'p> {
    pub(crate) pipelines: &'p Vec<Box<dyn ShaderHandle>>,
    pub(crate) num_pass: u8,
    pub(crate) cur_pass: u8,
    pub(crate) queue: Arc<Queue>,
    pub(crate) before_main_cb_future: Option<Box<dyn GpuFuture>>,
    // pub(crate) framebuffer: Arc<Framebuffer>,
    pub(crate) command_buffer: Option<AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>>,
    // pub(crate) camera: Camera2D,
}

impl<'p> Frame<'p> {
    pub fn next_pass<'f>(&'f mut self) -> Result<Option<PassState<'f, 'p>>> {
        Ok(
            if self.cur_pass < self.num_pass {
                self.cur_pass += 1;
                Some(PassState::DrawPass(Pass {
                    frame: self,
                }))
            } else if self.cur_pass == self.num_pass {
                self.cur_pass += 1;
                self.command_buffer
                .as_mut()
                .unwrap()
                .end_render_pass()?;

                let command_buffer = self.command_buffer.take().unwrap().build()?;

                let after_main_cb = self
                    .before_main_cb_future
                    .take()
                    .unwrap()
                    .then_execute(self.queue.clone(), command_buffer).unwrap();
        
                Some(PassState::Finished(after_main_cb.boxed()))
            } else {
                None
            }
        )
       
    }
}

pub enum PassState<'f, 'p: 'f> {
    DrawPass(Pass<'f, 'p>),
    Finished(Box<dyn GpuFuture>),
}

pub struct Pass<'f, 'p> {
    frame: &'f mut Frame<'p>,
}

impl<'f, 'p> Pass<'f, 'p> {
    pub fn draw_with(&mut self, d: Arc<dyn Drawable>, id: ShaderId, draw_info: DrawInfo) -> Result<()> {
        let shader_handle = self.frame.pipelines.get(id).unwrap();

        let commands = d.draw(
            self.frame.queue.clone(),
            shader_handle,
            draw_info,
        )?;

        self.frame.command_buffer.as_mut().unwrap().execute_commands(commands)?;

        Ok(())
    }
}