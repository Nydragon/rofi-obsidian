use std::fmt::Display;

use clap::{Parser, ValueEnum};

use crate::config::DisplayName;

#[derive(Debug, Clone, Default, ValueEnum)]
pub enum SubCommand {
    /// Initiate the configuration at the default location
    InitConfig,
    /// <unimplemented> Edit the configuration
    Config,
    /// Run rofi-obsidian, default behaviour
    #[default]
    Run,
}

impl Display for SubCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sub = match *self {
            SubCommand::Run => "run",
            SubCommand::Config => "config",
            SubCommand::InitConfig => "init-config",
        };

        write!(f, "{}", sub)
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[clap(long, short, default_value_t = SubCommand::default())]
    pub command: SubCommand,
    #[clap()]
    pub selection: Option<String>,
    #[clap(short, long, help = "The style of the vault name")]
    pub name: Option<DisplayName>,
    #[clap(short, long, help = "The icon to display for each entry")]
    pub icon: Option<String>,
}
