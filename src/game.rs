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

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit::{dpi::Size,dpi::PhysicalSize};
use winit_input_helper::WinitInputHelper;

use png;
use std::io::Cursor;
use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::SystemTime;

use crate::lib::*;
use crate::entity::*;
use crate::physics::*;

pub struct Game {
    pub event_loop: Option<winit::event_loop::EventLoop<()>>,
    pub queue: std::sync::Arc<vulkano::device::Queue>,
    pub surface: std::sync::Arc<vulkano::swapchain::Surface<winit::window::Window>>,
    pub device: std::sync::Arc<vulkano::device::Device>,
    pub swapchain: std::sync::Arc<vulkano::swapchain::Swapchain<winit::window::Window>>,
    pub sampler: std::sync::Arc<vulkano::sampler::Sampler>,
    pub framebuffers: std::vec::Vec<std::sync::Arc<dyn vulkano::framebuffer::FramebufferAbstract + std::marker::Send + std::marker::Sync>>,
    pub render_pass: std::sync::Arc<dyn RenderPassAbstract + std::marker::Send + std::marker::Sync>,
    pub dynamic_state: vulkano::command_buffer::DynamicState,
    pub buffer_pool: vulkano::buffer::CpuBufferPool<Vertex>,
    pub pipeline: std::sync::Arc<dyn vulkano::pipeline::GraphicsPipelineAbstract + std::marker::Send + std::marker::Sync>
}

impl Game {
    pub fn initialize_vulkan() -> Self {
        let required_extensions = vulkano_win::required_extensions();
        let instance = Instance::new(None, &required_extensions, None).unwrap();
        let physical = PhysicalDevice::enumerate(&instance).next().unwrap();
        println!(
            "Using device: {} (type: {:?})",
            physical.name(),
            physical.ty()
        );

        let size_h_w = PhysicalSize::new(800, 600);
        let size: Size = Size::Physical(size_h_w);

        let event_loop = EventLoop::new();
        let surface = WindowBuilder::new().with_inner_size(size)
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

        Self {
            event_loop: Some(event_loop),
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
        }
    }

