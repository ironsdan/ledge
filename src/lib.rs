//! # About
//! 
//! [ledge_engine](https://github.com/DanielIrons/ledge_engine) is a Rust graphics backend abstraction library for [Vulkano](http://vulkano.rs/guide/introduction) 
//! which is a pure rust safe wrapper for [Vulkan](https://www.khronos.org/vulkan/).
//! 
//! This framework is loosely based on many different existing frameworks including: ggez, Amethyst, and Three.js.
//! I found that the number of game development libraries for Vulkan was small, and the lower level Rust abstractions were one of the following:
//! too complicated to read in a concise way ([gfx-rs](https://github.com/gfx-rs/gfx)), not easily suited for game development, separated into multiple necessary packages and/or in rigorous development (Vulkano, Winit, etc.).
//! 
//! The library will one day contain useful abstractions and provide many useful systems such as sound, 2D (and maybe some simple 3D) drawing,
//! resource management, and event handling.
//! 
//! # Usage
//! ledge_engine contains a few modules to help the user get up and running quickly, namely the ```graphics``` and ```audio``` modules,
//! that handle drawing or interfacing with the backend and handling audio production respectively.
//! 
//! # Basic Graphics Example
//! ```
//! use winit::{
//!     event_loop::{ControlFlow},
//!     event::{Event, WindowEvent}
//! };
//! use vulkano::{ // TODO remove vulkano references
//!     descriptor::descriptor_set::PersistentDescriptorSet,
//!     buffer::{BufferUsage, CpuAccessibleBuffer},
//!     pipeline::vertex::SingleBufferDefinition
//! };
//! use std::sync::Arc;
//! use ledge_engine::graphics::{
//!     buffer::*,
//!     camera::PerspectiveCamera,
//!     shader::{Shader, ShaderProgram, PipelineObject, VertexOrder,},
//!     context::GraphicsContext,
//!     BlendMode,
//! };
//! use ledge_engine::conf::Conf;
//! 
//! #[derive(Default, Copy, Clone)]
//! struct Vertex {
//!     position: [f32; 3],
//! }
//! 
//! vulkano::impl_vertex!(Vertex, position);
//! 
//! pub mod vs {
//!     vulkano_shaders::shader! {
//!         ty: "vertex",
//!         path: "examples/shaders/shader.vert",
//!     }
//! }
//! 
//! pub mod fs {
//!     vulkano_shaders::shader! {
//!         ty: "fragment",
//!         path: "examples/shaders/shader.frag",
//!     }
//! }
//! 
//! #[derive(Clone, Copy)]
//! struct CameraMvp {
//!     model: [[f32; 4]; 4],
//!     view: [[f32; 4]; 4],
//!     proj: [[f32; 4]; 4],
//! }
//! 
//! fn main() {
//!     let (mut context, event_loop) = GraphicsContext::new(Conf::new("Wave"));
//! 
//!     let vs = vs::Shader::load(context.device.clone()).unwrap();
//!     let fs = fs::Shader::load(context.device.clone()).unwrap();
//! 
//!     let vertex_shader = Shader::new(vs.main_entry_point(), ());
//!     let fragment_shader = Shader::new(fs.main_entry_point(), ());
//! 
//!     let po = Arc::new(PipelineObject::new(
//!         &mut context, 
//!         SingleBufferDefinition::<Vertex>::new(), 
//!         VertexOrder::TriangleList,
//!         vertex_shader, 
//!         fragment_shader, 
//!         BlendMode::Alpha
//!     ));
//! 
//!     let shader_program = Arc::new(ShaderProgram::new(BlendMode::Alpha, po.clone()));
//! 
//!     let camera = PerspectiveCamera::new(75.0, 4.3/3.0, 5.0, 2000.0);
//! 
//!     let color = BufferAttribute::from_data(
//!         [1.0 as f32, 1.0 as f32, 1.0 as f32], 
//!         context.device.clone()
//!     );
//! 
//!     let mvp_data = CameraMvp {
//!         model: camera.model_array(),
//!         view: camera.view_array(),
//!         proj: camera.proj_array(),
//!     };
//! 
//!     let mvp = BufferAttribute::from_data(
//!         mvp_data, 
//!         context.device.clone()
//!     );
//! 
//!     let triangle = Arc::new(CpuAccessibleBuffer::from_data(
//!         context.device.clone(), 
//!         BufferUsage::vertex_buffer(), 
//!         false, 
//!         [
//!             Vertex {
//!                 position: [0.0, 0.0, 200.0],
//!             },
//!             Vertex {
//!                 position: [50.0, 0.0, 200.0],
//!             },
//!             Vertex {
//!                 position: [50.0, -100.0, 200.0],
//!             },
//!         ]
//!     ).unwrap());
//! 
//!     let descriptor = Arc::new(
//!         PersistentDescriptorSet::start(po.pipeline.descriptor_set_layout(0).unwrap().clone())
//!             .add_buffer(color.inner.clone()).unwrap() 
//!             .add_buffer(mvp.inner.clone()).unwrap()
//!             .build()
//!             .unwrap(),
//!     );
//! 
//!     event_loop.run(move |event, _, control_flow| {
//!         let context = &mut context;
//!         let now = std::time::Instant::now();
//!    
//!         match event {
//!             Event::WindowEvent { event, .. } => match event {
//!                 WindowEvent::CloseRequested => {
//!                     *control_flow = ControlFlow::Exit;
//!                 },
//!                 WindowEvent::Resized(_) => {
//!                     context.recreate_swapchain = true;
//!                 },
//!                 _ => {},
//!             },
//!             Event::MainEventsCleared => { 
//!                 context.create_command_buffer();
//! 
//!                 context.draw(triangle.clone(), shader_program.clone(), descriptor.clone());
//! 
//!                 context.present();
//! 
//!                 let mut sleep_time: f64 = 0.016 - now.elapsed().as_secs_f64();
//!                 if sleep_time < 0.0 {
//!                     sleep_time = 0.0
//!                 }
//! 
//!                 std::thread::sleep(std::time::Duration::from_secs_f64(sleep_time));
//!                 print!("{:.2}\r", now.elapsed().as_secs_f32() * 1000.0);
//!             },
//!             _ => {}
//!         }
//!     });
//! }
//! ```


/// The ```graphics``` module handles all drawing operations for any type implementing the ```Drawable``` trait.
pub mod graphics;
/// Graphics and other configuration options.
pub mod conf;
/// TODO: This module will one day be the interface to the filesystem and 
/// be a storage and loader device for images, and other file types.
// pub mod asset;
/// The ```input```module handles inputs from various different peripherals and passes has structs to  sto the current state.
// pub mod input;
/// TODO: Add some audio module.
// pub mod audio;
/// A module that stores timing data.
// pub mod timer;

// pub mod scene;

pub mod prelude {
    pub use crate::*;
    // pub use crate::graphics::camera::*;
    // // pub use crate::graphics::context::*;
    // pub use crate::graphics::shader::*;
    // pub use crate::graphics::material::*;

    // pub use crate::conf::*;
}