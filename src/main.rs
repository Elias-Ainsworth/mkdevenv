use clap::Parser;
use cli::{MkdevenvArgs, MkdevenvSubcommand};
use mkdevenv::*;
use std::io;

fn main() -> Result<(), io::Error> {
    let args = MkdevenvArgs::parse();
    if let Some(command) = args.command.as_ref() {
        match command {
            MkdevenvSubcommand::Generate(args) => {
                generate_completions(args)?;
            }
            MkdevenvSubcommand::Init(args) => {
                initialize(args)?;
            }
            MkdevenvSubcommand::Remove(args) => remove(args)?,
            MkdevenvSubcommand::Allow(args) => direnv_allow(args)?,
            MkdevenvSubcommand::Revoke(args) => direnv_revoke(args)?,
        }
    }
    // TODO: Handle errors in a way that doesn't throw false-positives
    direnv_allow(args.direnv_allow);
    direnv_revoke(args.direnv_revoke);
    cargo_init(args.cargo_init);
    Ok(())
}
