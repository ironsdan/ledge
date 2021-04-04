use crate::graphics::shader::Shader;

pub struct ShaderMaterial<Vs, Vss, Fs, Fss> {
    vertex_shader: Shader<Vs, Vss>,
    fragment_shader: Shader<Fs, Fss>,

}