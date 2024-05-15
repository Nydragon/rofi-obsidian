use anyhow::Result;
use clap::Parser;
use std::collections::HashMap;
use std::path::Path;
use std::{env, fs};
use url::form_urlencoded::Serializer;

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

// Merge and deduplicate both the flatpak and native configuration files.
fn merge(v1: Vec<String>, mut v2: Vec<String>) -> Vec<String> {
    v1.iter().for_each(|i| {
        if !v2.contains(i) {
            v2.push(i.to_string());
        };
    });

    v2
}

fn get_known_vaults() -> Vec<String> {
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

    let flatpak_conf: String =
        format!("{home}/.var/app/md.obsidian.Obsidian/config/obsidian/obsidian.json");
    let native_conf: String = format!("{xdg_conf}/obsidian/obsidian.json");

    let mut vault_paths = merge(
        get_vaults(native_conf).unwrap_or_default(),
        get_vaults(flatpak_conf).unwrap_or_default(),
    );

    vault_paths.sort();
    vault_paths
}

fn rofi_main(state: u8) -> Result<()> {
    let rofi_info: String = env::var("ROFI_INFO").unwrap_or_default();

    match state {
        // Prompting which vault to open
        0 => {
            get_known_vaults().iter().for_each(|vault| {
                let name = Path::new(vault)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or_else(|| vault);
                println!("{name}\0info\x1f{vault}");
            });
        }
        // Opening the selected vault
        1 => {
            let path: String = Serializer::new(String::default())
                .append_pair("path", &rofi_info)
                .finish();
            let path = path.replace('+', "%20");

            #[cfg(debug_assertions)]
            eprintln!("{path}");

            let path = format!("obsidian://open?{path}");

            open::that_detached(path)?;
        }
        _ => unimplemented!(),
    };

    Ok(())
}
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[clap(short, long)]
    config: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.config {
        // TODO: Implement configuration manipulation with a subcommand
        unimplemented!()
    } else if let Ok(state) = env::var("ROFI_RETV") {
        rofi_main(state.parse()?)?;
    } else {
        println!(
            "Error: {} cannot be run outside of rofi.",
            env!("CARGO_BIN_NAME")
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{get_vaults, merge};

    #[test]
    fn test_merge() {
        let elem_test: String = "TEST".into();
        let elem_foo: String = "FOO".into();
        let elem_bar: String = "BAR".into();

        let v1 = vec![elem_test.clone(), elem_foo.clone()];
        let v2 = vec![elem_test.clone(), elem_bar.clone()];

        let v3 = merge(v1, v2);

        assert_eq!(v3.len(), 3);
        assert!(v3.contains(&elem_test));
        assert!(v3.contains(&elem_foo));
        assert!(v3.contains(&elem_bar));
    }

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
