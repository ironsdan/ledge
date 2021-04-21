use ledge_engine::graphics::buffer::*;
use winit::{
    event_loop::{ControlFlow},
    event::{Event, WindowEvent}
};
use vulkano::{
    descriptor::descriptor_set::PersistentDescriptorSet,
    buffer::{BufferUsage, CpuAccessibleBuffer},
};
use cgmath::{Deg, Rad, Angle};
use std::sync::Arc;
use ledge_engine::graphics::camera::PerspectiveCamera;
use ledge_engine::graphics::shader::PipelineObject;
use ledge_engine::graphics::shader::Shader;
use ledge_engine::graphics::context::GraphicsContext;
use ledge_engine::conf::Conf;
use ledge_engine::graphics::BlendMode;
use ledge_engine::graphics::shader::ShaderProgram;
use vulkano::pipeline::vertex::SingleBufferDefinition;
use ledge_engine::graphics::shader::VertexOrder;

#[derive(Default, Copy, Clone)]
struct ParticleVertex {
    position: [f32; 3],
    scale: f32,
}

vulkano::impl_vertex!(ParticleVertex, position, scale);

pub mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "examples/shaders/particle.vert",
    }
}
pub mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "examples/shaders/particle.frag",
    }
}

pub mod vs2 {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "examples/shaders/particle-alt.vert",
    }
}

pub mod fs2 {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "examples/shaders/particle-alt.frag",
    }
}

#[derive(Clone, Copy)]
#[allow(unused)]
struct CameraMvp {
    model: [[f32; 4]; 4],
    view: [[f32; 4]; 4],
    proj: [[f32; 4]; 4],
}

const SEPARATION: f32 = 12.0;
const AMOUNTX: isize = 50;
const AMOUNTY: isize = 50;

fn main() {
    let (mut context, event_loop) = GraphicsContext::new(Conf::new("Level"));

    let vs = vs::Shader::load(context.device.clone()).unwrap();
    let fs = fs::Shader::load(context.device.clone()).unwrap();

    let vs2 = vs2::Shader::load(context.device.clone()).unwrap();
    let fs2 = fs2::Shader::load(context.device.clone()).unwrap();

    let shader_program = Arc::new(ShaderProgram::new(
        &mut context, 
        SingleBufferDefinition::<ParticleVertex>::new(), 
        VertexOrder::PointList,
        Shader::new(vs.main_entry_point(), ()), 
        Shader::new(fs.main_entry_point(), ()),  
        BlendMode::Alpha
    ));

    let shader_program2 = Arc::new(ShaderProgram::new(
        &mut context, 
        SingleBufferDefinition::<ParticleVertex>::new(), 
        VertexOrder::PointList,
        Shader::new(vs2.main_entry_point(), ()), 
        Shader::new(fs2.main_entry_point(), ()), 
        BlendMode::Alpha
    ));

    let mut camera = PerspectiveCamera::new(75.0, 4.3/3.0, 5.0, 1000.0);
    camera.rotate_x(Deg(20.0));
    camera.translate_z(600.0);

    let color = BufferAttribute::from_data(
        [1.0 as f32, 1.0 as f32, 1.0 as f32], 
        context.device.clone()
    );
    
    let mvp_data = CameraMvp {
        model: camera.model_array(),
        view: camera.view_array(),
        proj: camera.proj_array(),
    };
    
    let mvp = BufferAttribute::from_data(
        mvp_data, 
        context.device.clone()
    );

    let descriptor = Arc::new(
        PersistentDescriptorSet::start(shader_program.layout().clone())
            .add_buffer(color.inner.clone()).unwrap()
            .add_buffer(mvp.inner.clone()).unwrap()
            .build()
            .unwrap(),
    );

    let mut count = 0.0;
    let mut i = 0;

    event_loop.run(move |event, _, control_flow| {
        let context = &mut context;
        let now = std::time::Instant::now();
        
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                },
                WindowEvent::Resized(_) => {
                    context.recreate_swapchain = true;
                },
                _ => {},
            },
            Event::MainEventsCleared => { 
                context.create_command_buffer();

                let particles = update(context, &mut count);
            
                context.begin_frame();

                if i > 120 {
                    context.draw(particles.clone(), shader_program2.clone(), descriptor.clone());
                } else {
                    context.draw(particles.clone(), shader_program.clone(), descriptor.clone());
                }

                i += 1;

                context.present();

                let mut sleep_time: f64 = 0.016 - now.elapsed().as_secs_f64();
                if sleep_time < 0.0 {
                    sleep_time = 0.0
                }

                std::thread::sleep(std::time::Duration::from_secs_f64(sleep_time));
                print!("{:.2}\r", now.elapsed().as_secs_f32() * 1000.0);
            },
            _ => {}
        }
    });
}

fn update(context: &mut GraphicsContext, count: &mut f32) -> std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<[ParticleVertex; (AMOUNTX * AMOUNTY) as usize]>> {
    let mut i = 0;
    let mut particle_data = [ParticleVertex::default(); (AMOUNTX * AMOUNTY) as usize];
    
    for ix in 0..AMOUNTX {
        for iy in 0..AMOUNTY {
            let factor = 10.0;
            let scale = 2.0;
            let sin_ix = Rad((ix as f32 + *count) * 0.5).sin();
            let sin_iy = Rad((iy as f32 + *count) * 0.5).sin();

            particle_data[i].position[0] = (ix as f32 * SEPARATION) - (((AMOUNTX as f32 * SEPARATION) / 2.0));
            particle_data[i].position[1] = ( sin_ix * factor) + ( sin_iy * factor);
            particle_data[i].position[2] = (iy as f32 * SEPARATION) - (((AMOUNTY as f32 * SEPARATION) / 2.0));

            particle_data[i].scale = ( sin_ix + 1.5 ) * scale +
                            ( sin_iy + 1.5 ) * scale;

            i += 1;
        }
    }
    *count += 0.03;
    return CpuAccessibleBuffer::from_data(
        context.device.clone(), 
        BufferUsage::vertex_buffer(), 
        false, 
        particle_data
    ).unwrap()
}