use ron::de::from_str;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use std::vec::Vec;

use crate::graphics::gl::shader::shader_type::ShaderType;

#[derive(serde::Deserialize, Debug)]
pub struct ShaderProgramAsset {
    pub vertex_shader: PathBuf,
    pub fragment_shader: PathBuf
}

pub fn load_shader_program_metas_from_paths(meta_paths: &Vec<&Path>) -> Vec<ShaderProgramMeta> {
    let mut shaders = Vec::<ShaderProgramMeta>::new();
    for meta_path in meta_paths {
        let contents = read_to_string(meta_path).unwrap();
        let mut meta: ShaderProgramMeta = from_str(&contents).expect(&format!(
            "Failed to parse shader program meta from file '{}'",
            meta_path.display()
        ));

        // make path relative to asset folder
        // TODO: handle already asset folder relative paths
        let meta_dir = meta_path.parent().unwrap();
        meta.vertex_shader = meta_dir.join(meta.vertex_shader);
        meta.fragment_shader = meta_dir.join(meta.fragment_shader);
        shaders.push(meta);
    }
    shadersm
}
