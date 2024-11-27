use crate::{cli::DirenvArgs, direnv_revoke};
use std::{fs, io, path::Path};

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

            fs::remove_dir_all(&direnv)?;
            fs::remove_dir_all(&devenv)?;

            direnv_revoke(args)?;
        } else {
            eprintln!("Error: target is not provided");
        }
    }
    Ok(())
}
