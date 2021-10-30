use ledge_engine::prelude::*;
use cgmath::{Deg, Rad, Angle};
use winit::{
    event_loop::{ControlFlow},
    event::{Event, WindowEvent}
};
use ledge_engine::graphics::Vertex;

const SEPARATION: f32 = 10.0; // Point drawing values.
const AMOUNTX: isize = 71;
const AMOUNTY: isize = 71;

fn main() {
    let (mut context, event_loop) = GraphicsContext::new(Conf::new("Wave")); // Creating a new context.

    pub mod vs { vulkano_shaders::shader! { ty: "vertex", path: "examples/shaders/particle-color.vert", } }
    let vs = vs::Shader::load(context.device.clone()).unwrap(); // Load shaders at compile time.

    pub mod fs { vulkano_shaders::shader! { ty: "fragment", path: "examples/shaders/particle-color.frag", } }
    let fs = fs::Shader::load(context.device.clone()).unwrap();

    let shader_program = std::sync::Arc::new(ShaderProgram::new( // Create a new shader program.
        &mut context, 
        buffer::BufferDefinition::new().vertex::<Vertex>(), 
        VertexOrder::PointList,
        vs.main_entry_point(),
        fs.main_entry_point(), 
        BlendMode::Alpha
    ));

    context.add_perspective_camera();
    let camera = context.camera();
    camera.rotate_x(Deg(20.0));
    camera.translate_z(600.0);

    let color = std::sync::Arc::new(context.buffer_from([1.0 as f32, 1.0 as f32, 1.0 as f32]).unwrap());
    context.create_descriptor(color, 1);
    context.build_descriptor_set(shader_program.clone());
    context.pipe_data.vert_count = (AMOUNTX * AMOUNTY) as u32;

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
                context.begin_frame();
                // context.camera().translate_x(1.0);
                context.bind_descriptor_sets(shader_program.clone());
            
                context.draw(particles.clone(), shader_program.clone());
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
    std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<[Vertex; (AMOUNTX * AMOUNTY) as usize]>> 
    // std::sync::Arc<[Vertex; (AMOUNTX * AMOUNTY) as usize]>
{
    let mut i = 0;
    let mut data = [Vertex::default(); (AMOUNTX * AMOUNTY) as usize];
    
    for ix in 0..AMOUNTX {
        for iy in 0..AMOUNTY {
            let sin_factor = Rad(*count/4.0).sin();
            let sin_ix = Rad(sin_factor * (ix as f32 + *count) * 0.5).sin();
            let sin_iy = Rad(sin_factor * (iy as f32 + *count) * 0.5).sin();

            data[i].position[0] = (ix as f32 * SEPARATION) - (((AMOUNTX as f32 * SEPARATION) / 2.0));
            data[i].position[1] = ( sin_ix * 10.0) + ( sin_iy * 10.0);
            data[i].position[2] = (iy as f32 * SEPARATION) - (((AMOUNTY as f32 * SEPARATION) / 2.0));
            data[i].scale = 6.0;
            i += 1;
        }
    }
    *count += 0.03;
    if *count > (4.0*3.14) {
        *count = 0.0;
    }
    context.buffer_from(data).unwrap()
}