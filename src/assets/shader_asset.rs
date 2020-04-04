use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use std::vec::Vec;

use ron::de::from_str;

use crate::graphics::gl::shader::shader_type::ShaderType;

#[derive(Debug)]
pub struct ShaderAsset {
    pub asset_path: PathBuf,
    pub vertex_shader_source: CString,
    pub fragment_shader_source: CString,
}

#[derive(serde::Deserialize, Debug)]
struct ShaderMeta {
    pub id: String,
    pub vertex_shader_path: PathBuf,
    pub fragment_shader_path: PathBuf,
}

pub fn load_shader_assets(paths: &Vec<&Path>) -> (HashMap<String, u8>, Vec<ShaderAsset>) {
    assert!(
        paths.len() < 255,
        "Exceeded maximum number of shaders (255)"
    );
    let mut id_lookup_map = HashMap::<String, u8>::new();
    let mut assets = Vec::<ShaderAsset>::new();
    for path in paths {
        log::info!("Loading shader asset '{}'", path.display());

        let contents = read_to_string(path).unwrap();
        let meta: ShaderMeta = from_str(&contents).expect(&format!(
            "Failed to parse shader asset from file '{}'",
            path.display()
        ));

        // make path relative to asset folder
        // TODO: handle already asset folder relative paths
        let vertex_shader_path = path.parent().unwrap().join(meta.vertex_shader_path);
        let fragment_shader_path = path.parent().unwrap().join(meta.fragment_shader_path);

        let vertex_shader_source = read_shader_file(&vertex_shader_path);
        let fragment_shader_source = read_shader_file(&fragment_shader_path);

        id_lookup_map.insert(meta.id, id_lookup_map.len() as u8);
        assets.push(ShaderAsset {
            asset_path: path.to_path_buf(),
            vertex_shader_source,
            fragment_shader_source,
        });
    }
    (id_lookup_map, assets)
}

fn read_shader_file(path: &Path) -> CString {
    log::debug!("Reading shader file '{}'", path.display());
    let source =
        read_to_string(path).expect(&format!("Failed to read shader file '{}'", path.display()));
    log::debug!("Shader source:\n{}", source);
    return CString::new(source).unwrap();
}
