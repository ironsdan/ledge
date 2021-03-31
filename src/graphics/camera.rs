use cgmath::{Matrix4, Vector4};

pub struct PerspectiveCamera {
    fov: usize,
    aspect_ratio: f32,
    near: usize,
    far: usize,
    matrix: Matrix4<f32>,
}

impl Default for PerspectiveCamera {
    fn default() -> Self {
        let x = Vector4::new(1.0, 0.0, 0.0, 0.0);
        let y = Vector4::new(0.0, 1.0, 0.0, 0.0);
        let z = Vector4::new(0.0, 0.0, 1.0, 0.0);
        let w = Vector4::new(0.0, 0.0, 0.0, 1.0);
        
        let matrix = Matrix4::from_cols(x, y, z, w);
        Self {
            fov: 75,
            aspect_ratio: 4.0/3.0,
            near: 1,
            far: 1000,
            matrix,
        }
    }
}

impl PerspectiveCamera {
    pub fn new(fov: usize, aspect_ratio: f32, near: usize, far: usize) -> Self {
        let x = Vector4::new(1.0, 0.0, 0.0, 0.0);
        let y = Vector4::new(0.0, 1.0, 0.0, 0.0);
        let z = Vector4::new(0.0, 0.0, 1.0, 0.0);
        let w = Vector4::new(0.0, 0.0, 0.0, 1.0);
        
        let matrix = Matrix4::from_cols(x, y, z, w);
        Self {
            fov,
            aspect_ratio,
            near,
            far,
            matrix,
        }
    }
}