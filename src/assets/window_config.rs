use ron::de::from_str;
use std::fs::read_to_string;
use std::path::Path;

#[derive(serde::Deserialize, Debug)]
pub struct WindowConfig {
    pub title: String,
}

pub fn load_window_config_from_path(path: &Path) -> WindowConfig {
    let contents =
        read_to_string(path).expect(&format!("Failed to read file '{}'", path.display()));
    let window_config: WindowConfig = from_str(&contents).expect(&format!(
        "Failed to parse window config from file '{}'",
        path.display()
    ));
    window_config
}
