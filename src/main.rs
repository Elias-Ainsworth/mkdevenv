use clap::Parser;
use cli::{MkdevenvArgs, MkdevenvSubcommand};
use mkdevenv::*;
use std::io;

fn main() -> Result<(), io::Error> {
    let mkargs = MkdevenvArgs::parse();
    if let Some(command) = mkargs.command.as_ref() {
        match command {
            MkdevenvSubcommand::Generate(args) => {
                completions(args)?;
            }
            MkdevenvSubcommand::Init(args) => {
                rec_copy(args)?;
            }
            MkdevenvSubcommand::Remove(args) => remove(args)?,
        }
    }
    Ok(())
}
