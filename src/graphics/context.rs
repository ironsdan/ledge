use vulkano::{
    buffer::CpuBufferPool,
    command_buffer::{AutoCommandBufferBuilder, DynamicState},
    device::{Device, DeviceExtensions},
    framebuffer::{Subpass, RenderPassAbstract},
    image::ImageUsage,
    instance::{Instance, PhysicalDevice},
    pipeline::{GraphicsPipeline, GraphicsPipelineAbstract},
    sampler::{Filter, MipmapMode, Sampler, SamplerAddressMode},
    swapchain::{
        self, AcquireError, ColorSpace, FullscreenExclusive, PresentMode, SurfaceTransform, Swapchain,
        SwapchainCreationError,
    },
    sync::{self, FlushError, GpuFuture},
    command_buffer::pool::standard::StandardCommandPoolBuilder,
    swapchain::SwapchainAcquireFuture,
    command_buffer::SubpassContents,
    instance::debug::{DebugCallback, MessageSeverity, MessageType},
    instance::{self, InstanceExtensions},
};
use vulkano::descriptor::descriptor_set::PersistentDescriptorSetImg;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSetSampler;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSetBuf;
// use vulkano::buffer::cpu_pool::CpuBufferPoolSubbuffer;
use vulkano::buffer::cpu_pool::CpuBufferPoolChunk;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::image::ImmutableImage;
use vulkano::format::Format;
use vulkano::pipeline::vertex::OneVertexOneInstanceDefinition;
use vulkano::memory::pool::StdMemoryPool;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::{
    framebuffer::{Framebuffer, FramebufferAbstract},
    image::SwapchainImage,
    pipeline::viewport::Viewport,
};
use vulkano_win::VkSurfaceBuild;
use winit::{
    window::{Window, WindowBuilder},
    dpi::PhysicalSize,
};
use std::sync::Arc;
use std::collections::HashMap;
use crate::{
    graphics::{DrawInfo, Vertex, InstanceData, shader::PipelineObjectSet, shader::PipelineObject, BlendMode, sprite::SpriteBatch},
    // graphics::shader::{Shader, ShaderProgram},
    conf::*,
    graphics::{vs, fs},
};
use crate::asset::handle::HandleId;
use crate::asset::types::Texture;

type MvpUniform = vs::ty::mvp;

// #[derive(Default)]
pub struct FrameData {
    pub blend_mode: BlendMode,
    pub vbuf: Option<CpuBufferPoolChunk<Vertex, Arc<StdMemoryPool>>>,
    pub instance_data: Option<CpuBufferPoolChunk<InstanceData, Arc<StdMemoryPool>>>,
    pub uniform_descriptor_set: Option<Arc<PersistentDescriptorSet<((((), PersistentDescriptorSetBuf<Arc<CpuAccessibleBuffer<vs::ty::mvp>>>), PersistentDescriptorSetImg<Arc<ImmutableImage<Format>>>), PersistentDescriptorSetSampler)>>>,
}

pub struct GraphicsContext {
    pub queue: std::sync::Arc<vulkano::device::Queue>,
    pub surface: std::sync::Arc<vulkano::swapchain::Surface<winit::window::Window>>,
    pub device: std::sync::Arc<vulkano::device::Device>,
    pub swapchain: std::sync::Arc<vulkano::swapchain::Swapchain<winit::window::Window>>,
    pub sampler: std::sync::Arc<vulkano::sampler::Sampler>,
    pub framebuffers: std::vec::Vec<std::sync::Arc<dyn vulkano::framebuffer::FramebufferAbstract + std::marker::Send + std::marker::Sync>>,
    pub render_pass: std::sync::Arc<dyn RenderPassAbstract + std::marker::Send + std::marker::Sync>,
    pub dynamic_state: vulkano::command_buffer::DynamicState,
    pub vertex_buffer_pool: vulkano::buffer::CpuBufferPool<Vertex>,
    pub instance_buffer_pool: vulkano::buffer::CpuBufferPool<InstanceData>,
    pub mvp_buffer: std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<MvpUniform>>,
    pub frame_data: FrameData,
    pub default_pipeline_id: usize, 
    pub pipeline_sets: Vec<PipelineObjectSet>,
    pub image_num: usize,
    pub acquire_future: Option<SwapchainAcquireFuture<Window>>,
    pub recreate_swapchain: bool,
    pub previous_frame_end: Option<Box<dyn GpuFuture>>,
    pub command_buffer: Option<AutoCommandBufferBuilder<StandardCommandPoolBuilder>>,
    pub now: Option<std::time::Instant>,
    pub batches: HashMap<HandleId, SpriteBatch>,
}

