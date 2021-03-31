// use vulkano::{
//     command_buffer::{AutoCommandBufferBuilder, DynamicState},
//     device::{Device, DeviceExtensions},
//     framebuffer::{Subpass, RenderPassAbstract},
//     image::ImageUsage,
//     instance::{Instance, PhysicalDevice},
//     pipeline::{GraphicsPipeline, GraphicsPipelineAbstract},
//     swapchain::{
//         ColorSpace, FullscreenExclusive, PresentMode, SurfaceTransform, Swapchain,
//     },
//     sync::{self, GpuFuture},
//     command_buffer::pool::standard::StandardCommandPoolBuilder,
//     swapchain::SwapchainAcquireFuture,
//     instance::{InstanceExtensions},
// };

// use vulkano::pipeline::vertex::OneVertexOneInstanceDefinition;

// use vulkano::{
//     framebuffer::{Framebuffer, FramebufferAbstract},
//     image::SwapchainImage,
//     pipeline::viewport::Viewport,
// };
// use vulkano_win::VkSurfaceBuild;
// use winit::{
//     window::{Window, WindowBuilder},
//     dpi::PhysicalSize,
// };
// use std::sync::Arc;

// use crate::{
//     graphics::{Vertex, InstanceData, shader::PipelineObjectSet, shader::PipelineObject, BlendMode},
//     conf::*,
//     graphics::{vs, fs},
// };


// // #[derive(Default)]

// pub struct GraphicsBackend {
//     pub queue: std::sync::Arc<vulkano::device::Queue>,
//     pub swapchain: std::sync::Arc<vulkano::swapchain::Swapchain<winit::window::Window>>,
//     pub framebuffers: std::vec::Vec<std::sync::Arc<dyn vulkano::framebuffer::FramebufferAbstract + std::marker::Send + std::marker::Sync>>,
//     pub render_pass: std::sync::Arc<dyn RenderPassAbstract + std::marker::Send + std::marker::Sync>,
//     pub dynamic_state: vulkano::command_buffer::DynamicState,
//     pub image_num: usize,
//     pub acquire_future: Option<SwapchainAcquireFuture<Window>>,
//     pub recreate_swapchain: bool,
//     pub previous_frame_end: Option<Box<dyn GpuFuture>>,
//     pub command_buffer: Option<AutoCommandBufferBuilder<StandardCommandPoolBuilder>>,
// }

// impl GraphicsBackend {
//     pub fn new(event_loop: &winit::event_loop::EventLoop<()>, conf: Conf) -> Self{
//         let required_extensions = vulkano_win::required_extensions();

//         let extensions = InstanceExtensions {
//             ext_debug_utils: true,
//             ..required_extensions
//         };
    
//         let instance =
//             Instance::new(None, &extensions, vec![]).expect("failed to create Vulkan instance");

//         let physical = PhysicalDevice::enumerate(&instance).next().unwrap();
//         println!(
//             "Using device: {} (type: {:?})\nExtensions: {:?}",
//             physical.name(),
//             physical.ty(),
//             required_extensions,
//         );

//         let surface = WindowBuilder::new()
//             .with_inner_size(PhysicalSize::new(conf.window_mode.width, conf.window_mode.height))
//             .with_min_inner_size(PhysicalSize::new(conf.window_mode.min_width, conf.window_mode.min_height))
//             .with_resizable(conf.window_mode.resizable)
//             .with_title(conf.window_setup.title)
//             .with_maximized(conf.window_mode.maximized)
//             .build_vk_surface(event_loop, instance.clone())
//             .unwrap();

//         let queue_family = physical
//             .queue_families()
//             .find(|&q| q.supports_graphics() && surface.is_supported(q).unwrap_or(false))
//             .unwrap();

//         let device_ext = DeviceExtensions {
//             khr_swapchain: true,
//             ..DeviceExtensions::none()
//         };
        
//         let (device, mut queues) = Device::new(
//             physical,
//             physical.supported_features(),
//             &device_ext,
//             [(queue_family, 0.5)].iter().cloned(),
//         )
//         .unwrap();

