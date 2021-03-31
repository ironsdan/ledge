use ledge_engine::interface::*;
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
use std::sync::Arc;

#[derive(Default, Copy, Clone)]
struct ParticleVertex {
    position: [f32; 3],
    scale: u32,
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
        path: "src/particle.frag"
    }
}

const SEPARATION: f32 = 10.0;
const AMOUNTX: usize = 100;
const AMOUNTY: usize = 100;

fn main() {
    let (mut interface, event_loop) = InterfaceBuilder::new("Example01", "Dan").build().unwrap();

    let vs = vs::Shader::load(interface.graphics_context.device.clone()).unwrap();
    let fs = fs::Shader::load(interface.graphics_context.device.clone()).unwrap();

    let pipeline = Arc::new(
        GraphicsPipeline::start()
            .vertex_input_single_buffer::<ParticleVertex>()
            .vertex_shader(vs.main_entry_point(), ())
            .point_list()
            .viewports_dynamic_scissors_irrelevant(1)
            .fragment_shader(fs.main_entry_point(), ())
            .blend_alpha_blending()
            .render_pass(Subpass::from(interface.graphics_context.render_pass.clone(), 0).unwrap())
            .build(interface.graphics_context.device.clone())
            .unwrap()
    ) as Arc<dyn GraphicsPipelineAbstract + Send + Sync>;

    let mut particles = [ParticleVertex::default(); AMOUNTX * AMOUNTY];
    let mut i = 0;
    for ix in 0..AMOUNTX  {
        for iy in 0..AMOUNTY {
            let mut position: [f32; 3] = [0.0,0.0,0.0];
            position[0] = (ix as f32 * SEPARATION)  - (((AMOUNTX as f32 * SEPARATION) / 2.0)); // x
            position[1] = 0.0; // y
            position[2] = (iy as f32 * SEPARATION) - (((AMOUNTY as f32 * SEPARATION) / 2.0)); // z
            
            let particle = ParticleVertex {
                position: position,
                scale: 1,
            };
            particles[i] = particle;
            i+=1;
        }
    }

    let particle = CpuAccessibleBuffer::from_data(
        interface.graphics_context.device.clone(), 
        BufferUsage::vertex_buffer(), 
        false, 
        particles
    ).unwrap();

    let color = CpuAccessibleBuffer::from_data(
        interface.graphics_context.device.clone(), 
        BufferUsage::uniform_buffer_transfer_destination(), 
        false,
        [1.0 as f32, 1.0 as f32, 1.0 as f32],
    ).unwrap();
    
    let descriptor = Arc::new(
        PersistentDescriptorSet::start(pipeline.descriptor_set_layout(0).unwrap().clone())
            .add_buffer(interface.graphics_context.mvp_buffer.clone()).unwrap()
            .add_buffer(color.clone()).unwrap()
            .build()
            .unwrap(),
    );

    event_loop.run(move |event, _, control_flow| {
        let interface = &mut interface;

        interface.process_event(&event);
        
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

                interface.graphics_context.command_buffer.as_mut().unwrap().update_buffer(color.clone(), [1.0 as f32, 0.0 as f32, 1.0 as f32]).unwrap();
                
                interface.graphics_context.begin_frame();

                interface.graphics_context.command_buffer.as_mut().unwrap().draw(
                    pipeline.clone(),
                    &interface.graphics_context.dynamic_state,
                    vec![Arc::new(particle.clone())],
                    descriptor.clone(),
                    (),
                ).unwrap();

                interface.graphics_context.present();
            },
            Event::RedrawRequested(_) => {},
            Event::RedrawEventsCleared => {},
        }
        
    });
}