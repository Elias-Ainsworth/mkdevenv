use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};
use cli::{MkdevenvArgs, MkdevenvSubcommand, ShellCompletion};
use rec_copy::rec_copy;
use remove::remove;
use std::io::{self};

pub mod cli;
pub mod direnv_allow;
pub mod rec_copy;
pub mod remove;

fn main() -> Result<(), io::Error> {
    let mkargs = MkdevenvArgs::parse();
    if let Some(command) = mkargs.command.as_ref() {
        match command {
            MkdevenvSubcommand::Generate(args) => {
                let mut cmd = MkdevenvArgs::command();
                match &args.shell {
                    ShellCompletion::Bash => {
                        generate(Shell::Bash, &mut cmd, "mkdevenv", &mut std::io::stdout());
                    }
                    ShellCompletion::Zsh => {
                        generate(Shell::Zsh, &mut cmd, "mkdevenv", &mut std::io::stdout());
                    }
                    ShellCompletion::Fish => {
                        generate(Shell::Fish, &mut cmd, "mkdevenv", &mut std::io::stdout());
                    }
                }
            }
            MkdevenvSubcommand::Init(args) => {
                rec_copy(args)?;
            }
            MkdevenvSubcommand::Remove(args) => remove(args)?,
        }
    }
    Ok(())
}
