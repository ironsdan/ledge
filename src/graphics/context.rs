use vulkano::{
    command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer, SubpassContents},
    device::{Device, DeviceExtensions},
    image::ImageUsage,
    instance::{Instance},
    sampler::{Filter, MipmapMode, Sampler, SamplerAddressMode},
    swapchain::{self, AcquireError, Swapchain},
    sync::{self, FlushError, GpuFuture},
    instance::InstanceExtensions,
    render_pass::{Framebuffer, FramebufferAbstract, RenderPass},
    image::SwapchainImage,
    pipeline::{viewport::Viewport, PipelineBindPoint},
    image::view::ImageView,
    buffer::{BufferAccess, BufferUsage, CpuAccessibleBuffer},
    descriptor_set::{DescriptorSet},
    device::physical::PhysicalDevice,
};

use vulkano_win::VkSurfaceBuild;
use winit::{
    window::{Window, WindowBuilder},
    dpi::PhysicalSize,
};
use std::sync::Arc;
use std::collections::HashMap;
use crate::{
    conf::*,
    graphics::PipelineData,
    graphics::shader::ShaderHandle,
};


/// This is the context from which the graphics components gets all of its information 
/// about the physical device and the presentation area. It serves as the Vulkano abstraction,
/// which intern interfaces with the Vulkan API.
/// 
/// # Examples
/// 
/// ```
/// use winit::{
///     event_loop::{ControlFlow},
///     event::{Event, WindowEvent}
/// };
/// use ledge_engine::graphics::context::GraphicsContext;
/// use ledge_engine::conf::Conf;
/// 
/// fn main() {
///     let (mut context, event_loop) = GraphicsContext::new(Conf::default());
///
///     event_loop.run(move |event, _, control_flow| {
///         let now = std::time::Instant::now();
///
///         match event {
///             Event::WindowEvent { event, .. } => match event {
///                 WindowEvent::CloseRequested => {
///                     *control_flow = ControlFlow::Exit;
///                 },
///                 WindowEvent::Resized(_) => {
///                     context.recreate_swapchain = true;
///                 },
///                 _ => {},
///             },
///             Event::MainEventsCleared => { 
///                 context.create_command_buffer();
/// 
///                 // buffer updates
/// 
///                 context.begin_frame();
/// 
///                 // draw commands
/// 
///                 context.present();
/// 
///                 // without using timer you have to manually control the frame time.
///                 let mut sleep_time: f64 = 0.016 - now.elapsed().as_secs_f64();
///                 if sleep_time < 0.0 {
///                     sleep_time = 0.0
///                 }

///                 std::thread::sleep(std::time::Duration::from_secs_f64(sleep_time));
///                 print!("{:.2}\r", now.elapsed().as_secs_f32() * 1000.0);
///             },
///             _ => {}
///         }
///     });
/// }
/// ```
pub struct GraphicsContext {
    pub queue: std::sync::Arc<vulkano::device::Queue>,
    pub surface: std::sync::Arc<vulkano::swapchain::Surface<winit::window::Window>>,
    pub device: std::sync::Arc<vulkano::device::Device>,
    pub swapchain: std::sync::Arc<vulkano::swapchain::Swapchain<winit::window::Window>>,
    pub sampler: std::sync::Arc<vulkano::sampler::Sampler>,
    pub framebuffers: std::vec::Vec<std::sync::Arc<dyn vulkano::render_pass::FramebufferAbstract + std::marker::Send + std::marker::Sync>>,
    pub render_pass: std::sync::Arc<RenderPass>,
    // pub dynamic_state: vulkano::command_buffer::DynamicState,
    pub image_num: usize,
    pub acquire_future: Option<vulkano::swapchain::SwapchainAcquireFuture<Window>>,
    pub recreate_swapchain: bool,
    pub previous_frame_end: Option<Box<dyn vulkano::sync::GpuFuture>>,
    pub command_buffer: Option<AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>>,
    // pub command_buffer: Option<SyncCommandBufferBuilder>,
    // pub command_pool: std::sync::Arc<vulkano::command_buffer::pool::StandardCommandPool>,
    // pub command_buffer: Option<UnsafeCommandBufferBuilder>,

    pub pipe_data: PipelineData,
}

