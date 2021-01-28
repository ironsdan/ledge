use vulkano::buffer::CpuBufferPool;
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::device::{Device, DeviceExtensions};
use vulkano::format::Format;
use vulkano::framebuffer::{Subpass, RenderPassAbstract};
use vulkano::image::{Dimensions, ImageUsage, ImmutableImage};
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract};
use vulkano::sampler::{Filter, MipmapMode, Sampler, SamplerAddressMode};
use vulkano::swapchain::{
    self, AcquireError, ColorSpace, FullscreenExclusive, PresentMode, SurfaceTransform, Swapchain,
    SwapchainCreationError,
};
use vulkano::sync::{self, FlushError, GpuFuture};
use vulkano_win::VkSurfaceBuild;

use winit::window::WindowBuilder;
use winit::{dpi::Size,dpi::PhysicalSize};

// use png;
use image::ImageFormat;
use std::io::Cursor;
use std::sync::Arc;
// use std::rc::Rc;
// use std::cell::RefCell;
// use std::time::SystemTime;

use crate::lib::*;
use crate::sprite::*;
use crate::animation::*;
use crate::conf::*;
use crate::graphics::{vs, fs};

pub struct GraphicsContext {
    pub queue: std::sync::Arc<vulkano::device::Queue>,
    pub surface: std::sync::Arc<vulkano::swapchain::Surface<winit::window::Window>>,
    pub device: std::sync::Arc<vulkano::device::Device>,
    pub swapchain: std::sync::Arc<vulkano::swapchain::Swapchain<winit::window::Window>>,
    pub sampler: std::sync::Arc<vulkano::sampler::Sampler>,
    pub framebuffers: std::vec::Vec<std::sync::Arc<dyn vulkano::framebuffer::FramebufferAbstract + std::marker::Send + std::marker::Sync>>,
    pub render_pass: std::sync::Arc<dyn RenderPassAbstract + std::marker::Send + std::marker::Sync>,
    pub dynamic_state: vulkano::command_buffer::DynamicState,
    pub buffer_pool: vulkano::buffer::CpuBufferPool<Vertex>,
    pub pipeline: std::sync::Arc<dyn vulkano::pipeline::GraphicsPipelineAbstract + std::marker::Send + std::marker::Sync>,
    pub previous_frame_end: std::option::Option<std::boxed::Box<dyn vulkano::sync::GpuFuture>>, 
    pub recreate_swapchain: bool, 
    pub clear_value: std::vec::Vec<vulkano::format::ClearValue>,
    pub command_buffer: Option<vulkano::command_buffer::AutoCommandBufferBuilder>,
    pub acquire_future: Option<vulkano::swapchain::SwapchainAcquireFuture<winit::window::Window>>,
    pub image_num: Option<usize>,
    pub layout: std::sync::Arc<vulkano::descriptor::descriptor_set::UnsafeDescriptorSetLayout>,
}

impl GraphicsContext {
    pub fn new(event_loop: &winit::event_loop::EventLoop<()>, conf: Conf) -> Self {
        let required_extensions = vulkano_win::required_extensions();
        let instance = Instance::new(None, &required_extensions, None).unwrap();
        let physical = PhysicalDevice::enumerate(&instance).next().unwrap();
        
        println!(
            "Using device: {} (type: {:?})",
            physical.name(),
            physical.ty()
        );

        let surface = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(conf.window_mode.width, conf.window_mode.height))
            .with_min_inner_size(PhysicalSize::new(conf.window_mode.min_width, conf.window_mode.min_height))
            .with_resizable(conf.window_mode.resizable)
            .with_title(conf.window_setup.title)
            .with_maximized(conf.window_mode.maximized)
            // .with_window_icon(Some(conf.))
            .build_vk_surface(event_loop, instance.clone())
            .unwrap();

        let queue_family = physical
            .queue_families()
            .find(|&q| q.supports_graphics() && surface.is_supported(q).unwrap_or(false))
            .unwrap();

