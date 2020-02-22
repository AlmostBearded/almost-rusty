use config_struct::StructOptions;
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    vec::Vec,
};

use voca_rs::*;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn Error>> {
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
    fs::remove_dir_all(Path::new("src/assets"))?;

    // create modules
    let dirs = paths.iter().filter(|&p| p.is_dir());
    for dir in dirs {
        // create module source
        let mut modrs_source = String::new();
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if path.is_dir() {
                // declare submodule
                modrs_source = format!("{}pub mod {};\n", modrs_source, file_name);
            } else {
                // declare asset
                let file_name = file_name.replace(".", "_").to_uppercase();
                let file_stem = path.file_stem().unwrap().to_str().unwrap();
                let file_extension = path
                    .extension()
                    .expect(&format!(
                        "File extension can't be extracted from {}",
                        path.to_str().unwrap()
                    ))
                    .to_str()
                    .unwrap();

                if file_extension.to_lowercase() == "toml" {
                    // need to provide '/' path strings because of a bug in the config_struct package
                    config_struct::create_config(
                        &path,
                        Path::new("src")
                            .join(dir)
                            .join(Path::new(&[file_stem, ".rs"].concat())),
                        &StructOptions::serde_default(),
                    )
                    .expect(&format!(
                        "Failed to create config from {}",
                        path.to_str().unwrap()
                    ));
                    modrs_source = format!(
                        "{}mod {};\npub use {}::Config as {};\n",
                        modrs_source,
                        file_stem,
                        file_stem,
                        case::pascal_case(file_stem)
                    );
                } else {
                    modrs_source = format!("{}pub static {} : u8 = 0;\n", modrs_source, file_name);
                }
            }
        }

        // create module folder
        let dir = Path::new("src").join(&dir);
        fs::create_dir_all(&dir)?;

        // create module file
        let modrs_path = dir.join("mod.rs");
        fs::write(modrs_path, modrs_source)?;
    }
    Ok(())
}