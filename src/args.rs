use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Subcommand)]
pub enum SubCommand {
    /// Initiate the configuration at the default location
    InitConfig,
    /// <unimplemented> Edit the configuration
    Config,
    /// Run rofi-obsidian, default behaviour
    Run,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub sub: Option<SubCommand>,
}
