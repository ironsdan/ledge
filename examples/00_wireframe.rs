use ledge_engine::graphics::buffer::*;
use winit::{
    event_loop::{ControlFlow},
    event::{Event, WindowEvent}
};
use vulkano::{
    descriptor::descriptor_set::PersistentDescriptorSet,
};
use std::sync::Arc;
use ledge_engine::graphics::camera::PerspectiveCamera;
use ledge_engine::graphics::shader::PipelineObject;
use ledge_engine::graphics::context::GraphicsContext;
use ledge_engine::conf::Conf;
use ledge_engine::graphics::BlendMode;
use ledge_engine::graphics::shader::ShaderProgram;
use vulkano::pipeline::vertex::SingleBufferDefinition;
use ledge_engine::graphics::shader::VertexOrder;

#[derive(Default, Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    barycenter: [f32; 3],
}

vulkano::impl_vertex!(Vertex, position, barycenter);

pub mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "examples/shaders/wireframe.vert",
    }
}

pub mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "examples/shaders/wireframe.frag",
    }
}

#[derive(Clone, Copy)]
#[allow(unused)]
struct CameraMvp {
    model: [[f32; 4]; 4],
    view: [[f32; 4]; 4],
    proj: [[f32; 4]; 4],
}

fn main() {
    let (mut context, event_loop) = GraphicsContext::new(Conf::new("Wave"));

    let vs = vs::Shader::load(context.device.clone()).unwrap();
    let fs = fs::Shader::load(context.device.clone()).unwrap();

    let vertex_shader = ledge_engine::graphics::shader::Shader::new(vs.main_entry_point(), ());
    let fragment_shader = ledge_engine::graphics::shader::Shader::new(fs.main_entry_point(), ());
    
    let po = Arc::new(PipelineObject::new(
        &mut context, 
        SingleBufferDefinition::<Vertex>::new(), 
        VertexOrder::TriangleList,
        vertex_shader, 
        fragment_shader, 
        BlendMode::Alpha
    ));

    let shader_program = Arc::new(ShaderProgram::from_pipeline(BlendMode::Alpha, po.clone()));

    let camera = PerspectiveCamera::new(75.0, 4.3/3.0, 5.0, 2000.0);

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

    let barycenter = [[1.0, 0.0, 0.0],
                     [0.0, 1.0, 0.0],
                     [0.0, 0.0, 1.0]];

    let triangle = BufferAttribute::from_data(
        [
            Vertex {
                position: [0.0, 0.0, 200.0],
                barycenter: barycenter[2],
            },
            Vertex {
                position: [50.0, 0.0, 200.0],
                barycenter: barycenter[0],
            },
            Vertex {
                position: [50.0, -100.0, 200.0],
                barycenter: barycenter[1],
            },
            Vertex {
                position: [0.0, 0.0, 200.0],
                barycenter: barycenter[2],
            },
            Vertex {
                position: [-50.0, 0.0, 200.0],
                barycenter: barycenter[0],
            },
            Vertex {
                position: [-50.0, 100.0, 200.0],
                barycenter: barycenter[1],
            },
        ],
        context.device.clone()
    );

    let descriptor = Arc::new(
        PersistentDescriptorSet::start(po.pipeline.descriptor_set_layout(0).unwrap().clone())
            .add_buffer(color.inner.clone()).unwrap() 
            .add_buffer(mvp.inner.clone()).unwrap()
            .build()
            .unwrap(),
    );

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

            
                context.begin_frame();

                context.draw(triangle.inner.clone(), shader_program.clone(), descriptor.clone());

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