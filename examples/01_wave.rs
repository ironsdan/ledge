use winit::{
    event_loop::{ControlFlow},
    event::{Event, WindowEvent}
};
use ledge_engine::prelude::*;
use cgmath::{Deg, Rad, Angle};
use std::sync::Arc;

#[derive(Default, Copy, Clone)]
struct ParticleVertex {
    position: [f32; 3],
    scale: f32,
}

vulkano::impl_vertex!(ParticleVertex, position, scale);

const SEPARATION: f32 = 12.0; // Point drawing values.
const AMOUNTX: isize = 51;
const AMOUNTY: isize = 51;

fn main() {
    let (mut context, event_loop) = GraphicsContext::new(Conf::new("Wave")); // Creating a new context.

    pub mod vs { vulkano_shaders::shader! { ty: "vertex", path: "examples/shaders/particle.vert", } }
    let vs = vs::Shader::load(context.device.clone()).unwrap(); // Load shaders at compile time.

    pub mod fs { vulkano_shaders::shader! { ty: "fragment", path: "examples/shaders/particle.frag", } }
    let fs = fs::Shader::load(context.device.clone()).unwrap();

    let shader_program = Arc::new(ShaderProgram::new( // Create a new shader program.
        &mut context,
        buffer::BufferDefinition::new().vertex::<ParticleVertex>(),
        VertexOrder::PointList,
        vs.main_entry_point(), 
        fs.main_entry_point(), 
        BlendMode::Alpha
    ));

    let mut shader_material = ShaderMaterial::new(shader_program.clone()); // Load shader program into material.
    shader_material.add_uniform([1.0 as f32, 1.0 as f32, 1.0 as f32], context.device.clone());

    let mut camera = PerspectiveCamera::new(75.0, 4.3/3.0, 5.0, 1000.0); // Create and move camera.
    camera.rotate_x(Deg(20.0));
    camera.translate_z(600.0);
    
    let mvp = context.buffer_from(camera.as_mvp()).unwrap();

    let descriptor = vulkano::descriptor_set::PersistentDescriptorSet::start(shader_program.layout());
    let descriptor = descriptor.add_buffer(shader_material.uniforms[0].clone()).unwrap();
    let descriptor = descriptor.add_buffer(mvp.clone()).unwrap();
    let descriptor = Arc::new(descriptor.build().unwrap());

    let mut count = 0.0;

    event_loop.run(move |event, _, control_flow| {
        let now = std::time::Instant::now();
        
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => { *control_flow = ControlFlow::Exit; },
                WindowEvent::Resized(_) => { context.recreate_swapchain = true; },
                _ => {},
            },
            Event::MainEventsCleared => { 
                let particles = update(&mut context, &mut count);
            
                context.create_command_buffer();
                context.begin_frame();
                context.draw(particles.clone(), shader_material.shader_program.clone(), descriptor.clone());
                context.present();

                let sleep_time = std::time::Duration::from_secs_f64(0.016).checked_sub(now.elapsed());
                if let Some(value) = sleep_time { std::thread::sleep(value); }
                print!("{:.2}\r", now.elapsed().as_secs_f32() * 1000.0);
            },
            _ => {}
        }
    });
}

fn update(context: &mut GraphicsContext, count: &mut f32) -> 
    std::sync::Arc<buffer::CpuBuffer<[ParticleVertex; (AMOUNTX * AMOUNTY) as usize]>> 
{
    let mut i = 0;
    let mut data = [ParticleVertex::default(); (AMOUNTX * AMOUNTY) as usize];
    
    for ix in 0..AMOUNTX {
        for iy in 0..AMOUNTY {
            let sin_ix = Rad((ix as f32 + *count) * 0.5).sin();
            let sin_iy = Rad((iy as f32 + *count) * 0.5).sin();

            data[i].position[0] = (ix as f32 * SEPARATION) - (((AMOUNTX as f32 * SEPARATION) / 2.0));
            data[i].position[1] = ( sin_ix * 10.0) + ( sin_iy * 10.0);
            data[i].position[2] = (iy as f32 * SEPARATION) - (((AMOUNTY as f32 * SEPARATION) / 2.0));
            data[i].scale = ( sin_ix + 1.5 ) * 2.0 + ( sin_iy + 1.5 ) * 2.0;

            i += 1;
        }
    }
    *count += 0.03;
    buffer::CpuBuffer::from_data(context.device.clone(), buffer::BufferUsage::vertex_buffer(), false, data).unwrap()
}