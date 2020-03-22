use gl::types::GLenum;

#[derive(serde::Deserialize, Debug)]
pub enum ShaderType {
    FRAGMENT,
    VERTEX,
}

impl ShaderType {
    pub fn to_gl(&self) -> GLenum {
        match self {
            ShaderType::FRAGMENT => gl::FRAGMENT_SHADER,
            ShaderType::VERTEX => gl::VERTEX_SHADER,
        }
    }
}
