use crate::assets::shader_asset::{ShaderAsset, ShaderAssetMeta};

pub static TRIANGLE_FRAG: ShaderAsset = ShaderAsset {
    path: "assets/shaders/triangle.frag",
    meta: ShaderAssetMeta {
        shader_type: "fragment"
    },
};

pub static TRIANGLE_VERT: u8 = 0;
