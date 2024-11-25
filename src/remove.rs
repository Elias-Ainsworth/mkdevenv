use crate::cli::DirenvArgs;
use std::{fs, io, path::Path, process::Command};

pub fn remove(args: &DirenvArgs) -> Result<(), io::Error> {
    if args.target.as_ref().is_some_and(|x| {
        Path::new(format!("{}/.direnv", x.display()).as_str()).exists()
            && Path::new(format!("{}/.devenv", x.display()).as_str()).exists()
    }) {
        if let Some(target) = args.target.as_ref() {
            let (direnv, devenv) = (
                format!("{}.direnv", target.display()),
                format!("{}.devenv", target.display()),
            );

            println!("Attempting to remove: {}", &direnv);
            fs::remove_dir_all(&direnv)?;
            println!("Attempting to remove: {}", &devenv);
            fs::remove_dir_all(&devenv)?;

            Command::new("direnv").arg("revoke").arg(target).spawn()?;
        } else {
            eprintln!("Error: target is not provided");
        }
    }
    Ok(())
}
