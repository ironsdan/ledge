use crate::graphics::{
    image::Image,
    Vertex,
};

use vulkano::buffer::CpuAccessibleBuffer;

pub struct Mesh {
    buffer: CpuAccessibleBuffer<Vertex>
    image: Image,
}

pub struct MeshBuilder {
    buffer: CpuAccessibleBuffer<Vertex>
}