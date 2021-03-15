pub mod context;
pub mod animation;
pub mod sprite;
pub mod shader;
pub mod image;

use cgmath::{
    Matrix4,
    Vector3,
    Rad,
};

use crate::graphics::context::GraphicsContext;

pub mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/graphics/texture.vert",
        // dump: true,
    }
}

pub mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/graphics/texture.frag"
    }
}

#[derive(Clone, PartialEq, Hash, Eq)]
pub enum BlendMode {
    Default,
    Alpha,
    
}

pub trait Drawable {
    fn draw(&mut self, context: &mut GraphicsContext);
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Vertex {
    pub a_pos: [f32; 2],
    pub a_uv: [f32; 2],
    pub a_vert_color: [f32; 4],
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct InstanceData {
    a_src: [f32; 4],
    a_color: [f32; 4],
    a_transform: [[f32; 4]; 4],
}

#[derive(Debug, Clone, PartialEq)]
pub enum Transform {
    Matrix(Matrix4<f32>)
}

impl Default for Transform {
    fn default() -> Self {
        Transform::Matrix(Matrix4::new(1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0))
    }
}

impl Transform {
    fn as_mat4(&self) -> Matrix4<f32> {
        match self {
            Transform::Matrix(mat) => *mat,
            // _ => 
        }
    }

    fn translate(&mut self, x: f32, y: f32, z: f32) {
        match self {
            Transform::Matrix(mat) => {
                *mat = *mat * Matrix4::from_translation(Vector3::new(x, y, z));
            }
        }
    }

    fn rotate(&mut self, x: f32, y: f32, z: f32) {
        let rotation = Matrix4::from_angle_x(Rad(x)) + 
                       Matrix4::from_angle_y(Rad(y)) + 
                       Matrix4::from_angle_z(Rad(z));
        match self {
            Transform::Matrix(mat) => {
                *mat = *mat * rotation;
            }
        }
    }

    fn nonuniform_scale(&mut self, x: f32, y: f32, z: f32) {
        match self {
            Transform::Matrix(mat) => {
                *mat = *mat * Matrix4::from_nonuniform_scale(x, y, z);
            }
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DrawInfo {
    pub texture_rect: Rect,
    pub color: [f32; 4],
    pub transform: Transform,
}

impl DrawInfo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn into_instance_data(&self) -> InstanceData {
        InstanceData {
            a_src: self.texture_rect.as_vec(),
            a_color: self.color,
            a_transform: self.transform.as_mat4().into(),
        }
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.transform.translate(x, y, z);
    }

    pub fn rotate(&mut self, x: f32, y: f32, z: f32) {
        self.transform.rotate(x, y, z);
    }

    pub fn nonuniform_scale(&mut self, x: f32, y: f32, z: f32) {
        self.transform.nonuniform_scale(x, y, z);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn as_vec(&self) -> [f32; 4] {
        [self.x, self.y, self.w, self.h]
    }
}