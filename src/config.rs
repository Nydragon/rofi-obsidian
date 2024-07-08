use anyhow::{anyhow, Result};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{self, create_dir_all, write},
    path::PathBuf,
};

#[derive(Serialize, Deserialize, ValueEnum, Default, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DisplayName {
    #[default]
    VaultName,
    Path,
    Unique,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub display_name: DisplayName,
    pub source: Source,
    #[serde(default = "default_icon")]
    pub icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    pub flatpak: bool,
    pub native: bool,
    pub additional_sources: Vec<String>,
}

impl Config {
    fn get_config_home() -> Result<PathBuf> {
        env::var("XDG_CONFIG_HOME")
            .or_else(|_| env::var("HOME").map(|home| format!("{home}/.config")))
            .map(PathBuf::from)
            .map_err(|_| anyhow::Error::msg("Unable to find XDG_CONFIG_HOME or HOME"))
    }

    fn get_path() -> Result<PathBuf> {
        Config::get_config_home().map(|path| path.join("rofi-obsidian/config.toml"))
    }

    fn get_path_folder() -> Result<PathBuf> {
        Config::get_config_home().map(|path| path.join("rofi-obsidian"))
    }

    pub fn parse() -> Self {
        Self::get_path()
            .and_then(|path| fs::read_to_string(path).map_err(|e| anyhow!(e)))
            .and_then(|conf| toml::from_str(&conf).map_err(|e| anyhow!(e)))
            .unwrap_or_default()
    }

    pub fn write(&self) -> Result<PathBuf> {
        Self::get_path_folder().and_then(|path| {
            create_dir_all(&path)?;

            let path = path.join("config.toml");

            write(&path, toml::to_string(self)?)?;

            Ok(path)
        })
    }
}

fn default_icon() -> String {
    "obsidian".to_string()
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

impl Default for Config {
    fn default() -> Self {
        Self {
            display_name: DisplayName::default(),
            source: Source::default(),
            icon: "obsidian".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Config;

    #[test]
    fn test_config_default() {
        let conf = Config::default();

        assert_eq!(conf.icon, "obsidian");
    }
}
