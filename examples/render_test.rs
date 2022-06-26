use ledge::event;
use ledge::graphics::camera::OrthographicCamera;
use ledge::interface::*;
use ledge::graphics::{self, shader::*};
use std::sync::Arc;
use vulkano::pipeline::graphics::vertex_input::BuffersDefinition;
use bytemuck::{Pod, Zeroable};
use ledge::graphics::render_pass::frame;
use ledge::graphics::Color;
use ledge::graphics::image::Image;

use anyhow::Result;

use vulkano::sync::GpuFuture;

// struct LineShader {}

// impl ShaderHandle for LineShader {
//     fn draw(
//         &self,
//         command_buffer: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
//         pipe_data: Box<dyn PipelineData>,
//     ) {

//     }
//     // fn set_blend_mode(&mut self, mode: BlendMode);
//     fn blend_mode(&self) -> graphics::BlendMode {
//         graphics::BlendMode::Alpha
//     }

//     fn layout(&self) -> &[Arc<DescriptorSetLayout>] {
//         &[]
//     }

//     fn pipeline(&self) -> Arc<GraphicsPipeline> {
        
//     }
// }

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
pub struct TestVertex {
    pub pos: [f32; 3],
    pub uv: [f32; 2],
    pub vert_color: [f32; 4],
}

vulkano::impl_vertex!(TestVertex, pos, uv, vert_color);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
pub struct InstanceData {
    src: [f32; 4],
    color: [f32; 4],
    transform: [[f32; 4]; 4],
}

vulkano::impl_vertex!(InstanceData, src, color, transform);

struct MainState {
    test_shader: ShaderId,
    camera: Arc<OrthographicCamera>,
    image: Arc<Image>,
}

impl event::EventHandler for MainState {
    fn start(interface: &mut Interface) -> Self {
        let Interface {
            renderer,
            ..
        } = interface;

        mod vs {
            vulkano_shaders::shader! {
                ty: "vertex",
                path: "examples/shaders/basic.vert",
            }
        }

        mod fs {
            vulkano_shaders::shader! {
                ty: "fragment",
                path: "examples/shaders/basic.frag",
            }
        }

        let vs = vs::load(renderer.device.clone()).unwrap();
        let fs = fs::load(renderer.device.clone()).unwrap();

        let v_type = BuffersDefinition::new()
            .vertex::<TestVertex>()
            .instance::<InstanceData>();

        let test_shader = Arc::new(ledge::graphics::shader::Shader {
            vertex: vs.entry_point("main").unwrap(),
            fragment: fs.entry_point("main").unwrap(),
            topology: graphics::shader::VertexTopology::TriangleFan,
            // vertex_definition: v_type,
        });

        let mut render_pass = crate::graphics::render_pass::RenderPass::new( 
            renderer.queue.clone(),
            vulkano::ordered_passes_renderpass!(renderer.device.clone(),
                attachments: {
                    final_color: {
                        load: Clear,
                        store: Store,
                        format: renderer.output_format(),
                        samples: 1,
                    }
                },
                passes: [
                    {
                        color: [final_color],
                        depth_stencil: {},
                        input: []
                    }
                ]
            ).unwrap(),
        ).unwrap();

        let test_shader = render_pass.register_shader(test_shader, v_type).unwrap();

        renderer.render_passes.push(render_pass);

        let image = Arc::new(
            Image::new(
                renderer.queue.clone(), 
                renderer.samplers[0].clone(), 
                "examples/images/pokeball.png",
            ),
        );
        
        MainState{
            test_shader,
            camera: Arc::new(OrthographicCamera::new(1.0, 1000.0)),
            image: image,
        }
    }

    fn update(&mut self, _interface: &mut Interface) -> Result<()> {
        Ok(())
    }
    
    fn draw(&mut self, interface: &mut Interface, before_future: Box<dyn GpuFuture>) -> Result<Box<dyn GpuFuture>> {
        let Interface {
            renderer,
            ..
        } = interface;

        let clear = Color::grey();

        let final_image = renderer.final_image();
        let mut frame = renderer.render_passes[0].frame(
            clear.into(), 
            before_future, 
            final_image, 
            self.camera.clone(),
        )?;

        let mut after_future = None;
        while let Some(pass) = frame.next_pass()? {
            after_future = match pass {
                frame::PassState::DrawPass(mut pass) => {
                    let params = graphics::DrawInfo::default();
                    pass.draw_with(
                        self.image.clone(), 
                        self.test_shader, 
                        params,
                    )?;
                    None
                },
                frame::PassState::Finished(af) => {
                    Some(af)
                }
            }
        }

        Ok(after_future.unwrap())
    }

    fn resize(&mut self, _width: u32, _height: u32) -> Result<()> {
        Ok(())
    }
}

fn main() {
    let (interface, event_loop) = 
        InterfaceBuilder::new("test", "Dan")
            .build()
            .unwrap();
    
    event::run::<MainState>(interface, event_loop);
}