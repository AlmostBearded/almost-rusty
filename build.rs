use std::{
    fs,
    path::{Path, PathBuf},
    vec::Vec,
};

use walkdir::WalkDir;

fn main() {
    let mut paths = Vec::<PathBuf>::new();

    // find all asset file paths
    for entry in WalkDir::new("assets") {
        let entry = entry.unwrap();
        let path = entry.path();
        paths.push(path.to_path_buf());
    }

    // rerun build script if asset files change
    for path in &paths {
        println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
    }

    // clear modules
    fs::remove_dir_all(Path::new("src/assets")).unwrap();

    // create modules
    let dirs = paths.iter().filter(|&p| p.is_dir());
    for dir in dirs {
        // create module source
        let mut modrs_source = String::new();
        for entry in fs::read_dir(&dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if path.is_dir() {
                // declare submodule
                modrs_source = format!("{}pub mod {};\n", modrs_source, file_name);
            } else {
                // declare asset
                let file_name = file_name.replace(".", "_").to_uppercase();
                modrs_source = format!("{}pub static {} : u8 = 0;\n", modrs_source, file_name);
            }
        }

        // create module folder
        let dir = Path::new("src").join(&dir);
        fs::create_dir_all(&dir).unwrap();

        // create module file
        let modrs_path = dir.join("mod.rs");
        fs::write(modrs_path, modrs_source).unwrap();
    }
}
