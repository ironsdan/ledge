use cgmath::prelude::*;
use cgmath::{Deg, Matrix4, Rad, Vector3, Vector4};

pub trait Camera {
    fn model_array(&self) -> [[f32; 4]; 4];

    fn view_array(&self) -> [[f32; 4]; 4];

    fn proj_array(&self) -> [[f32; 4]; 4];

    fn mv_array(&self) -> [[f32; 4]; 4];

    fn mvp_array(&self) -> [[f32; 4]; 4];

    fn rotate_x(&mut self, degs: Deg<f32>);

    fn rotate_y(&mut self, degs: Deg<f32>);

    fn rotate_z(&mut self, degs: Deg<f32>);

    fn translate_x(&mut self, amount: f32);

    fn translate_y(&mut self, amount: f32);

    fn translate_z(&mut self, amount: f32);

    fn as_mvp(&self) -> CameraMvp;
}

#[allow(unused)]
/// A model of an ideal pinhole camera that follows perspective projection.
///  
/// Useful for 3D images where perspective is necessary. The struct contains methods for doing any
/// common transformation on the camera by transforming the model, view, or projection component.
///
/// Note: Follows Vulkan tradition of x: (-1, 1), y: (-1, 1), z: (0, 1) starting at the top left-front (-1,-1, 0),
/// continuing with the consitency of Vulkan the camera looks down the POSITIVE z-direction rather than the negative
/// that is the standard in OpenGL.
///
/// Note: Default values are fov: 75, aspect_ratio: 4.0/3.0, near: 5, far: 1000.
///
/// # Examples
/// ```
/// use ledge_engine::graphics::camera;
/// use cgmath::Deg;
///
/// pub fn main() {
///     let camera = camera::PerspectiveCamera::new(75, 800.0/600.0, 5, 1000);
///     camera.rotate_x(Deg(20.0));
///     camera.translate_z(100.0);
/// }
/// ```
#[derive(Debug)]
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
        let aspect_ratio = 4.0 / 3.0;
        let n = 5.0;
        let f = 1000.0;

        PerspectiveCamera::new(fov, aspect_ratio, n, f)
    }
}

// impl Display for PerspectiveCamera {
//     fn display() -> String {

//     }
// }

impl PerspectiveCamera {
    pub fn new(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        let angle_rad: Rad<f32> = Deg(fov).into();
        let focal_length = 1.0 / Rad::tan(angle_rad / 2.0);

        let model_x = Vector4::new(1.0, 0.0, 0.0, 0.0);
        let model_y = Vector4::new(0.0, 1.0, 0.0, 0.0);
        let model_z = Vector4::new(0.0, 0.0, 1.0, 0.0);
        let model_w = Vector4::new(0.0, 0.0, 0.0, 1.0);

        let model = Matrix4::from_cols(model_x, model_y, model_z, model_w);

        let view_x = Vector4::new(1.0, 0.0, 0.0, 0.0);
        let view_y = Vector4::new(0.0, 1.0, 0.0, 0.0);
        let view_z = Vector4::new(0.0, 0.0, 1.0, 0.0);
        let view_w = Vector4::new(0.0, 0.0, 0.0, 1.0);

        let view = Matrix4::from_cols(view_x, view_y, view_z, view_w);

        let c0r0 = focal_length / aspect_ratio;
        let c1r1 = focal_length;
        let c2r2 = (far) / (far - near);
        let c3r2 = -(far * near) / (far - near);

        let proj_x = Vector4::new(c0r0, 0.0, 0.0, 0.0);
        let proj_y = Vector4::new(0.0, c1r1, 0.0, 0.0);
        let proj_z = Vector4::new(0.0, 0.0, c2r2, 1.0);
        let proj_w = Vector4::new(0.0, 0.0, c3r2, 0.0);

        let proj = Matrix4::from_cols(proj_x, proj_y, proj_z, proj_w);

        Self {
            fov,
            aspect_ratio,
            near,
            far,
            model,
            view,
            proj,
        }
    }
}

impl Camera for PerspectiveCamera {
    fn model_array(&self) -> [[f32; 4]; 4] {
        self.model.into()
    }

    fn view_array(&self) -> [[f32; 4]; 4] {
        self.view.into()
    }

    fn proj_array(&self) -> [[f32; 4]; 4] {
        self.proj.into()
    }

    fn mv_array(&self) -> [[f32; 4]; 4] {
        let mv = self.model * self.view;
        mv.into()
    }

    fn mvp_array(&self) -> [[f32; 4]; 4] {
        let mvp = self.model * self.view * self.proj;
        mvp.into()
    }

    fn rotate_x(&mut self, degs: Deg<f32>) {
        let rotation = Matrix4::from_angle_x(degs);
        self.model = rotation * self.model;
    }

    fn rotate_y(&mut self, degs: Deg<f32>) {
        let rotation = Matrix4::from_angle_y(degs);
        self.model = rotation * self.model;
    }

    fn rotate_z(&mut self, degs: Deg<f32>) {
        let rotation = Matrix4::from_angle_z(degs);
        self.model = rotation * self.model;
    }

    fn translate_x(&mut self, amount: f32) {
        let translation = Matrix4::from_translation(Vector3::new(amount, 0.0, 0.0));
        self.view = translation * self.view;
    }

    fn translate_y(&mut self, amount: f32) {
        let translation = Matrix4::from_translation(Vector3::new(0.0, amount, 0.0));
        self.view = translation * self.view;
    }

    fn translate_z(&mut self, amount: f32) {
        let translation = Matrix4::from_translation(Vector3::new(0.0, 0.0, amount));
        self.view = translation * self.view;
    }

    // fn zoom(&mut self, amount: f32) {

    // }

    fn as_mvp(&self) -> CameraMvp {
        CameraMvp {
            model: self.model_array(),
            view: self.view_array(),
            proj: self.proj_array(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[allow(unused)]
pub struct CameraMvp {
    // Camera struct for conversion to uniform.
    model: [[f32; 4]; 4],
    view: [[f32; 4]; 4],
    proj: [[f32; 4]; 4],
}
