use cgmath::{Matrix4, Vector4, Rad, Deg};
use cgmath::prelude::*;

pub struct PerspectiveCamera {
    fov: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32,
    model: Matrix4<f32>,
    view: Matrix4<f32>,
    proj: Matrix4<f32>,
}

impl Default for PerspectiveCamera {
    fn default() -> Self {
        let fov: f32 = 75.0;

        let x_rot: Rad<f32> = Deg(20.0).into();
        println!("{:?}", x_rot);

        let model_x = Vector4::new(1.0, 0.0, 0.0, 0.0);
        let model_y = Vector4::new(0.0, x_rot.cos(), x_rot.sin(), 0.0);
        let model_z = Vector4::new(0.0, -x_rot.sin(), x_rot.cos(), 0.0);
        let model_w = Vector4::new(0.0, 0.0, 0.0, 1.0);
        
        let model = Matrix4::from_cols(model_x, model_y, model_z, model_w);

        let view_x = Vector4::new(1.0, 0.0, 0.0, 0.0);
        let view_y = Vector4::new(0.0, 1.0, 0.0, 0.0);
        let view_z = Vector4::new(0.0, 0.0, 1.0, 0.0);
        let view_w = Vector4::new(0.0, 20.0, 600.0, 1.0);
        
        let view = Matrix4::from_cols(view_x, view_y, view_z, view_w);

        let angle_rad: Rad<f32> = Deg(fov).into();
        // println!("angle deg: {:?}, angle rad: {:?}", Deg(fov), angle_rad);
        
        let focal_length = 1.0 / Rad::tan(angle_rad/2.0);
        let aspect_ratio = 4.0/3.0;
        let n = 5.0;
        let f = 1000.0;

        let c0r0 = focal_length / aspect_ratio;
        let c1r1 = -focal_length;
        let c2r2 = (-f) / (f - n);
        let c3r2 = (f * n) / (f - n);

        let proj_x = Vector4::new(c0r0, 0.0, 0.0, 0.0);
        let proj_y = Vector4::new(0.0, c1r1, 0.0, 0.0);
        let proj_z = Vector4::new(0.0, 0.0, c2r2, -1.0);
        let proj_w = Vector4::new(0.0, 0.0, c3r2, 0.0);

        let proj = Matrix4::from_cols(proj_x, proj_y, proj_z, proj_w);

        let test = proj*view*model * Vector4::new(0.0, 0.0, 10.0, 1.0);

        // println!("{:?}", proj*view*model);
        // println!("{:?}\n{:?}", test, test/test[3]);

        Self {
            fov,
            aspect_ratio,
            near: n,
            far: f,
            model: model,
            view: view,
            proj: proj,
        }
    }
}

impl PerspectiveCamera {
    pub fn new(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
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