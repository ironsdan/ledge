use cgmath::{Matrix4, Vector4};

pub struct PerspectiveCamera {
    fov: usize,
    aspect_ratio: f32,
    near: usize,
    far: usize,
    model: Matrix4<f32>,
    view: Matrix4<f32>,
    proj: Matrix4<f32>,
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
            model: matrix.clone(),
            view: matrix.clone(),
            proj: matrix.clone(),
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
            model: matrix.clone(),
            view: matrix.clone(),
            proj: matrix.clone(),
        }
    }

    pub fn model_array(&self) -> [[f32; 4]; 4] {
        return self.model.into();
    }

    pub fn view_array(&self) -> [[f32; 4]; 4] {
        return self.view.into();
    }

    pub fn proj_array(&self) -> [[f32; 4]; 4] {
        return self.proj.into();
    }

    pub fn mv_array(&self) -> [[f32; 4]; 4] {
        let mv = self.model * self.view;
        return mv.into();
    }

    pub fn mvp_array(&self) -> [[f32; 4]; 4] {
        let mvp = self.model * self.view * self.proj;

        return mvp.into();
    }
}