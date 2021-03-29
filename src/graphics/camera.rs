use cgmath::Matrix4;

struct PerspectiveCamera {
    fov: usize,
    aspect_ratio: f32,
    near: usize 
    far: usize,
    matrix: Matrix4,
}

impl PerspectiveCamera {
    pub fn new(fov: usize, aspect_ratio: f32, near: usize, far: usize) -> Self {
        Self {
            fov,
            aspect_ratio,
            near,
            far,
            Matrix4::default(),
        }
    }
}