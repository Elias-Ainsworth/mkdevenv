use std::io;

use crate::cli::{GenerateArgs, MkdevenvArgs, ShellCompletion};
use clap::CommandFactory;
use clap_complete::{generate, Shell};

pub fn completions(args: &GenerateArgs) -> Result<(), io::Error> {
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
    Ok(())
}