impl GraphicsContext {
    pub fn new(event_loop: &winit::event_loop::EventLoop<()>, conf: Conf) -> Self{
        let required_extensions = vulkano_win::required_extensions();

        let extensions = InstanceExtensions {
            ext_debug_utils: true,
            ..required_extensions
        };
    
        // println!("List of Vulkan debugging layers available to use:");
        // let mut layers = instance::layers_list().unwrap();
        // while let Some(l) = layers.next() {
        //     println!("\t{}", l.name());
        // }
    
        // let layer = "VK_LAYER_KHRONOS_validation";
        // let layers = vec![layer];
    
        let instance =
            Instance::new(None, &extensions, vec![]).expect("failed to create Vulkan instance");

        // let severity = MessageSeverity {
        //     error: true,
        //     warning: true,
        //     information: true,
        //     verbose: true,
        // };
    
        // let ty = MessageType::all();
    
        // let debug_callback = DebugCallback::new(&instance, severity, ty, |msg| {
        //     let _severity = if msg.severity.error { "error" } 
        //     else if msg.severity.warning { "warning" } 
        //     else if msg.severity.information { "information" } 
        //     else if msg.severity.verbose { "verbose" } 
        //     else { panic!("no-impl"); };
    
        //     let _ty = if msg.ty.general { "general" } 
        //     else if msg.ty.validation { "validation" } 
        //     else if msg.ty.performance { "performance" } 
        //     else { panic!("no-impl"); };
    
        //     // println!("[{}]: {} {}: {}", ty, msg.layer_prefix, severity, msg.description);
        // })
        // .ok();

        let physical = PhysicalDevice::enumerate(&instance).next().unwrap();
        println!(
            "Using device: {} (type: {:?})\nExtensions: {:?}",
            physical.name(),
            physical.ty(),
            required_extensions,
        );

        let surface = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(conf.window_mode.width, conf.window_mode.height))
            .with_min_inner_size(PhysicalSize::new(conf.window_mode.min_width, conf.window_mode.min_height))
            .with_resizable(conf.window_mode.resizable)
            .with_title(conf.window_setup.title)
            .with_maximized(conf.window_mode.maximized)
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
        )
        .unwrap();

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
                PresentMode::Immediate, // This is definitely not great. But keeps from frame spikes
                FullscreenExclusive::Default,
                true,
                ColorSpace::SrgbNonLinear,
            )
            .unwrap()
        };

        // Vertex Buffer Pool
        let buffer_pool: CpuBufferPool<Vertex> = CpuBufferPool::vertex_buffer(device.clone());

        let instance_buffer_pool: CpuBufferPool<InstanceData> = CpuBufferPool::vertex_buffer(device.clone());

        // Model View Projection buffer
        let default_mvp_mat = MvpUniform { mvp: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]]};
        let mvp_buffer = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::uniform_buffer_transfer_destination(), false, default_mvp_mat).unwrap();
        
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

        vulkano::impl_vertex!(Vertex, a_pos, a_uv, a_vert_color);
        vulkano::impl_vertex!(InstanceData, a_src, a_color, a_transform);

        let vs = vs::Shader::load(device.clone()).unwrap();
        let fs = fs::Shader::load(device.clone()).unwrap();


        let pipeline = Arc::new(
            GraphicsPipeline::start()
                // .vertex_input_single_buffer::<Vertex>()
                .vertex_input(OneVertexOneInstanceDefinition::<Vertex, InstanceData>::new())
                .vertex_shader(vs.main_entry_point(), ())
                .triangle_strip()
                .viewports_dynamic_scissors_irrelevant(1)
                .fragment_shader(fs.main_entry_point(), ())
                .blend_alpha_blending()
                .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
                .build(device.clone())
                .unwrap()
        ) as Arc<dyn GraphicsPipelineAbstract + Send + Sync>;

        let pipeline_object = PipelineObject::new(pipeline);
        let mut pipeline_sets = PipelineObjectSet::new(128);
        pipeline_sets.insert(BlendMode::Alpha, pipeline_object);
        
        let mut dynamic_state = DynamicState {
            line_width: None,
            viewports: None,
            scissors: None,
            compare_mask: None,
            write_mask: None,
            reference: None,
        };

        let sampler = Sampler::new( 
            device.clone(),
            Filter::Linear,
            Filter::Linear,
            MipmapMode::Nearest,
            SamplerAddressMode::ClampToBorder(vulkano::sampler::BorderColor::IntTransparentBlack),
            SamplerAddressMode::ClampToBorder(vulkano::sampler::BorderColor::IntTransparentBlack),
            SamplerAddressMode::ClampToBorder(vulkano::sampler::BorderColor::IntTransparentBlack),
            0.0, 1.0, 0.0, 0.0,
        ).unwrap();

        let default_future = sync::now(device.clone()).boxed();

        let framebuffers =
            window_size_dependent_setup(&images, render_pass.clone(), &mut dynamic_state);

        // let default_shader = Shader::new();

        let mut graphics = Self {
            queue: queue,
            surface: surface,
            device: device,
            swapchain: swapchain,
            sampler: sampler,
            framebuffers: framebuffers,
            render_pass: render_pass,
            dynamic_state: dynamic_state,
            vertex_buffer_pool: buffer_pool,
            instance_buffer_pool: instance_buffer_pool, 
            mvp_buffer: mvp_buffer,
            frame_data: FrameData{ vbuf: None, instance_data: None, uniform_descriptor_set: None, blend_mode: BlendMode::Alpha },
            default_pipeline_id: 0,
            pipeline_sets: vec![pipeline_sets],
            image_num: 0,
            acquire_future: None,
            previous_frame_end: Some(default_future),
            recreate_swapchain: false,
            command_buffer: None,
            now: None,
            batches: HashMap::new(),
            // debugger: debug_callback
        };

        graphics.create_command_buffer();
        graphics.begin_frame();
        graphics
    }

    fn create_command_buffer(&mut self,) {
        let builder =
        AutoCommandBufferBuilder::primary_one_time_submit(self.device.clone(), self.queue.family())
            .unwrap();
        self.command_buffer = Some(builder);
    }

    // Handles setup of a new frame, called when the graphics pipeline is first created and 
    // at the end of every frame to start the next one. This is necessary because the swapchain
    // could be out of date and the command_buffer needs to be recreated each frame, as well as
    // Updating the image_num, optimality, and the swapcahin future.
    pub fn begin_frame(&mut self) {
        self.now = Some(std::time::Instant::now());
        self.previous_frame_end.as_mut().unwrap().cleanup_finished();

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

        self.image_num = image_num;
        self.acquire_future = Some(acquire_future);
    }

    pub fn draw(&mut self) {
        let clear_values = vec![[0.2, 0.2, 0.2, 1.0].into()];
        self.command_buffer.as_mut().unwrap().begin_render_pass(self.framebuffers[self.image_num].clone(), SubpassContents::Inline, clear_values).unwrap();

        self.command_buffer.as_mut().unwrap().draw(
            self.pipeline_sets[self.default_pipeline_id].get(&self.frame_data.blend_mode).unwrap().pipeline.clone(),
            &self.dynamic_state,
            vec!(Arc::new(self.frame_data.vbuf.as_ref().unwrap().clone()), Arc::new(self.frame_data.instance_data.as_ref().unwrap().clone())),
            self.frame_data.uniform_descriptor_set.as_ref().unwrap().clone(),
            (),
        ).unwrap();

        self.command_buffer.as_mut().unwrap().end_render_pass().unwrap();
    }

    pub fn batch(&mut self, handle_id: HandleId, draw_info: DrawInfo) {
        // sprite batch wont draw until all sprites have been added, who knows when that'll be.
        // so just add sprites till the user says "Okay, present the frame." then draw all sprite batches

        match self.batches.get(&handle_id) {
            Some(batch) => { // else add draw settings to existing sprite batch.
                
            },
            None => { // if sprite batch texture handle isn't present.
                // create a new sprite batch and store it.
            }
        }

        // remove sprite how???
            // 1. clear a vec every round.
            // 2. or never save it just send it to the GPU.
            // 3. create a remove command that the sprite entity can use like: sprite.remove(context).

        // set context to draw on present.
    }

    pub fn present(&mut self) {
        let command_buffer = self.command_buffer.take().unwrap().build().unwrap();

        let future = self.previous_frame_end
            .take()
            .unwrap()
            .join(self.acquire_future.take().unwrap())
            .then_execute(self.queue.clone(), command_buffer)
            .unwrap()
            .then_swapchain_present(self.queue.clone(), self.swapchain.clone(), self.image_num);

        let future = future.then_signal_fence_and_flush();

        match future {
            Ok(future) => {
                self.previous_frame_end = Some(future.boxed());
            }
            Err(FlushError::OutOfDate) => {
                self.recreate_swapchain = true;
                self.previous_frame_end = Some(sync::now(self.device.clone()).boxed());
            }
            Err(e) => {
                println!("Failed to flush future: {:?}", e);
                self.previous_frame_end = Some(sync::now(self.device.clone()).boxed());
            }
        };

        // Limits the frame rate since PresentMode::Immediate has to be used.
        let mut sleep_time = 0.016 - self.now.unwrap().elapsed().as_secs_f32();
        if sleep_time < 0.0 {
            sleep_time = 0.0
        }
        std::thread::sleep(std::time::Duration::from_secs_f32(sleep_time));

        self.create_command_buffer();
        self.begin_frame();
    }

    pub fn get_default_pipeline(&self) -> &PipelineObject {
        self.pipeline_sets[self.default_pipeline_id].get(&self.frame_data.blend_mode).unwrap()
    }
}

