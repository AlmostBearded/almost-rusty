#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(dead_code)]

use std::borrow::Cow;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub struct Config {
    pub title: Cow<'static, str>,
}

pub const CONFIG: Config = Config {
    title: Cow::Borrowed("AlmostRusty"),
};

#[cfg(debug_assertions)]
impl Config {
    pub fn load() -> Cow<'static, Self> {
        let filepath = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/config/window.toml");
        Self::load_from(filepath.as_ref()).expect("Failed to load Config.")
    }

    pub fn load_from(filepath: &::std::path::Path) -> Result<Cow<'static, Self>, Box<dyn ::std::error::Error>> {
        let file_contents = ::std::fs::read_to_string(filepath)?;
        let result: Self = ::toml::from_str(&file_contents)?;
        Ok(Cow::Owned(result))
    }
}

#[cfg(not(debug_assertions))]
impl Config {
    #[inline(always)]
    pub fn load() -> Cow<'static, Self> {
        Cow::Borrowed(&CONFIG)
    }

    #[inline(always)]
    pub fn load_from(_: &::std::path::Path) -> Result<Cow<'static, Self>, Box<dyn ::std::error::Error>> {
        Ok(Cow::Borrowed(&CONFIG))
    }
}