    pub fn run(mut self) {
        let (texture, tex_future) = {
            let png_bytes = include_bytes!("SweaterGuy.png").to_vec();
            let cursor = Cursor::new(png_bytes);
            let decoder = png::Decoder::new(cursor);
            let (info, mut reader) = decoder.read_info().unwrap();
            let dimensions = Dimensions::Dim2d {
                width: info.width,
                height: info.height,
            };
            let mut image_data = Vec::new();
            image_data.resize((info.width * info.height * 4) as usize, 0);
            reader.next_frame(&mut image_data).unwrap();
    
            ImmutableImage::from_iter(
                image_data.iter().cloned(),
                dimensions,
                Format::R8G8B8A8Srgb,
                self.queue.clone(),
            )
            .unwrap()
        };

        let handler = self.event_loop.take().unwrap();
        let mut recreate_swapchain = false;
        let mut previous_frame_end = Some(tex_future.boxed());
    
        let mut collision_world = CollisionWorld::<Entity, Entity>::new();
    
        let player = Entity::new("Dan".to_string(), 1, [0.0, 0.0], texture.clone(), [0.0, 0.0], [16.0,22.0]);
        let player2 = Entity::new("Dan2".to_string(), 1, [0.0, 0.0], texture.clone(), [0.0, 0.0], [16.0,22.0]);
        let player_ref = Rc::new(RefCell::new(player));
    
        collision_world.entities.push(Rc::clone(&player_ref));
    
        let mut input = WinitInputHelper::new();

        let layout = self.pipeline.descriptor_set_layout(0).unwrap();

        let set = Arc::new(
            PersistentDescriptorSet::start(layout.clone())
                .add_sampled_image(texture.clone(), self.sampler.clone())
                .unwrap()
                .build()
                .unwrap(),
        );
    
        let mut timestep: f32 = 0.0;
        handler.run(move |event, _, control_flow| {
            player_ref.borrow_mut().horizontal_move = false;
            if input.update(&event) {
                let key_w_released = input.key_released(winit::event::VirtualKeyCode::W);
                let key_w_pressed = input.key_pressed(winit::event::VirtualKeyCode::W);
                let key_a = input.key_held(winit::event::VirtualKeyCode::A);
                let key_d = input.key_held(winit::event::VirtualKeyCode::D);
    
                if key_w_pressed {
                    player_ref.borrow_mut().take_input(MovementInput::UpPress);
                }
                if key_w_released {
                    player_ref.borrow_mut().take_input(MovementInput::UpRelease);
                }
                if key_a {
                    player_ref.borrow_mut().take_input(MovementInput::Left);
                }
                if key_d {
                    player_ref.borrow_mut().take_input(MovementInput::Right);
                }
            }
            
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    ..
                } => {
                    recreate_swapchain = true;
                }
                Event::MainEventsCleared => {
                    collision_world.step(timestep);
                }
                Event::RedrawRequested(_) => {
                    let start = SystemTime::now();
                    previous_frame_end.as_mut().unwrap().cleanup_finished();
    
                    if recreate_swapchain {
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
                        recreate_swapchain = false;
                    }
            
                    let (image_num, suboptimal, acquire_future) =
                        match swapchain::acquire_next_image(self.swapchain.clone(), None) {
                            Ok(r) => r,
                            Err(AcquireError::OutOfDate) => {
                                recreate_swapchain = true;
                                return;
                            }
                            Err(e) => panic!("Failed to acquire next image: {:?}", e),
                        };
            
                    if suboptimal {
                        recreate_swapchain = true;
                    }
            
                    // let clear_values = vec![[0.2, 0.2, 0.2, 1.0].into()];
            
                    let mut sprites_to_render: Vec<vulkano::buffer::cpu_pool::CpuBufferPoolChunk<Vertex, std::sync::Arc<_>>> = Vec::new();
                    let data = &player_ref.borrow().sprite.rect;
                    let data2 = &player2.sprite.rect;
    
            
                    // Allocate a new chunk from buffer_pool
                    let vertex_buffer = self.buffer_pool.chunk(data.to_vec()).unwrap();
                    let vertex_buffer2 = self.buffer_pool.chunk(data2.to_vec()).unwrap();
    
                    sprites_to_render.push(vertex_buffer);
                    sprites_to_render.push(vertex_buffer2);
            
                    let mut builder =
                        AutoCommandBufferBuilder::primary_one_time_submit(self.device.clone(), self.queue.family())
                            .unwrap();
    
                    let clear_values = vec![[0.2, 0.2, 0.2, 1.0].into()];
                    builder.begin_render_pass(self.framebuffers[image_num].clone(), false, clear_values).unwrap();
                            
                    for sprite in sprites_to_render.iter() {
                        builder.draw(
                            self.pipeline.clone(),
                            &self.dynamic_state,
                            vec!(Arc::new(sprite.clone())),
                            set.clone(),
                            (),
                        ).unwrap();
                    }
                    builder.end_render_pass().unwrap();
                    let command_buffer = builder.build().unwrap();
            
                    let future = previous_frame_end
                        .take()
                        .unwrap()
                        .join(acquire_future)
                        .then_execute(self.queue.clone(), command_buffer)
                        .unwrap()
                        .then_swapchain_present(self.queue.clone(), self.swapchain.clone(), image_num)
                        .then_signal_fence_and_flush();
            
                    match future {
                        Ok(future) => {
                            previous_frame_end = Some(future.boxed());
                        }
                        Err(FlushError::OutOfDate) => {
                            recreate_swapchain = true;
                            previous_frame_end = Some(sync::now(self.device.clone()).boxed());
                        }
                        Err(e) => {
                            println!("Failed to flush future: {:?}", e);
                            previous_frame_end = Some(sync::now(self.device.clone()).boxed());
                        }
                    };
                    
                    timestep = start.elapsed().unwrap().as_millis() as f32;
                    // print!("Redraw: ");
                    self.surface.window().request_redraw();
                }
                Event::RedrawEventsCleared => {
                    // print!("Cleared: ");
                }
                _ => {
                    // print!("Other: ");
                },
            }
            // println!("{:?}", start_entire.elapsed().unwrap());
        });
    }
}