// This method is called once during initialization, then again whenever the window is resized
pub fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
    dynamic_state: &mut DynamicState,
) -> Vec<Arc<dyn FramebufferAbstract + Send + Sync>> {
    let dimensions = images[0].dimensions();

    let viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [dimensions[0] as f32, dimensions[1] as f32],
        depth_range: 0.0..1.0,
    };
    dynamic_state.viewports = Some(vec![viewport]);

    images
        .iter()
        .map(|image| {
            Arc::new(
                Framebuffer::start(render_pass.clone())
                    .add(image.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            ) as Arc<dyn FramebufferAbstract + Send + Sync>
        })
        .collect::<Vec<_>>()
}

pub fn convert_to_screen_space(size: [u32;2], dimensions: [u32; 2]) -> [f32; 2] {
    let window_width = dimensions[0];
    let window_height = dimensions[1];

    let aspect_ratio;

    if window_height > window_width {
        aspect_ratio = window_height as f32 / window_width as f32;
    } else {
        aspect_ratio = window_width as f32 / window_height as f32;
    }

    let pixel_size_y = 1.0/window_height as f32;
    let pixel_size_x = 1.0/window_width as f32;

    let screen_width = 2.0*pixel_size_x*size[0] as f32;
    let screen_height = 2.0*pixel_size_y*size[1] as f32;

    let screen_size = [screen_width, screen_height];
    return screen_size;
}