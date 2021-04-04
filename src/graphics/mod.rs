pub mod context;
pub mod shader;
pub mod image;
pub mod camera;
pub mod buffer;
pub mod material;

use crate::graphics::context::GraphicsContext;
use vulkano::buffer::BufferAccess;
use std::collections::HashMap;

use cgmath::{
    Matrix,
    Matrix4,
    Vector4,
    Vector3,
    Rad,
    prelude::Angle,
};

use vulkano::{
    descriptor::descriptor_set::PersistentDescriptorSet,
    descriptor::descriptor_set::PersistentDescriptorSetBuilder,
    pipeline::GraphicsPipelineAbstract,
};

use std::sync::Arc;

// pub mod vs {
//     vulkano_shaders::shader! {
//         ty: "vertex",
//         path: "src/graphics/texture.vert",
//         // dump: true,
//     }
// }

// pub mod fs {
//     vulkano_shaders::shader! {
//         ty: "fragment",
//         path: "src/graphics/texture.frag"
//     }
// }

#[derive(Clone, PartialEq, Hash, Eq)]
pub enum BlendMode {
    Default,
    Alpha,
    
}

pub trait Drawable {
    fn draw(&self, context: &mut GraphicsContext);
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
    Components {
        pos: Vector3<f32>,
        rotation: Rad<f32>,
        scale: Vector3<f32>,
        offset: Vector3<f32>,
    },
    Matrix(Matrix4<f32>)
}

impl Default for Transform {
    fn default() -> Self {
        Transform::identity()
    }
}

impl Transform {
    fn identity() -> Self {
        Self::Components {
            pos: Vector3::from((0.0, 0.0, 0.0)),
            rotation: Rad(0.0),
            scale: Vector3::from((1.0, 1.0, 0.0)),
            offset: Vector3::from((0.0, 0.0, 0.0)),
        }
    }

    pub fn as_mat4(&self) -> Matrix4<f32> {
        match self {
            Transform::Matrix(mat) => *mat,
            Transform::Components {
                pos,
                rotation,
                scale,
                offset,
            } => {
                let (sinr, cosr) = rotation.sin_cos();
                let cr00 = cosr * scale.x;
                let cr01 = -sinr * scale.y;
                let cr10 = sinr * scale.x;
                let cr11 = cosr * scale.y;
                let cr03 = offset.x * (1.0 - cr00) - offset.y * cr01 + pos.x;
                let cr13 = offset.y * (1.0 - cr11) - offset.x * cr10 + pos.y;
                
                Matrix4::from_cols(
                    Vector4::new(cr00, cr01, 0.0, cr03,),
                    Vector4::new(cr10, cr11, 0.0, cr13,),
                    Vector4::new(0.0, 0.0, 1.0, 0.0,),
                    Vector4::new(0.0, 0.0, 0.0, 1.0,),
                ).transpose()
            }
        }
    }

    fn translate(&mut self, x: f32, y: f32, z: f32) {
        match self {
            Transform::Matrix(mat) => {
                *mat = *mat + Matrix4::from_translation(Vector3::new(x, y, z));
            }
            Transform::Components {
                pos,
                ..
            } => {
                *pos += Vector3::from((x, y, z));
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
            Transform::Components {
                // rotation,
                ..
            } => {
                // *rotation += Rad(3.14);
            }
        }
    }

    fn rotate_value(&mut self, r: Rad<f32>) {
        match self {
            Transform::Matrix(mat) => {}
            Transform::Components {
                rotation,
                ..
            } => {
                *rotation = r;
            }
        }
    }

    fn nonuniform_scale(&mut self, x: f32, y: f32, z: f32) {
        match self {
            Transform::Matrix(mat) => {
                *mat = *mat * Matrix4::from_nonuniform_scale(x, y, z);
            }
            Transform::Components {
                scale,
                ..
            } => {
                *scale = Vector3::from((x, y, z));
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DrawInfo {
    pub texture_rect: Rect,
    pub color: [f32; 4],
    pub transform: Transform,
}

impl Default for DrawInfo {
    fn default() -> Self {
        Self {
            texture_rect: Rect::default(),
            color: [0.0, 0.0, 0.0, 1.0],
            transform: Transform::identity(),
        }
    }
}

impl DrawInfo {
    pub fn new() -> Self {
        Self {
            texture_rect: Rect::default(),
            color: [0.0, 0.0, 0.0, 1.0],
            transform: Transform::identity(),
        }
    }
    
    pub fn with_rect(rect: Rect) -> Self {
        Self {
            texture_rect: rect,
            color: [0.0, 0.0, 0.0, 1.0],
            transform: Transform::identity(),
        }
    }

    pub fn with_transform(transform: Transform) -> Self {
        Self {
            texture_rect: Rect::default(),
            color: [0.0, 0.0, 0.0, 1.0],
            transform: transform,
        }
    }

    pub fn with_color(color: [f32; 4]) -> Self {
        Self {
            texture_rect: Rect::default(),
            color: color,
            transform: Transform::identity(),
        }
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

    pub fn rotate_value(&mut self, r: f32) {
        self.transform.rotate_value(Rad(r));
    }

    pub fn nonuniform_scale(&mut self, x: f32, y: f32, z: f32) {
        self.transform.nonuniform_scale(x, y, z);
    }

    pub fn scale(&mut self, s: f32) {
        self.transform.nonuniform_scale(s, s, s);
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

pub struct Descriptor<T> {
    inner: T,
    attributes: HashMap<String, Box<dyn BufferAccess>>,
}

pub struct DescriptorBuilder {
    inner: PersistentDescriptorSetBuilder<()>,
    // attributes: HashMap<String, Box<dyn BufferAccess>>,
}

impl DescriptorBuilder {
    pub fn new(pipeline: &Arc<GraphicsPipelineAbstract>) -> Self {
        Self {
            inner: PersistentDescriptorSet::start(pipeline.descriptor_set_layout(0).unwrap().clone())
        }
    }
}