impl GraphicsContext {
    pub fn new(conf: Conf) -> (Self, winit::event_loop::EventLoop<()>) {
        let required_extensions = vulkano_win::required_extensions();

        let extensions = InstanceExtensions {
            ext_debug_utils: true,
            ..required_extensions
        };
    
        let instance =
            Instance::new(None, vulkano::Version::major_minor(1,1), &extensions, vec![]).expect("failed to create Vulkan instance");

        let physical = PhysicalDevice::enumerate(&instance).next().unwrap();
        println!(
            "Using device: {} (type: {:?})\n",
            physical.properties().device_name,
            physical.properties().device_type,
        );

        let event_loop = winit::event_loop::EventLoop::new();

        let surface = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(conf.window_mode.width, conf.window_mode.height))
            .with_min_inner_size(PhysicalSize::new(conf.window_mode.min_width, conf.window_mode.min_height))
            .with_resizable(conf.window_mode.resizable)
            .with_title(conf.window_setup.title)
            .with_maximized(conf.window_mode.maximized)
            .build_vk_surface(&event_loop, instance.clone())
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

        let pool = Device::standard_command_pool(&device, queue_family);

        let queue = queues.next().unwrap(); 

        let (swapchain, images) = {
            let caps = surface.capabilities(physical).unwrap();
            let composite_alpha = caps.supported_composite_alpha.iter().next().unwrap();
            let format = caps.supported_formats[0].0;
            let dimensions: [u32; 2] = surface.window().inner_size().into();
    
            Swapchain::start(device.clone(), surface.clone())
                .num_images(caps.min_image_count)
                .format(format)
                .dimensions(dimensions)
                .usage(ImageUsage::color_attachment())
                .sharing_mode(&queue)
                .composite_alpha(composite_alpha)
                .build()
                .unwrap()
        };

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

        // let mut dynamic_state = DynamicState::none();

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
            window_size_dependent_setup(&images, render_pass.clone(), );
            // &mut dynamic_state);

        let pipe_data = PipelineData {
            vert_buf: Arc::new(CpuAccessibleBuffer::from_data(
                device.clone(),
                BufferUsage::vertex_buffer(),
                false,
                &[0]
            ).unwrap()),
            // texture: Image::empty(),
            instance_data: None,
            descriptor_sets: Some(HashMap::new()),
        };
        
        let graphics = Self {
            queue,
            surface,
            device,
            swapchain,
            sampler,
            framebuffers,
            render_pass,
            // dynamic_state,
            image_num: 0,
            acquire_future: None,
            previous_frame_end: Some(default_future),
            recreate_swapchain: false,
            command_buffer: None,
            // command_pool: pool,
            pipe_data,
        };
    
