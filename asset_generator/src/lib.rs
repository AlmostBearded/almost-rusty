use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    vec::Vec,
};

use config_struct::StructOptions;
use voca_rs::*;
use walkdir::WalkDir;

pub fn generate_database(src_root: &Path, dest_root: &Path) -> Result<(), Box<dyn Error>> {
    let mut paths = Vec::<PathBuf>::new();

    // find all asset file paths
    for entry in WalkDir::new("assets") {
        let entry = entry?;
        let path = entry.path();
        paths.push(path.to_path_buf());
    }

    // rerun build script if asset files change
    for path in &paths {
        let path = path
            .to_str()
            .expect("Paths may only contain UTF-8 characters");
        println!("cargo:rerun-if-changed={}", path);
    }

    // clear modules
    fs::remove_dir_all(Path::new("src/assets/database")).ok();

    // create modules
    let dirs = paths.iter().filter(|&p| p.is_dir());
    for dir in dirs {
        // create module source
        let mut modrs_code = String::new();
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if path.is_dir() {
                // declare submodule
                modrs_code = format!("{}pub mod {};\n", modrs_code, file_name);
            } else {
                // declare asset
                let file_extension = path
                    .extension()
                    .expect(&format!(
                        "File extension can't be extracted from {}",
                        path.to_str().unwrap()
                    ))
                    .to_str()
                    .unwrap()
                    .to_lowercase();

                let code = match file_extension.as_str() {
                    "meta" => "".to_string(),
                    "toml" => generate_toml_asset(src_root, path.as_path(), dest_root)?,
                    "frag" | "vert" => generate_shader_asset(path.as_path())?,
                    _ => panic!(format!(
                        "Unhandled asset file extension {} at file {}",
                        file_extension,
                        path.display()
                    )),
                };

                modrs_code = format!("{}\n{}", modrs_code, code)
            }
        }

        // create module folder
        let dir = Path::new("src/assets/database").join(dir.strip_prefix("assets").unwrap());
        fs::create_dir_all(&dir)?;

        // create module file
        let modrs_path = dir.join("mod.rs");
        fs::write(modrs_path, modrs_code)?;
    }
    Ok(())
}

fn generate_toml_asset(
    src_root: &Path,
    src_path: &Path,
    dest_root: &Path,
) -> Result<String, Box<dyn Error>> {
    let file_stem = src_path
        .file_stem()
        .expect(format!("Failed to get file stem for asset '{}'", src_path.display()).as_str())
        .to_str()
        .expect(
            format!(
                "Asset path '{}' contains invalid characters",
                src_path.display()
            )
            .as_str(),
        );

    let dest_path = dest_root
        .join(src_path.strip_prefix(src_root).unwrap())
        .with_extension("rs");

    config_struct::create_config(&src_path, dest_path, &StructOptions::serde_default()).expect(
        &format!(
            "Failed to create config from {}",
            src_path.to_str().unwrap()
        ),
    );

    Ok(format!(
        "mod {};\npub use {}::Config as {};",
        file_stem,
        file_stem,
        case::pascal_case(file_stem)
    ))
}

fn generate_shader_asset(src_path: &Path) -> Result<String, Box<dyn Error>> {
    let file_extension = src_path
        .extension()
        .unwrap()
        .to_str()
        .unwrap()
        .to_lowercase();
    let file_name = src_path.file_name().unwrap().to_str().unwrap();
    let var_name = file_name.to_uppercase().replace(".", "_");

    let shader_type = match file_extension.as_str() {
        "frag" => "fragment",
        "vert" => "vertex",
        _ => panic!(format!(
            "Unhandled shader file extension on asset '{}'",
            src_path.display()
        )),
    };

    Ok(format!(
        "pub static {}: crate::assets::shader_asset::ShaderAsset = crate::assets::shader_asset::ShaderAsset {{
    path: \"{}\",
    meta: crate::assets::shader_asset::ShaderAssetMeta {{
        shader_type: \"{}\",
    }},
}};",
        var_name, src_path.display().to_string().replace("\\", "/"), shader_type
    ))
}
