use std::ffi::CString;
use std::fs;

use gl;

use crate::graphics::gl::shader::Shader;

#[derive(Debug)]
pub struct ShaderAsset {
    pub path: &'static str,
    pub meta: ShaderAssetMeta,
}

#[derive(Debug)]
pub struct ShaderAssetMeta {
    pub shader_type: &'static str
}

impl ShaderAsset {
    pub fn load(&self) -> Result<Shader, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(self.path)
            .expect(&format!("Cannot read file {} to string", self.path));

        let shader_type = match self.meta.shader_type {
            "fragment" => gl::FRAGMENT_SHADER,
            "vertex" => gl::VERTEX_SHADER,
            _ => panic!(format!("Invalid shader type '{}' in meta data", self.meta.shader_type))
        };

        Ok(Shader::from_source(&CString::new(content)?, shader_type)?)
    }
}