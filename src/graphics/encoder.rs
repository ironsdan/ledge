use crate::graphics::*;
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer, SubpassContents, CommandBufferLevel};
use vulkano::command_buffer::pool::{standard::StandardCommandPool, CommandPool};
use vulkano::pipeline::{viewport::Viewport, PipelineBindPoint};
use winit::window::Window;
use vulkano::sync::GpuFuture;
use vulkano::swapchain::{self, AcquireError};
use vulkano::command_buffer::pool::CommandPoolBuilderAlloc;
use vulkano::command_buffer::synced::SyncCommandBufferBuilder;
use vulkano::render_pass::FramebufferAbstract;
use fnv::FnvHashMap;
use std::marker::PhantomData;
use vulkano::OomError;

// Abstracts the command buffer to make an easier to use interface.
pub struct Encoder<T: CommandPool> {
    pub command_buffer_pool: T,
    pub command_buffer: Option<T::Builder>,
    in_render_pass: bool,
    // pub image_num: usize,
    // pub acquire_future: Option<vulkano::swapchain::SwapchainAcquireFuture<Window>>,
    // pub recreate_swapchain: bool,
    // pub previous_frame_end: Option<Box<dyn vulkano::sync::GpuFuture>>,
}

impl<T: CommandPool> Encoder<T> {
    pub fn new(pool: T) -> Self {
        // let default_future = vulkano::sync::now(device.clone()).boxed();
        // let pool = StandardCommandPool::new(device, queue_family);
        Self {
            command_buffer_pool: pool,
            command_buffer: None,
            in_render_pass: false,
            // image_num: 0,
            // acquire_future: None,
            // recreate_swapchain: false,
            // previous_frame_end: Some(default_future),
        }
    }

    fn alloc<F>(&mut self, usage: CommandBufferUsage, level: CommandBufferLevel<F>,) -> Result<T::Builder, OomError>
    where
        F: FramebufferAbstract + Clone + 'static,
    {
        unsafe{
            let pool_builder_alloc = self.command_buffer_pool
                .alloc(!matches!(level, CommandBufferLevel::Primary), 1)?
                .next()
                .expect("Requested one command buffer from the command pool, but got zero.");
            let inner = SyncCommandBufferBuilder::new(pool_builder_alloc.inner(), level, usage)?;

            Ok(AutoCommandBufferBuilder {
                inner,
                pool_builder_alloc,
                queue_family_id: self.command_buffer_pool.queue_family().id(),
                render_pass_state: None,
                query_state: FnvHashMap::default(),
                inheritance: None,
                usage,
                _data: PhantomData,
            })
        }
    }

    pub fn draw(&mut self, pipeline: Arc<vulkano::pipeline::GraphicsPipeline>, pipe_data: &PipelineData) {
        if self.command_buffer.is_none() {
            self.command_buffer = self.command_buffer_pool.alloc(false, 1).unwrap().next();
        }

        // self.command_buffer.as_mut().unwrap().bind_pipeline_graphics(pipeline.clone());
        // for (bind,descriptor_set) in pipe_data.descriptor_sets.clone() {
        //     self.command_buffer.as_mut().unwrap().bind_descriptor_sets(
        //         PipelineBindPoint::Graphics,
        //         pipeline.layout().clone(),
        //         bind,
        //         descriptor_set.clone(),
        //     );
        // }

        // self.command_buffer.as_mut().unwrap().bind_vertex_buffers(0, pipe_data.vert_buf);
    }

    pub fn flush() {

    }

    pub fn flush_no_reset() {

    }

    pub fn fenced_flush() {

    }

    pub fn fenced_flush_no_reset() {

    }

    pub fn reset() {

    }

    pub fn update_buffer() {

    }

    pub fn update_texture() {

    }

    pub fn execute() {
        
    }
}

pub struct GpuCommand {

}