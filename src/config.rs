use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, create_dir_all, write},
    path::PathBuf,
};

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub enum DisplayName {
    #[default]
    VaultName,
    Path,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub display_name: DisplayName,
    pub source: Source,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    pub flatpak: bool,
    pub native: bool,
    pub additional_sources: Vec<String>,
}

impl Config {
    fn get_path() -> PathBuf {
        let conf_home = std::env::var("XDG_CONFIG_HOME").unwrap_or("~/.config".into());
        PathBuf::from(conf_home).join("rofi-obsidian/config.toml")
    }

    pub fn parse() -> Self {
        let path = Self::get_path();

        let conf = fs::read_to_string(path).unwrap_or_default();

        toml::from_str(&conf).unwrap_or_default()
    }

    pub fn write(&self) -> Result<PathBuf> {
        let path = Self::get_path();
        create_dir_all(path.parent().unwrap())?;
        write(path.clone(), toml::to_string(self)?)?;
        Ok(path)
    }
}

impl Default for Source {
    fn default() -> Self {
        Self {
            flatpak: true,
            native: true,
            additional_sources: vec![],
        }
    }
}
