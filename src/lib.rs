use vulkano::command_buffer::DynamicState;
use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, RenderPassAbstract};
use vulkano::image::SwapchainImage;
use vulkano::pipeline::viewport::Viewport;
use winit::window::Window;
use std::sync::Arc;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Rect {
    pub height: f32,
    pub width: f32,
    pub x: f32,
    pub y: f32,
    pub vertices: [Vertex; 4],
}

impl Rect {
    pub fn new(width: f32, height: f32, pos: [f32; 2], texture_coord: [[f32; 2]; 4]) -> Self {
        // println!("pos X: {}, pos Y: {}, height: {}, width: {}", pos[0], pos[1], height, width);
        Self {
            height: height,
            width: width,
            x: pos[0],
            y: pos[1],
            vertices: [
                Vertex {
                    position: [pos[0], pos[1]],
                    tex_coords: texture_coord[0],
                },
                Vertex {
                    position: [pos[0], pos[1] + height],
                    tex_coords: texture_coord[1],
                },
                Vertex {
                    position: [pos[0] + width, pos[1]],
                    tex_coords: texture_coord[2],
                },
                Vertex {
                    position: [pos[0] + width, pos[1] + height],
                    tex_coords: texture_coord[3],
                },
            ],
        }
    }

    pub fn width(&self) -> f32 {
        return self.width;
    }

    pub fn height(&self) -> f32 {
        return self.height;
    }

    pub fn update(&mut self, position: [f32; 2]) {
        let default = [
            [   0.0,         0.0    ],
            [   0.0,     self.height],
            [self.width,     0.0    ],
            [self.width, self.height],
        ];
        
        for i in 0..4 {
            for j in 0..2 {
                self.vertices[i].position[j] = default[i][j] + position[j];
            }
        }

        self.x = position[0];
        self.y = position[1];
    }

    pub fn update_size(&mut self, size: [f32; 2]) {
        self.width = size[0];
        self.height = size[1];
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}

// This method is called once during initialization, then again whenever the window is resized
pub fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
    dynamic_state: &mut DynamicState,
) -> Vec<Arc<dyn FramebufferAbstract + Send + Sync>> {
    let dimensions = images[0].dimensions();

    let viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [dimensions[0] as f32, dimensions[1] as f32],
        depth_range: 0.0..1.0,
    };
    dynamic_state.viewports = Some(vec![viewport]);

    images
        .iter()
        .map(|image| {
            Arc::new(
                Framebuffer::start(render_pass.clone())
                    .add(image.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            ) as Arc<dyn FramebufferAbstract + Send + Sync>
        })
        .collect::<Vec<_>>()
}

pub fn convert_to_screen_space(size: [u32;2], dimensions: [u32; 2]) -> [f32; 2] {
    let window_width = dimensions[0];
    let window_height = dimensions[1];

    let aspect_ratio;

    if window_height > window_width {
        aspect_ratio = window_height as f32 / window_width as f32;
    } else {
        aspect_ratio = window_width as f32 / window_height as f32;
    }

    let pixel_size_y = 1.0/window_height as f32;
    let pixel_size_x = 1.0/window_width as f32;

    let screen_width = 2.0*pixel_size_x*size[0] as f32;
    let screen_height = 2.0*pixel_size_y*size[1] as f32;

    let screen_size = [screen_width, screen_height];
    return screen_size;
}



pub enum MovementInput {
    UpPress,
    UpRelease,
    // Down,
    Left,
    Right
}