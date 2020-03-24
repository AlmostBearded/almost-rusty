use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;
use std::vec::Vec;

use ron::de::from_str;

#[derive(Debug)]
pub struct ShaderProgramAsset {
    pub vertex_shader_id: String,
    pub fragment_shader_id: String,
}

#[derive(serde::Deserialize, Debug)]
struct ShaderProgramMeta {
    pub id: String,
    pub vertex_shader_id: String,
    pub fragment_shader_id: String,
}

pub fn load_shader_program_assets(
    meta_paths: &Vec<&Path>,
) -> (HashMap<String, u8>, Vec<ShaderProgramAsset>) {
    assert!(
        meta_paths.len() < 255,
        "Exceeded maximum number of shader programs (255)"
    );
    let mut id_lookup_map = HashMap::<String, u8>::new();
    let mut assets = Vec::<ShaderProgramAsset>::new();
    for meta_path in meta_paths {
        let contents = read_to_string(meta_path).unwrap();
        let meta: ShaderProgramMeta = from_str(&contents).expect(&format!(
            "Failed to parse shader program meta from file '{}'",
            meta_path.display()
        ));
        id_lookup_map.insert(meta.id, id_lookup_map.len() as u8);
        assets.push(ShaderProgramAsset {
            vertex_shader_id: meta.vertex_shader_id,
            fragment_shader_id: meta.fragment_shader_id,
        });
    }
    (id_lookup_map, assets)
}