//         let queue = queues.next().unwrap(); 

//         let (swapchain, images) = {
//             let caps = surface.capabilities(physical).unwrap();
//             let alpha = caps.supported_composite_alpha.iter().next().unwrap();
//             let format = caps.supported_formats[0].0;
//             let dimensions: [u32; 2] = surface.window().inner_size().into();

//             Swapchain::new(
//                 device.clone(),
//                 surface.clone(),
//                 caps.min_image_count,
//                 format,
//                 dimensions,
//                 1,
//                 ImageUsage::color_attachment(),
//                 &queue,
//                 SurfaceTransform::Identity,
//                 alpha,
//                 PresentMode::Immediate, // This is definitely not great. But keeps from frame spikes
//                 FullscreenExclusive::Default,
//                 true,
//                 ColorSpace::SrgbNonLinear,
//             )
//             .unwrap()
//         };
        
//         let render_pass = Arc::new(
//             vulkano::single_pass_renderpass!(device.clone(),
//                 attachments: {
//                     color: {
//                         load: Clear,
//                         store: Store,
//                         format: swapchain.format(),
//                         samples: 1,
//                     }
//                 },
//                 pass: {
//                     color: [color],
//                     depth_stencil: {}
//                 }
//             )
//             .unwrap(),
//         );



//         let pipeline = Arc::new(
//             GraphicsPipeline::start()
//                 // .vertex_input_single_buffer::<Vertex>()
//                 .vertex_input(OneVertexOneInstanceDefinition::<Vertex, InstanceData>::new())
//                 .vertex_shader(vs.main_entry_point(), ())
//                 .triangle_strip()
//                 .viewports_dynamic_scissors_irrelevant(1)
//                 .fragment_shader(fs.main_entry_point(), ())
//                 .blend_alpha_blending()
//                 .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
//                 .build(device.clone())
//                 .unwrap()
//         ) as Arc<dyn GraphicsPipelineAbstract + Send + Sync>;

//         let pipeline_object = PipelineObject::new(pipeline);
//         let mut pipeline_sets = PipelineObjectSet::new(128);
//         pipeline_sets.insert(BlendMode::Alpha, pipeline_object);
        
//         let mut dynamic_state = DynamicState {
//             line_width: None,
//             viewports: None,
//             scissors: None,
//             compare_mask: None,
//             write_mask: None,
//             reference: None,
//         };

//         let default_future = sync::now(device.clone()).boxed();

//         let framebuffers =
//             window_size_dependent_setup(&images, render_pass.clone(), &mut dynamic_state);

//         Self {
//             queue: queue,
//             swapchain: swapchain,
//             framebuffers: framebuffers,
//             render_pass: render_pass,
//             dynamic_state: dynamic_state,
//             image_num: 0,
//             acquire_future: None,
//             previous_frame_end: Some(default_future),
//             recreate_swapchain: false,
//             command_buffer: None,
//         }
//     }

// }

// // This method is called once during initialization, then again whenever the window is resized
// pub fn window_size_dependent_setup(
//     images: &[Arc<SwapchainImage<Window>>],
//     render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
//     dynamic_state: &mut DynamicState,
// ) -> Vec<Arc<dyn FramebufferAbstract + Send + Sync>> {
//     let dimensions = images[0].dimensions();

//     let viewport = Viewport {
//         origin: [0.0, 0.0],
//         dimensions: [dimensions[0] as f32, dimensions[1] as f32],
//         depth_range: 0.0..1.0,
//     };
//     dynamic_state.viewports = Some(vec![viewport]);

//     images
//         .iter()
//         .map(|image| {
//             Arc::new(
//                 Framebuffer::start(render_pass.clone())
//                     .add(image.clone())
//                     .unwrap()
//                     .build()
//                     .unwrap(),
//             ) as Arc<dyn FramebufferAbstract + Send + Sync>
//         })
//         .collect::<Vec<_>>()
// }