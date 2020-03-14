use std::path::Path;
use std::vec::Vec;
use std::fs::read_to_string;
use ron::de::from_str;
use crate::graphics::gl::shader::Shader;

#[derive(serde::Deserialize, Debug)]
struct WindowConfig {
    title: String,
}

#[derive(serde::Deserialize, Debug)]
enum ShaderType {
    FRAGMENT,
    VERTEX,
}

#[derive(serde::Deserialize, Debug)]
struct ShaderMeta {
    shader_type: ShaderType,
}

pub struct ShaderAssetManager {
    pub meta_paths: Vec<&'static str>
}

impl ShaderAssetManager {
    pub fn load_all(&self) -> Vec<Shader> {
        let shaders: Vec<Shader>;
        for meta_path in self.meta_paths {
            let contents = read_to_string(meta_path).unwrap();
            let meta: ShaderMeta = from_str(&contents).unwrap();
            println!("{:?}", meta);
        }
        shaders;
    }
}