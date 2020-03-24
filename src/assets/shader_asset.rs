use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use std::vec::Vec;

use ron::de::from_str;

use crate::graphics::gl::shader::shader_type::ShaderType;

#[derive(Debug)]
pub struct ShaderAsset {
    pub shader_type: ShaderType,
    pub source_path: PathBuf,
}

#[derive(serde::Deserialize, Debug)]
struct ShaderMeta {
    pub id: String,
    pub shader_type: ShaderType,
    pub source_path: PathBuf,
}

pub fn load_shader_assets(meta_paths: &Vec<&Path>) -> (HashMap<String, u8>, Vec<ShaderAsset>) {
    assert!(
        meta_paths.len() < 255,
        "Exceeded maximum number of shaders (255)"
    );
    let mut id_lookup_map = HashMap::<String, u8>::new();
    let mut assets = Vec::<ShaderAsset>::new();
    for meta_path in meta_paths {
        let contents = read_to_string(meta_path).unwrap();
        let meta: ShaderMeta = from_str(&contents).expect(&format!(
            "Failed to parse shader meta from file '{}'",
            meta_path.display()
        ));

        // make path relative to asset folder
        // TODO: handle already asset folder relative paths
        let source_path = meta_path.parent().unwrap().join(meta.source_path);

        id_lookup_map.insert(meta.id, id_lookup_map.len() as u8);
        assets.push(ShaderAsset {
            shader_type: meta.shader_type,
            source_path,
        });
    }
    (id_lookup_map, assets)
}
