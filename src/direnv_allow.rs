use std::{io, path::PathBuf, process::Command};

pub fn direnv_allow(path: Option<PathBuf>) -> Result<(), io::Error> {
    if let Some(target) = path {
        Command::new("direnv").arg("allow").arg(target).spawn()?;
    }
    Ok(())
}
