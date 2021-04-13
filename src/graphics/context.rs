use vulkano::{
    command_buffer::{AutoCommandBufferBuilder, DynamicState},
    device::{Device, DeviceExtensions},
    framebuffer::{RenderPassAbstract},
    image::ImageUsage,
    instance::{Instance, PhysicalDevice},
    sampler::{Filter, MipmapMode, Sampler, SamplerAddressMode},
    swapchain::{
        self, AcquireError, ColorSpace, FullscreenExclusive, PresentMode, SurfaceTransform, Swapchain,
        SwapchainCreationError,
    },
    sync::{self, FlushError, GpuFuture},
    command_buffer::pool::standard::StandardCommandPoolBuilder,
    swapchain::SwapchainAcquireFuture,
    command_buffer::SubpassContents,
    instance::InstanceExtensions,
    framebuffer::{Framebuffer, FramebufferAbstract},
    image::SwapchainImage,
    pipeline::viewport::Viewport,
    image::view::ImageView,
    buffer::{BufferAccess, BufferUsage, CpuAccessibleBuffer},
    descriptor::descriptor_set::DescriptorSet,
};
use vulkano_win::VkSurfaceBuild;
use winit::{
    window::{Window, WindowBuilder},
    dpi::PhysicalSize,
};
use std::sync::Arc;
use crate::{
    conf::*,
    graphics::PipelineData,
    graphics::shader::ShaderHandle,
    graphics::image::Image,
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
    pub framebuffers: std::vec::Vec<std::sync::Arc<dyn vulkano::framebuffer::FramebufferAbstract + std::marker::Send + std::marker::Sync>>,
    pub render_pass: std::sync::Arc<dyn RenderPassAbstract + std::marker::Send + std::marker::Sync>,
    pub dynamic_state: vulkano::command_buffer::DynamicState,
    pub image_num: usize,
    pub acquire_future: Option<SwapchainAcquireFuture<Window>>,
    pub recreate_swapchain: bool,
    pub previous_frame_end: Option<Box<dyn GpuFuture>>,
    pub command_buffer: Option<AutoCommandBufferBuilder<StandardCommandPoolBuilder>>,

    pub pipe_data: std::sync::Arc<PipelineData>,
}

impl GraphicsContext {
    pub fn new(conf: Conf) -> (Self, winit::event_loop::EventLoop<()>) {
        let required_extensions = vulkano_win::required_extensions();

        let extensions = InstanceExtensions {
            ext_debug_utils: true,
            ..required_extensions
        };
    
        let instance =
            Instance::new(None, &extensions, vec![]).expect("failed to create Vulkan instance");

        let physical = PhysicalDevice::enumerate(&instance).next().unwrap();
        println!(
            "Using device: {} (type: {:?})\n",
            physical.name(),
            physical.ty(),
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

        let mut dynamic_state = DynamicState::none();

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

        let pipe_data = Arc::new(PipelineData {
            vert_buf: Arc::new(CpuAccessibleBuffer::from_data(
                device.clone(),
                BufferUsage::vertex_buffer(),
                false,
                &[0]
            ).unwrap()),
            texture: Image::empty(),
            instance_data: Arc::new(CpuAccessibleBuffer::from_data(
                device.clone(),
                BufferUsage::vertex_buffer(),
                false,
                &[0]
            ).unwrap()),
            descriptor: None,
        });
        
        let graphics = Self {
            queue,
            surface,
            device,
            swapchain,
            sampler,
            framebuffers,
            render_pass,
            dynamic_state,
            image_num: 0,
            acquire_future: None,
            previous_frame_end: Some(default_future),
            recreate_swapchain: false,
            command_buffer: None,
            pipe_data,
        };
    
        (graphics, event_loop)
    }

    /// Due to the nature of the command buffer and the safety requirements Vulkano tries to meet
    /// the command buffer is recreated every frame.
    pub fn create_command_buffer(&mut self,) {
        let builder =
        AutoCommandBufferBuilder::primary_one_time_submit(self.device.clone(), self.queue.family())
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
        let clear_values = vec![[0.2, 0.2, 0.2, 1.0].into()];
        self.command_buffer.as_mut().unwrap().begin_render_pass(self.framebuffers[self.image_num].clone(), SubpassContents::Inline, clear_values).unwrap();
    }

    /// Interacts with the given shader handle (which by default is a ``` ledge_engine::graphics::shader::ShaderProgram```)
    /// to use that specific shader to draw the vertex buffer to the screen.
    pub fn draw<'a>(&mut self, vertices: Arc<dyn BufferAccess + Send + Sync>, shader_handle: Arc<dyn ShaderHandle>, descriptor: Arc<dyn DescriptorSet + Send + Sync>) {
        shader_handle.draw(self, vertices, descriptor).unwrap();
    }

    /// This function submits the command buffer to the queue and fences the operation, 
    /// storing a future refering to the operation.
    /// 
    /// This function must be run once at the end of all updates and draw calls in order for the frame to be sumbitted.
    /// The context will panic if this is not called once per frame.
    pub fn present(&mut self) {
        self.command_buffer.as_mut().unwrap().end_render_pass().unwrap();
        let command_buffer = self.command_buffer.take().unwrap().build().unwrap();

        let future = self.previous_frame_end
            .take().unwrap()
            .join(self.acquire_future.take().unwrap())
            .then_execute(self.queue.clone(), command_buffer).unwrap()
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

        // Limit the frame rate since PresentMode::Immediate has to be used.
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