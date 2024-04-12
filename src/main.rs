use serde_json;
use std::collections::HashMap;
use std::os::unix::process::CommandExt;
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

    let buf = fs::read_to_string(flatpak_conf).unwrap();

    let vaults: VaultDB = serde_json::from_str(&buf).unwrap();

    vaults
        .vaults
        .into_iter()
        .map(|(_, vault)| vault.path)
        .collect()
}

fn main() {
    let rofi_state: u8 = env::var("ROFI_RETV").unwrap().parse().unwrap();
    let rofi_data: String = env::var("ROFI_DATA").unwrap_or_default();

    match rofi_state {
        0 => {
            let vaults = get_known_vaults();

            vaults.iter().for_each(|vault| {
                println!("{}", vault);
                println!("\0data\x1f{}", vault);
            });
        }
        1 => {
            let vault = PathBuf::from(rofi_data);
            let vault = vault.file_name().unwrap();
            let vault = vault.to_str().unwrap();

            std::process::Command::new("bash")
                .args([
                    "-c",
                    &format!(
                        "coproc (md.obsidian.Obsidian \"obsidian://open?vault={vault}\" > /dev/null 2>&1)",
                    ),
                ])
                .exec();
        }
        _ => unimplemented!(),
    };
}
