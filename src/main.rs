use crate::config::DisplayName;
use anyhow::Result;
use args::{Args, SubCommand};
use clap::Parser;
use config::Config;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::{env, fs};
use url::form_urlencoded::Serializer;

mod args;
mod config;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct VaultDB {
    vaults: HashMap<String, Vault>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Vault {
    path: String,
    ts: usize,
    open: Option<bool>,
}

fn get_vaults(path: String) -> Result<Vec<String>> {
    let buf: String = fs::read_to_string(path)?;
    let vaults: VaultDB = serde_json::from_str(&buf)?;

    let vault_paths: Vec<String> = vaults
        .vaults
        .into_values()
        .map(|vault| vault.path)
        .collect();
    Ok(vault_paths)
}

fn build_sources(conf: &Config) -> Vec<String> {
    let mut sources: Vec<String> = vec![];
    let home = env::var("HOME").unwrap_or_default();
    let xdg_conf = env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
        if !home.is_empty() {
            format!("{home}/.config")
        } else {
            String::default()
        }
    });

    if home.is_empty() && xdg_conf.is_empty() {
        return vec![];
    }

    if conf.source.flatpak {
        sources.push(format!(
            "{home}/.var/app/md.obsidian.Obsidian/config/obsidian/obsidian.json"
        ));
    };

    if conf.source.native {
        sources.push(format!("{xdg_conf}/obsidian/obsidian.json"));
    };

    sources.append(&mut conf.source.additional_sources.clone());

    sources
}

fn get_known_vaults(conf: &Config) -> Vec<String> {
    let sources = build_sources(conf);

    let mut vaults = sources
        .iter()
        .flat_map(|path| get_vaults(path.to_string()).unwrap_or_default())
        .collect::<HashSet<String>>()
        .into_iter()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();

    vaults.sort();
    vaults
}

fn rofi_main(state: u8, conf: Config, _args: Args) -> Result<()> {
    let rofi_info: String = env::var("ROFI_INFO").unwrap_or_default();

    match state {
        // Prompting which vault to open
        0 => {
            get_known_vaults(&conf).iter().for_each(|vault| {
                let name = match conf.display_name {
                    DisplayName::VaultName => Path::new(vault)
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or_else(|| vault),
                    DisplayName::Path => vault,
                };

                println!("{name}\0info\x1f{vault}");
            });
        }
        // Opening the selected vault
        1 => {
            let path: String = Serializer::new(String::default())
                .append_pair("path", &rofi_info)
                .finish()
                .replace('+', "%20");

            #[cfg(debug_assertions)]
            eprintln!("{path}");

            let path = format!("obsidian://open?{path}");

            open::that_detached(path)?;
        }
        _ => unimplemented!(),
    };

    Ok(())
}

fn main() -> Result<()> {
    let conf = config::Config::parse();
    let args = Args::parse();

    match args.sub {
        Some(SubCommand::InitConfig) => {
            let location = conf.write()?;
            println!("Config written to: {:?}", location);
        }
        Some(SubCommand::Config) => {
            unimplemented!()
        }
        Some(SubCommand::Run) | None => {
            if let Ok(state) = env::var("ROFI_RETV") {
                let state = state.parse()?;
                rofi_main(state, conf, args)?;
            } else {
                println!(
                    "Error: {} cannot be run outside of rofi.",
                    env!("CARGO_BIN_NAME")
                );
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::get_vaults;

    #[test]
    fn test_base_json() {
        let paths = get_vaults("./test_assets/base.json".into()).unwrap();

        assert_eq!(paths.len(), 2);
    }

    #[test]
    fn test_extra_fields_json() {
        let paths = get_vaults("./test_assets/extra_fields.json".into()).unwrap();

        assert_eq!(paths.len(), 2);
    }
}
