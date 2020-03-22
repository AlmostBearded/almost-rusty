use gl::types::GLenum;
use ron::de::from_str;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use std::vec::Vec;
use crate::graphics::gl::shader::shader_type::ShaderType;

#[derive(serde::Deserialize, Debug)]
pub struct ShaderMeta {
    pub id: String,
    pub shader_type: ShaderType,
    pub source_path: PathBuf,
}

pub fn load_shader_metas_from_paths(meta_paths: &Vec<&Path>) -> Vec<ShaderMeta> {
    let mut shaders: Vec<ShaderMeta> = Vec::new();
    for meta_path in meta_paths {
        let contents = read_to_string(meta_path).unwrap();
        let mut meta: ShaderMeta = from_str(&contents).expect(&format!(
            "Failed to parse shader meta from file '{}'",
            meta_path.display()
        ));

        // make path relative to asset folder
        // TODO: handle already asset folder relative paths
        meta.source_path = meta_path.parent().unwrap().join(meta.source_path);
        shaders.push(meta);
    }
    shaders
}
