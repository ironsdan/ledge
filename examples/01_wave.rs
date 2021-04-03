use ledge_engine::interface::*;
use ledge_engine::graphics::buffer::*;
use winit::{
    event_loop::{ControlFlow},
    event::{Event, WindowEvent}
};
use vulkano::{
    framebuffer::{Subpass},
    pipeline::{GraphicsPipeline, GraphicsPipelineAbstract},
    descriptor::descriptor_set::PersistentDescriptorSet,
    buffer::{BufferUsage, CpuAccessibleBuffer},
};
use cgmath::{Deg, Rad, Angle};
use std::sync::Arc;
use ledge_engine::graphics::camera::PerspectiveCamera;
use ledge_engine::graphics::shader::PipelineObject;
use ledge_engine::graphics::shader::Shader;
use vulkano::SafeDeref;

#[derive(Default, Copy, Clone)]
struct ParticleVertex {
    position: [f32; 3],
    scale: f32,
}

vulkano::impl_vertex!(ParticleVertex, position, scale);

pub mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/particle.vert",
    }
}

pub mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/particle.frag",
    }
}

const SEPARATION: f32 = 12.0;
const AMOUNTX: isize = 100;
const AMOUNTY: isize = 100;

fn main() {
    let (mut interface, event_loop) = InterfaceBuilder::new("Wave01", "Dan").build().unwrap();

    let vs = vs::Shader::load(interface.graphics_context.device.clone()).unwrap();
    let fs = fs::Shader::load(interface.graphics_context.device.clone()).unwrap();

    let vertex_shader = Shader {
        entry_point: vs.main_entry_point(),
        specialization_constants: (),
    };

    let fragment_shader = Shader {
        entry_point: fs.main_entry_point(),
        specialization_constants: (),
    };

    // let pipeline = PipelineObject::new(&interface.graphics_context, vertex_shader, fragment_shader);

    let pipeline = PipelineObject::new(&mut interface.graphics_context, ParticleVertex::default(), vertex_shader, fragment_shader);

    let mut camera = PerspectiveCamera::new(75.0, 4.3/3.0, 5.0, 1000.0);
    camera.rotate_x(Deg(20.0));
    camera.translate_z(100.0);

    let color = BufferAttribute::from_data([1.0 as f32, 1.0 as f32, 1.0 as f32], interface.graphics_context.device.clone());
    
    let mvp_data = vs::ty::mvp {
        model: camera.model_array(),
        view: camera.view_array(),
        projection: camera.proj_array(),
    };
    
    let mvp = BufferAttribute::from_data(mvp_data, interface.graphics_context.device.clone());

    // let descriptor = DescriptorBuilder::new();

    let descriptor = Arc::new(
        PersistentDescriptorSet::start(pipeline.descriptor_set_layout(0).unwrap().clone())
            .add_buffer(color.inner.clone()).unwrap()
            .add_buffer(mvp.inner.clone()).unwrap()
            .build()
            .unwrap(),
    );

    let mut count = 0.0;

    event_loop.run(move |event, _, control_flow| {
        let interface = &mut interface;
        let now = std::time::Instant::now();
        
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                },
                WindowEvent::Resized(_) => {
                    interface.graphics_context.recreate_swapchain = true;
                },
                _ => {},
            },
            Event::DeviceEvent { event, .. } => match event {
                _ => (),
            },
            Event::Resumed => {},
            Event::Suspended => {},
            Event::NewEvents(_) => {},
            Event::UserEvent(_) => {},
            Event::LoopDestroyed => {},
            Event::MainEventsCleared => { 
                const DESIRED_FPS: u32 = 60;
                interface.timer_state.tick();

                interface.graphics_context.create_command_buffer();

                while interface.timer_state.check_update_time(DESIRED_FPS) {}
                let particles = update(interface, &mut count);
            
                interface.graphics_context.begin_frame();

                interface.graphics_context.command_buffer.as_mut().unwrap().draw(
                    pipeline.clone(),
                    &interface.graphics_context.dynamic_state,
                    vec![Arc::new(particles.clone())],
                    descriptor.clone(),
                    (),
                    vec![],
                ).unwrap();

                interface.graphics_context.present();

                let mut sleep_time = 0.016 - now.elapsed().as_secs_f32();
                if sleep_time < 0.0 {
                    sleep_time = 0.0
                }
                std::thread::sleep(std::time::Duration::from_secs_f32(sleep_time));
            },
            Event::RedrawRequested(_) => {},
            Event::RedrawEventsCleared => {},
        }
    });
}

fn update(interface: &Interface, count: &mut f32) -> std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<[ParticleVertex; (AMOUNTX * AMOUNTY) as usize]>> {
    let mut i = 0;
    let mut particle_data = [ParticleVertex::default(); (AMOUNTX * AMOUNTY) as usize];
    
    for ix in 0..AMOUNTX {
        for iy in 0..AMOUNTY {
            let factor = 5.0;
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
        interface.graphics_context.device.clone(), 
        BufferUsage::vertex_buffer(), 
        false, 
        particle_data
    ).unwrap()
}