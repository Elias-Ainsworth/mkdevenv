use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Subcommand, ValueEnum, Debug, Clone, PartialEq, Eq)]
pub enum ShellCompletion {
    Bash,
    Zsh,
    Fish,
}

#[derive(Args, Debug, PartialEq, Eq)]
pub struct GenerateArgs {
    #[arg(value_enum, help = "Type of Shell completion to generate")]
    pub shell: ShellCompletion,
}

#[derive(Args, Debug, PartialEq, Eq)]
pub struct InitArgs {
    #[arg(name = "PATH", help = "Directory to initialize")]
    pub target: Option<PathBuf>,
    #[arg(name = "LANGUAGE", help = "Language to initialize")]
    pub language: Option<String>,
}

#[derive(Args, Debug, PartialEq, Eq)]
pub struct DirenvArgs {
    #[arg(name = "PATH", help = "Directory to (un)initialize")]
    pub target: Option<PathBuf>,
}

#[derive(Subcommand, Debug, PartialEq, Eq)]
pub enum MkdevenvSubcommand {
    #[command(name = "generate", about = "Generate shell completions", hide = true)]
    Generate(GenerateArgs),
    #[command(
        name = "init",
        about = "Intialize dev environment for specified language in specified directory."
    )]
    Init(InitArgs),
    #[command(
        name = "remove",
        about = "Remove devenv and direnv files and revoke direnv permission."
    )]
    Remove(DirenvArgs),
    //TODO: Add 'allow' and 'revoke' commands.
}

#[derive(Parser, Debug)]
#[command(
    name = "mkdevenv",
    about = "Initialize development environments based on templates using devenv and direnv"
)]
pub struct MkdevenvArgs {
    #[command(subcommand)]
    pub command: Option<MkdevenvSubcommand>,
}
