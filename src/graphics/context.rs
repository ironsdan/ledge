use vulkano::{
    command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer, SubpassContents},
    device::{Device, DeviceExtensions},
    image::ImageUsage,
    instance::{Instance},
    // descriptor_set::{SingleLayoutDescSetPool},
    // sampler::{Filter, MipmapMode, Sampler, SamplerAddressMode},
    swapchain::{self, AcquireError, Swapchain, SwapchainCreationError},
    sync::{self, FlushError, GpuFuture},
    instance::InstanceExtensions,
    render_pass::{Framebuffer, FramebufferAbstract, RenderPass},
    image::SwapchainImage,
    pipeline::{viewport::Viewport},
    image::view::ImageView,
    buffer::{BufferAccess, BufferUsage, CpuAccessibleBuffer},
    device::physical::PhysicalDevice,
};
use vulkano::command_buffer::PrimaryCommandBuffer;

use vulkano_win::VkSurfaceBuild;
use winit::{
    window::{Window, WindowBuilder},
    dpi::PhysicalSize,
};
use std::sync::Arc;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use crate::{
    conf::*,
    graphics::*,
    graphics::shader::{ShaderId, ShaderHandle},
};
use vulkano::descriptor_set::persistent::PersistentDescriptorSet;
use vulkano::descriptor_set::layout::DescriptorDescTy;
use vulkano::pipeline::{PipelineBindPoint};
use vulkano::pipeline::vertex::BuffersDefinition;

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
    pub framebuffers: std::vec::Vec<std::sync::Arc<dyn vulkano::render_pass::FramebufferAbstract + std::marker::Send + std::marker::Sync>>,
    pub render_pass: std::sync::Arc<RenderPass>,
    image_num: usize,
    pub recreate_swapchain: bool,
    previous_frame_end: Option<Box<dyn vulkano::sync::GpuFuture>>,
    present_future: Option<Box<dyn vulkano::sync::GpuFuture>>,
    pub command_buffer: Option<AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>>,
    in_render_pass: bool,
    camera: Option<Box<dyn crate::graphics::camera::Camera>>,
    camera_binding: u32,
    default_shader: ShaderId,
    current_shader: Rc<RefCell<Option<ShaderId>>>,
    pub shaders: Vec<Box<dyn crate::graphics::shader::ShaderHandle>>,
    pub set: Option<Arc<dyn DescriptorSet>>,
    pub last_frame_time: std::time::Instant,

    pub pipe_data: PipelineData,
    pub quad_vertex_buffer: [Vertex;4],
    pub vertex_buffer: std::sync::Arc<vulkano::buffer::DeviceLocalBuffer<[Vertex;2601]>>,
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

        let default_future = sync::now(device.clone()).boxed();

        let framebuffers =
            window_size_dependent_setup(&images, render_pass.clone());

        let pipe_data = PipelineData {
            vert_buf: Arc::new(CpuAccessibleBuffer::from_data(
                device.clone(),
                BufferUsage::vertex_buffer(),
                false,
                &[0]
            ).unwrap()),
            vert_count: 0,
            instance_data: None,
            descriptor_sets: HashMap::new(),
        };

        let vertex_buffer: std::sync::Arc<vulkano::buffer::DeviceLocalBuffer<[Vertex;2601]>> = 
        vulkano::buffer::device_local::DeviceLocalBuffer::new(
            device.clone(),
            BufferUsage::vertex_buffer(),
            physical.queue_families(),
        ).unwrap();
        
        let mut context = Self {
            queue: queue,
            surface: surface,
            device: device,
            swapchain: swapchain,
            framebuffers: framebuffers,
            render_pass: render_pass,
            image_num: 0,
            present_future: None,
            previous_frame_end: Some(default_future),
            recreate_swapchain: false,
            command_buffer: None,
            in_render_pass: false,
            camera: None,
            camera_binding: 0,
            default_shader: 0,
            current_shader: Rc::new(RefCell::new(None)),
            shaders: Vec::new(),
            last_frame_time: std::time::Instant::now(),
            pipe_data: pipe_data,
            quad_vertex_buffer: QUAD_VERTS,
            set: None,
            vertex_buffer,
        };

        let v_shader = vs::Shader::load(context.device.clone()).unwrap();
        let f_shader = fs::Shader::load(context.device.clone()).unwrap();

        let default_program = shader::ShaderProgram::new(
            &mut context,
            BuffersDefinition::new()
            .vertex::<Vertex>()
            .instance::<InstanceData>(),
            shader::VertexOrder::TriangleFan,
            v_shader.main_entry_point(),
            f_shader.main_entry_point(),
            BlendMode::Alpha,
        );

        context.shaders.push(Box::new(default_program));
    
        (context, event_loop)
    }

    /// Due to the nature of the command buffer and the safety requirements Vulkano tries to meet
    /// the command buffer is recreated every frame.
    fn create_command_buffer(&mut self)
    {
        self.command_buffer = Some(AutoCommandBufferBuilder::primary(
            self.device.clone(), 
            self.queue.family(), 
            CommandBufferUsage::OneTimeSubmit,
        ).unwrap());
    }

    /// Handles setup of a new frame, called when the graphics pipeline is first created and 
    /// at the end of every frame to start the next one. 
    /// 
    /// This is necessary because the swapchain could be out of date, 
    /// as well as updating the image_num, optimality, and the swapcahin future.
    pub fn begin_frame(&mut self, color: Color) {
        self.create_command_buffer();

        self.previous_frame_end.as_mut().unwrap().cleanup_finished();

        if self.recreate_swapchain {
            let dimensions: [u32; 2] = self.surface.window().inner_size().into();
            let (new_swapchain, new_images) =
                match self.swapchain.recreate().dimensions(dimensions).build() {
                    Ok(r) => r,
                    Err(SwapchainCreationError::UnsupportedDimensions) => return,
                    Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
                };

            self.swapchain = new_swapchain;
            self.framebuffers = window_size_dependent_setup(
                &new_images,
                self.render_pass.clone(),
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
        let clear_values = vec![color.as_vec().into()];
        self.command_buffer.as_mut().unwrap().begin_render_pass(
            self.framebuffers[self.image_num].clone(), 
            SubpassContents::Inline, 
            clear_values,
        ).unwrap();

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

        self.present_future = Some(self.previous_frame_end.take().unwrap().join(acquire_future).boxed());
    }


    /// Interacts with the given shader handle (which by default is a ```ledge_engine::graphics::shader::ShaderProgram```)
    /// to use that specific shader to draw the vertex buffer to the screen.
    pub fn draw<'a>(
        &mut self, 
        // vertices: Arc<dyn BufferAccess + Send + Sync>, 
        vertices: Arc<dyn BufferAccess + Send + Sync>,
        shader_handle: Arc<dyn ShaderHandle>, 
    ) {
        self.command_buffer.as_mut().unwrap().bind_pipeline_graphics(shader_handle.pipeline().clone());

        self.command_buffer.as_mut().unwrap().bind_descriptor_sets(
            PipelineBindPoint::Graphics,
            shader_handle.pipeline().layout().clone(),
            0,
            self.set.clone().unwrap(),
        );

        self.command_buffer.as_mut().unwrap().bind_vertex_buffers(0, self.vertex_buffer.clone());
        // self.command_buffer.as_mut().unwrap().draw(self.pipe_data.vert_count, 1, 0, 0).unwrap();
        shader_handle.draw(&mut self.command_buffer.as_mut().unwrap(), vertices, self.set.as_ref().unwrap().clone(), &self.pipe_data).unwrap();
    }

    /// This function submits the command buffer to the queue and fences the operation, 
    /// storing a future refering to the operation.
    /// 
    /// This function must be run once at the end of all updates and draw calls in order for the frame to be sumbitted.
    pub fn present(&mut self) {
        self.command_buffer.as_mut().unwrap().end_render_pass().unwrap();
        let command_buffer = self.command_buffer.take().unwrap().build().unwrap();

        let future = command_buffer.execute_after(self.present_future.take().unwrap(), self.queue.clone()).unwrap();
        let future = swapchain::present(self.swapchain.clone(), future, self.queue.clone(), self.image_num);
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

    pub fn bind_descriptor_sets(&mut self, shader_handle: Arc<dyn ShaderHandle>) {
        self.command_buffer.as_mut().unwrap().bind_descriptor_sets(
            PipelineBindPoint::Graphics,
            shader_handle.pipeline().layout().clone(),
            0,
            self.set.as_ref().unwrap().clone(),
        );
    }

    pub fn build_descriptor_set(&mut self, shader_handle: Arc<dyn ShaderHandle>) {
        if !self.camera.is_none() {
            let mvp = self.buffer_from(self.camera.as_ref().unwrap().as_mvp()).unwrap();
            self.create_descriptor(mvp, 0);
        }

        let mut builder = PersistentDescriptorSet::start(shader_handle.layout()[0].clone());
        for i in 0..shader_handle.layout()[0].num_bindings() {
            match shader_handle.layout()[0].descriptor(i).unwrap().ty {
                DescriptorDescTy::UniformBuffer => {builder.add_buffer(self.pipe_data.descriptor_sets.get(&i).unwrap().clone()).unwrap();},
                _ => {panic!("Unsupported descriptor type in shader.")},
            }
        }

        self.set = Some(Arc::new(builder.build().unwrap()));
    }

    pub fn create_descriptor(&mut self, buffer: Arc<dyn BufferAccess + Send + Sync>, bind: usize) {
        self.pipe_data.descriptor_sets.insert(bind as u32, buffer);
    }

    pub fn add_perspective_camera(&mut self) {
        self.camera = Some(Box::new(camera::PerspectiveCamera::new(75.0, 4.3/3.0, 5.0, 1000.0)));
    }

    pub fn add_perspective_camera_with_binding(&mut self, binding: u32) {

        self.camera = Some(Box::new(camera::PerspectiveCamera::new(75.0, 4.3/3.0, 5.0, 1000.0)));
    }

    pub fn camera(&mut self) -> &mut Box<dyn camera::Camera> {
        self.camera.as_mut().unwrap()
    }

    // pub fn update_camera(&mut self, x: f32) {
    //     self.camera().translate_x(x);
    //     let mvp = self.buffer_from(self.camera.as_ref().unwrap().as_mvp()).unwrap();
    //     let buf = self.pipe_data.descriptor_sets.get(&0).unwrap().clone();
    //     self.command_buffer.as_mut().unwrap().update_buffer(buf, Arc::new(self.camera.as_ref().unwrap().as_mvp())).unwrap();
    // }
}

// This method is called once during initialization, then again whenever the window is resized
pub fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: Arc<RenderPass>,
) -> Vec<Arc<dyn FramebufferAbstract + Send + Sync>> {
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