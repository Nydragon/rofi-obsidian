use serde_json;
use std::collections::HashMap;
use std::path::PathBuf;
use std::{env, fs};

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

fn get_known_vaults() -> Vec<String> {
    let user_home = env::var("HOME").unwrap();
    let xdg_conf = env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| format!("{user_home}/.config"));

    let flatpak_conf: String =
        format!("{user_home}/.var/app/md.obsidian.Obsidian/config/obsidian/obsidian.json");
    let default_conf: String = format!("{xdg_conf}/obsidian/obsidian.json");

    let buf = fs::read_to_string(flatpak_conf).unwrap();

    let vaults: VaultDB = serde_json::from_str(&buf).unwrap();

    let mut vault_paths: Vec<String> = vaults
        .vaults
        .into_iter()
        .map(|(_, vault)| vault.path)
        .collect();
    vault_paths.sort();
    vault_paths
}

fn main() {
    let rofi_state: u8 = env::var("ROFI_RETV").unwrap().parse().unwrap();
    let rofi_info: String = env::var("ROFI_INFO").unwrap_or_default();

    match rofi_state {
        0 => {
            get_known_vaults().iter().for_each(|vault| {
                println!("{vault}\0info\x1f{vault}");
            });
        }
        1 => {
            let vault = PathBuf::from(rofi_info);
            let vault = vault.file_name().unwrap().to_str().unwrap().to_string();
            let path = format!("obsidian://open?vault={vault}");

            open::that_detached(path).unwrap();
        }
        _ => unimplemented!(),
    };
}