        (graphics, event_loop)
    }

    /// Due to the nature of the command buffer and the safety requirements Vulkano tries to meet
    /// the command buffer is recreated every frame.
    pub fn create_command_buffer(&mut self,) {
        // let level = CommandBufferLevel::primary();
        // let usage = CommandBufferUsage::OneTimeSubmit;
        // unsafe {
        //     let pool_builder_alloc = self.command_pool
        //         .alloc(!matches!(level, CommandBufferLevel::Primary), 1).unwrap()
        //         .next()
        //         .expect("Requested one command buffer from the command pool, but got zero.");
        //     let inner = SyncCommandBufferBuilder::new(pool_builder_alloc.inner(), level, usage).unwrap();

        //     let builder = SyncCommandBufferBuilder::new(
        //         &pool_builder_alloc.inner(),
        //         level,
        //         usage,
        //     ).unwrap();

        //     self.command_buffer = Some(builder);
        // }
        
        let builder = AutoCommandBufferBuilder::primary(self.device.clone(), self.queue.family(), CommandBufferUsage::OneTimeSubmit,)
            .unwrap();
        self.command_buffer = Some(builder);
    }

    /// Handles setup of a new frame, called when the graphics pipeline is first created and 
    /// at the end of every frame to start the next one. 
    /// 
    /// This is necessary because the swapchain could be out of date, 
    /// as well as updating the image_num, optimality, and the swapcahin future.
    pub fn begin_frame(&mut self) {
        self.previous_frame_end.as_mut().unwrap().cleanup_finished();

        // if self.recreate_swapchain {
        //     let dimensions: [u32; 2] = self.surface.window().inner_size().into();
        //     let (new_swapchain, new_images) =
        //         match self.swapchain.recreate().dimensions(dimensions).build() {
        //             Ok(r) => r,
        //             Err(SwapchainCreationError::UnsupportedDimensions) => return,
        //             Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
        //         };

        //     self.swapchain = new_swapchain;
        //     self.framebuffers = window_size_dependent_setup(
        //         &new_images,
        //         self.render_pass.clone(),
        //         &mut self.dynamic_state,
        //         // &mut viewport,
        //     );
        //     self.recreate_swapchain = false;
        // }

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
        let clear_values = vec![[0.2, 0.2, 0.2, 1.0].into()];
        self.command_buffer.as_mut().unwrap().begin_render_pass(
            self.framebuffers[self.image_num].clone(), 
            SubpassContents::Inline, 
            clear_values,
        ).unwrap();
    }


    /// Interacts with the given shader handle (which by default is a ``` ledge_engine::graphics::shader::ShaderProgram```)
    /// to use that specific shader to draw the vertex buffer to the screen.
    /// Very scary to look at, at the moment. Due to Rust not being able to deal with the safety of looping/changing
    /// the descriptor type each loop, I had to manually right out the loop which limits the number of possible descriptors
    /// and only supports uniforms, as others will break it. Incredibly fragile and hoping to fix it later.
    pub fn draw<'a>(
        &mut self, 
        vertice_count: u32,
        vertices: Arc<dyn BufferAccess + Send + Sync>, 
        shader_handle: Arc<dyn ShaderHandle>, 
        descriptor_set: std::collections::HashMap<u32, Arc<dyn DescriptorSet + Send + Sync>>,
    ) {
        // let layout = shader_handle.layout();
        // let num_bindings = layout.num_bindings();
        
        // let descriptor: PersistentDescriptorSetBuilder<dyn PersistentDescriptorSetResources> = PersistentDescriptorSet::start(layout.clone());
        // for i in 0..num_bindings {
        //     descriptor = descriptor.add_buffer(self.pipe_data.descriptor[i].clone()).unwrap();
        // }
        let mut dimensions: [f32; 2] = [0.,0.];
        dimensions[0] = self.framebuffers[0].dimensions()[0] as f32;
        dimensions[1] = self.framebuffers[0].dimensions()[1] as f32;
        self.command_buffer.as_mut().unwrap().set_viewport(
            0,
            [Viewport {
                origin: [0.0; 2],
                dimensions: dimensions,
                depth_range: 0.0..1.0,
            }],
        );

        self.command_buffer.as_mut().unwrap().bind_pipeline_graphics(shader_handle.pipeline().clone());
        for (bind,descriptor_set) in descriptor_set {
            self.command_buffer.as_mut().unwrap().bind_descriptor_sets(
                PipelineBindPoint::Graphics,
                shader_handle.pipeline().layout().clone(),
                bind,
                descriptor_set,
            );
        }
        // let vertice_count = vertices.len() as u32;
        self.command_buffer.as_mut().unwrap().bind_vertex_buffers(0, vertices);
        self.command_buffer.as_mut().unwrap().draw(vertice_count, 1, 0, 0).unwrap();
        // shader_handle.draw(self, vertices, descriptor_set).unwrap();
    }

    /// This function submits the command buffer to the queue and fences the operation, 
    /// storing a future refering to the operation.
    /// 
    /// This function must be run once at the end of all updates and draw calls in order for the frame to be sumbitted.
    /// The context will panic if this is not called once per frame.
    pub fn present(&mut self) {
        self.command_buffer.as_mut().unwrap().end_render_pass();
        let command_buffer = self.command_buffer.take().unwrap().build().unwrap();

        let future = self.previous_frame_end
            .take().unwrap()
            .join(self.acquire_future.take().unwrap())
            .then_execute(self.queue.clone(), command_buffer).unwrap()
            .then_swapchain_present(self.queue.clone(), self.swapchain.clone(), self.image_num);

        // let future = self.previous_frame_end.take().unwrap().join(self.acquire_future.take().unwrap());

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
    }

    pub fn buffer_from<T>(&self, data: T) -> Result<Arc<CpuAccessibleBuffer<T>>, vulkano::memory::DeviceMemoryAllocError> 
    where
        T: Copy + 'static 
    {
        CpuAccessibleBuffer::from_data(
            self.device.clone(), 
            BufferUsage::all(), 
            false,
            data,
        )
    }
}

// This method is called once during initialization, then again whenever the window is resized
pub fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: Arc<RenderPass>,
    // dynamic_state: &mut DynamicState,
) -> Vec<Arc<dyn FramebufferAbstract + Send + Sync>> {
    let dimensions = images[0].dimensions();

    let viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [dimensions[0] as f32, dimensions[1] as f32],
        depth_range: 0.0..1.0,
    };
    // dynamic_state.viewports = Some(vec![viewport]);

    images
        .iter()
        .map(|image| {
            Arc::new(
                Framebuffer::start(render_pass.clone())
                    .add(ImageView::new(image.clone()).unwrap())
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

    let pixel_size_y = 1.0/window_height as f32;
    let pixel_size_x = 1.0/window_width as f32;

    let screen_width = 2.0*pixel_size_x*size[0] as f32;
    let screen_height = 2.0*pixel_size_y*size[1] as f32;

    let screen_size = [screen_width, screen_height];
    return screen_size;
}