        let device_ext = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::none()
        };
        let (device, mut queues) = Device::new(
            physical,
            physical.supported_features(),
            &device_ext,
            [(queue_family, 0.5)].iter().cloned(),
        ).unwrap();
        
        let queue = queues.next().unwrap(); 

        let (swapchain, images) = {
            let caps = surface.capabilities(physical).unwrap();
            let alpha = caps.supported_composite_alpha.iter().next().unwrap();
            let format = caps.supported_formats[0].0;
            let dimensions: [u32; 2] = surface.window().inner_size().into();

            Swapchain::new(
                device.clone(),
                surface.clone(),
                caps.min_image_count,
                format,
                dimensions,
                1,
                ImageUsage::color_attachment(),
                &queue,
                SurfaceTransform::Identity,
                alpha,
                PresentMode::Fifo,
                FullscreenExclusive::Default,
                true,
                ColorSpace::SrgbNonLinear,
            )
            .unwrap()
        };

        vulkano::impl_vertex!(Vertex, position, tex_coords);

        // Vertex Buffer Pool
        let buffer_pool: CpuBufferPool<Vertex> = CpuBufferPool::vertex_buffer(device.clone());

        let vs = vs::Shader::load(device.clone()).unwrap();
        let fs = fs::Shader::load(device.clone()).unwrap();

        let render_pass = Arc::new(
            vulkano::single_pass_renderpass!(device.clone(),
                attachments: {
                    color: {
                        load: Clear,
                        store: Store,
                        format: swapchain.format(),
                        samples: 1,
                    }
                },
                pass: {
                    color: [color],
                    depth_stencil: {}
                }
            )
            .unwrap(),
        );

        let sampler = Sampler::new( 
            device.clone(),
            Filter::Linear,
            Filter::Linear,
            MipmapMode::Nearest,
            SamplerAddressMode::Repeat,
            SamplerAddressMode::Repeat,
            SamplerAddressMode::Repeat,
            0.0,
            1.0,
            0.0,
            0.0,
        )
        .unwrap();

        let pipeline = Arc::new(
            GraphicsPipeline::start()
                .vertex_input_single_buffer::<Vertex>()
                .vertex_shader(vs.main_entry_point(), ())
                .triangle_strip()
                .viewports_dynamic_scissors_irrelevant(1)
                .fragment_shader(fs.main_entry_point(), ())
                .blend_alpha_blending()
                .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
                .build(device.clone())
                .unwrap(),
        ) as Arc<dyn GraphicsPipelineAbstract + Send + Sync>;
        
        let mut dynamic_state = DynamicState {
            line_width: None,
            viewports: None,
            scissors: None,
            compare_mask: None,
            write_mask: None,
            reference: None,
        };

        let framebuffers =
            window_size_dependent_setup(&images, render_pass.clone(), &mut dynamic_state);

        let (_, empty_future) = {
            ImmutableImage::from_iter(
                [0,0,0,0].to_vec().iter().cloned(),
                Dimensions::Dim2d {width: 1, height: 1},
                Format::R8G8B8A8Srgb,
                queue.clone(),
            ).unwrap()
        };

        let layout = pipeline.clone().descriptor_set_layout(0).unwrap().clone();

        Self {
            queue: queue,
            surface: surface,
            device: device,
            swapchain: swapchain,
            sampler: sampler,
            framebuffers: framebuffers,
            render_pass: render_pass,
            dynamic_state: dynamic_state,
            buffer_pool: buffer_pool,
            pipeline: pipeline,
            previous_frame_end: Some(empty_future.boxed()),
            recreate_swapchain: false,
            clear_value: vec![[0.2, 0.2, 0.2, 1.0].into()],
            command_buffer: None,
            acquire_future: None,
            image_num: None,
            layout: layout,
        }
    }

    pub fn add_physics_object(&mut self, name: String, position: [f32;2], file_bytes: &[u8], size: [u32;2], matrix_dims:[u32;2], animation_machine: Option<AnimationStateMachine>) {
        let sprite = self.create_sprite(name, position, file_bytes, size, matrix_dims, animation_machine);
    }

    pub fn create_sprite(&self, name: String, position: [f32; 2], file_bytes: &[u8], size: [u32; 2], matrix_dims: [u32; 2], animation_machine: Option<AnimationStateMachine>) -> Sprite {
        let (texture, _) = {
            let image = image::load_from_memory_with_format(file_bytes,
                ImageFormat::Png).unwrap().to_rgba8();
            let dimensions = image.dimensions();
            let image_data = image.into_raw().clone();
    
            ImmutableImage::from_iter(
                image_data.iter().cloned(),
                Dimensions::Dim2d { width: dimensions.0, height: dimensions.1 },
                Format::R8G8B8A8Srgb,
                self.queue.clone(),
            )
            .unwrap()
        };

        let sprite = Sprite::new(name, texture.clone(), position, size, matrix_dims, animation_machine);
        // self.sprites.push(sprite);

        return sprite;
    }

    pub fn begin_frame(&mut self, previous_frame_end: &mut std::option::Option<std::boxed::Box<dyn vulkano::sync::GpuFuture>>) {
        // let start = SystemTime::now();
        previous_frame_end.as_mut().unwrap().cleanup_finished();

        if self.recreate_swapchain {
            let dimensions: [u32; 2] = self.surface.window().inner_size().into();
            let (new_swapchain, new_images) =
                match self.swapchain.recreate_with_dimensions(dimensions) {
                    Ok(r) => r,
                    Err(SwapchainCreationError::UnsupportedDimensions) => return,
                    Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
                };

            self.swapchain = new_swapchain;
            self.framebuffers = window_size_dependent_setup(
                &new_images,
                self.render_pass.clone(),
                &mut self.dynamic_state,
            );
            self.recreate_swapchain = false;
        }

        let (image_num, suboptimal, acquire_future) =
            match swapchain::acquire_next_image(self.swapchain.clone(), None) {
                Ok(r) => r,
                Err(AcquireError::OutOfDate) => {
                    self.recreate_swapchain = true;
                    return;
                }
                Err(e) => panic!("Failed to acquire next image: {:?}", e),
            };

        if suboptimal {
            self.recreate_swapchain = true;
        }

        self.image_num = Some(image_num);
        self.acquire_future = Some(acquire_future);

        let mut builder =
            AutoCommandBufferBuilder::primary_one_time_submit(self.device.clone(), self.queue.family())
                .unwrap();

        self.command_buffer = Some(builder);

        let clear_values = vec![[0.2, 0.2, 0.2, 1.0].into()];
        self.command_buffer.as_mut().unwrap().begin_render_pass(self.framebuffers[image_num].clone(), false, clear_values).unwrap();
                        
    }

    // Uses Vulkano magic to draw the selected sprites to the screen.
    pub fn draw(&mut self, sprite: Sprite) {
        let data = &sprite.rect;
        let vertex_buffer = self.buffer_pool.chunk(data.vertices.to_vec()).unwrap();
        let set = Arc::new(
            PersistentDescriptorSet::start(self.layout.clone())
                .add_sampled_image(sprite.texture.clone(), self.sampler.clone())
                .unwrap()
                .build()
                .unwrap(),
        );
        self.command_buffer.as_mut().unwrap().draw(
            self.pipeline.clone(),
            &self.dynamic_state,
            vec!(Arc::new(vertex_buffer.clone())),
            set.clone(),
            (),
        ).unwrap();
    }

    pub fn present(&mut self, previous_frame_end: &mut std::option::Option<std::boxed::Box<dyn vulkano::sync::GpuFuture>>) {
        self.command_buffer.as_mut().unwrap().end_render_pass().unwrap();
        let command_buffer = self.command_buffer.take().unwrap().build().unwrap();

        let future = previous_frame_end
            .take()
            .unwrap()
            .join(self.acquire_future.take().unwrap())
            .then_execute(self.queue.clone(), command_buffer)
            .unwrap()
            .then_swapchain_present(self.queue.clone(), self.swapchain.clone(), self.image_num.unwrap())
            .then_signal_fence_and_flush();

        match future {
            Ok(future) => {
                *previous_frame_end = Some(future.boxed());
            }
            Err(FlushError::OutOfDate) => {
                self.recreate_swapchain = true;
                *previous_frame_end = Some(sync::now(self.device.clone()).boxed());
            }
            Err(e) => {
                println!("Failed to flush future: {:?}", e);
                *previous_frame_end = Some(sync::now(self.device.clone()).boxed());
            }
        };
        
        // timestep = start.elapsed().unwrap().as_millis() as f32;
        // print!("Redraw: ");
        self.surface.window().request_redraw();
    }
}