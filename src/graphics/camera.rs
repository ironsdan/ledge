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
        // let fov = 75;
        let left = -1.25;
        let right = 1.25;
        let bot = -1;
        let top = 1;

        let aspect_ratio = 4.0/3.0;
        let near = 1;
        let far = 1000;

        let x = Vector4::new(1.0, 0.0, 0.0, 0.0);
        let y = Vector4::new(0.0, 1.0, 0.0, 0.0);
        let z = Vector4::new(0.0, 0.0, 1.0, 0.0);
        let w = Vector4::new(0.0, 0.0, 0.0, 1.0);
        
        let matrix = Matrix4::from_cols(x, y, z, w);

        let c0r0 = (2*near)/(right - left);
        let c1r1 = (2*near) / (top - bot);
        let c2r0 = (right + left) / (right - left);
        let c2r1 = (top + bot) / (top - bot);
        let c2r2 = -1*(far + near) / (far - near);
        let c3r2 = -1*(2*far*near) / (far - near);

        let proj_x = Vector4::new(c0r0, 0.0, 0.0, 0.0);
        let proj_y = Vector4::new(0.0, c1r1, 0.0, 0.0);
        let proj_z = Vector4::new(c2r0, c2r1, c2r2, -1.0);
        let proj_w = Vector4::new(0.0, 0.0, c3r2, 1.0);

        let proj = Matrix4::from_cols(proj_x, proj_y, proj_z, proj_w);

        Self {
            fov,
            aspect_ratio,
            near,
            far,
            model: matrix.clone(),
            view: matrix,
            proj: proj,
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