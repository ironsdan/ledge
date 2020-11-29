
use vulkano::buffer::CpuBufferPool;
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::device::{Device, DeviceExtensions};
use vulkano::format::Format;
use vulkano::framebuffer::Subpass;
use vulkano::image::{Dimensions, ImageUsage, ImmutableImage};
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::pipeline::GraphicsPipeline;
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
use std::time::{Duration, SystemTime};
use std::thread;

mod lib;
use lib::*;

fn main() {
    // The start of this example is exactly the same as `triangle`. You should read the
    // `triangle` example if you haven't done so yet.

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

    let (mut swapchain, images) = {
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

    let dimensions = images[0].dimensions();
    let aspect = dimensions[1] as f32 / dimensions[0] as f32;

    let char_width = 18.0*aspect;
    let char_height = 22.0;

    let act_width = 0.15;
    let act_height = (char_height / char_width) * act_width;

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
            queue.clone(),
        )
        .unwrap()
    };

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
    );

    let layout = pipeline.layout().descriptor_set_layout(0).unwrap();
    
    let set = Arc::new(
        PersistentDescriptorSet::start(layout.clone())
            .add_sampled_image(texture.clone(), sampler.clone())
            .unwrap()
            .build()
            .unwrap(),
    );

    let mut dynamic_state = DynamicState {
        line_width: None,
        viewports: None,
        scissors: None,
        compare_mask: None,
        write_mask: None,
        reference: None,
    };

    let mut framebuffers =
        window_size_dependent_setup(&images, render_pass.clone(), &mut dynamic_state);

    let mut recreate_swapchain = false;
    let mut previous_frame_end = Some(tex_future.boxed());

    let mut player_y = 0.0;
    let mut player_x = 0.0;

    let mut input = WinitInputHelper::new();

    let mut total_time: f32 = 0.0;
    let mut grounded = false;
    let mut timestep: f32 = 0.0;
    let mut position: Vec<f32> = vec![0.0, 0.0];
    let mut v: Vec<f32> = vec![0.0,0.0];
    let a: Vec<f32> = vec![0.0, -0.0000008];
    let mut jump_cool = 20;
    let mut horizontal_move = false;
    let mut left = false;
    let mut right = true;
    let mut jump_cleared = true;

    event_loop.run(move |event, _, control_flow| {
        let start_entire = SystemTime::now();
        let mut texture_coord_options = [
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 0.0],
            [0.0, 1.0],
        ];
        let player = Character::new("Dan".to_string(), 1, [0.0, 0.0], texture.clone(), [
                Vertex {
                    position: [-(act_width / 2.0)*1.0, -(act_height / 2.0)/1.0],
                    tex_coords: texture_coord_options[0],
                    
                },
                Vertex {
                    position: [-(act_width / 2.0)*1.0, (act_height / 2.0)/1.0],
                    
                    tex_coords: texture_coord_options[1],
                },
                Vertex {
                    position: [(act_width / 2.0)*1.0, -(act_height / 2.0)/1.0],
                    tex_coords: texture_coord_options[2],
                },
                Vertex {
                    position: [(act_width / 2.0)*1.0, (act_height / 2.0)/1.0],
                    tex_coords: texture_coord_options[3],
                },
            ]
        );

        if jump_cool > 0 {
            jump_cool -= 1;
        }

        horizontal_move = false;
        if v[0] > 0.0005 {
            v[0] = 0.0005;
        } else if v[0] < -0.0005 {
            v[0] = -0.0005;
        }

        position[0] += timestep * (v[0] + timestep * a[0]/2.0);
        v[0] += timestep * a[0];
        player_x = position[0];

        if !grounded {
            total_time += timestep;
            
            position[1] -= timestep * (v[1] + timestep * a[1]/2.0);
            
            v[1] += timestep * a[1];
            if v[1] < -0.0008 {
                v[1] = -0.0008;
            }
            
            player_y = position[1];
        }

        if player_y+(act_height/2.0) > 1.0 {
            grounded = true;
            // player_y = 1.0-(act_height / 2.0);
            v[1] = 0.0;
        }

        // println!("{} {} {} {} {} {}", v[0], v[1], player_y, player_x, grounded, jump_cool);

        if input.update(&event) {
            let key_w_released = input.key_released(winit::event::VirtualKeyCode::W);
            let key_w_pressed = input.key_pressed(winit::event::VirtualKeyCode::W);
            let key_a = input.key_held(winit::event::VirtualKeyCode::A);
            // let key_s = input.key_held(winit::event::VirtualKeyCode::S);
            let key_d = input.key_held(winit::event::VirtualKeyCode::D);
            // println!("w: {}, a: {}, s: {}, d: {}", key_w, key_a, key_s, key_d);
            if key_w_pressed {
                if jump_cleared && grounded && jump_cool == 0 {
                    v[1] = 0.0005;
                    grounded = false;
                    jump_cleared = false;
                }
            }
            if key_w_released {
                jump_cleared = true;
            }
            if key_a {
                if grounded {
                    v[0] -= 0.005;
                } else {
                    v[0] -= 0.005;
                }
                left = true;
                right = false;
                horizontal_move = true;
            }
            if key_d {
                if grounded {
                    v[0] += 0.005;
                } else {
                    v[0] += 0.005;
                }
                right = true;
                left = false;
                horizontal_move = true;
            }
        }

        if !horizontal_move {
            v[0] = 0.0;
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
            Event::RedrawRequested(_) => {
                let start = SystemTime::now();
                previous_frame_end.as_mut().unwrap().cleanup_finished();

                if recreate_swapchain {
                    let dimensions: [u32; 2] = surface.window().inner_size().into();
                    let (new_swapchain, new_images) =
                        match swapchain.recreate_with_dimensions(dimensions) {
                            Ok(r) => r,
                            Err(SwapchainCreationError::UnsupportedDimensions) => return,
                            Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
                        };
        
                    swapchain = new_swapchain;
                    framebuffers = window_size_dependent_setup(
                        &new_images,
                        render_pass.clone(),
                        &mut dynamic_state,
                    );
                    recreate_swapchain = false;
                }
        
                let (image_num, suboptimal, acquire_future) =
                    match swapchain::acquire_next_image(swapchain.clone(), None) {
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
        
                let clear_values = vec![[0.2, 0.2, 0.2, 1.0].into()];
        
                let data = player.sprite.rect;
        
                // Allocate a new chunk from buffer_pool
                let vertex_buffer = buffer_pool.chunk(data.to_vec()).unwrap();
        
                let mut builder =
                    AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family())
                        .unwrap();
                builder
                    .begin_render_pass(framebuffers[image_num].clone(), false, clear_values)
                    .unwrap()
                    .draw(
                        pipeline.clone(),
                        &dynamic_state,
                        vertex_buffer.clone(),
                        set.clone(),
                        (),
                    )
                    .unwrap()
                    .end_render_pass()
                    .unwrap();
                let command_buffer = builder.build().unwrap();
        
                let future = previous_frame_end
                    .take()
                    .unwrap()
                    .join(acquire_future)
                    .then_execute(queue.clone(), command_buffer)
                    .unwrap()
                    .then_swapchain_present(queue.clone(), swapchain.clone(), image_num)
                    .then_signal_fence_and_flush();
        
                match future {
                    Ok(future) => {
                        previous_frame_end = Some(future.boxed());
                    }
                    Err(FlushError::OutOfDate) => {
                        recreate_swapchain = true;
                        previous_frame_end = Some(sync::now(device.clone()).boxed());
                    }
                    Err(e) => {
                        println!("Failed to flush future: {:?}", e);
                        previous_frame_end = Some(sync::now(device.clone()).boxed());
                    }
                };
                
                if start.elapsed().unwrap() < Duration::from_millis(16) {
                    // println!("{:?}", Duration::from_millis(16) - start.elapsed().unwrap());
                    thread::sleep(Duration::from_millis(15));
                }
                // println!("{:?}", start.elapsed().unwrap());
                timestep = 16 as f32;
            }
            Event::MainEventsCleared => {
                print!("main ");
            }
            _ => {
                print!("none ");
            },
        }
        println!("{:?}", start_entire.elapsed().unwrap());
    });
}