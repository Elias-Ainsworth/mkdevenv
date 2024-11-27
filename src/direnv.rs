use std::{
    fs::{self, File},
    io::{self, Read},
    path::{Path, PathBuf},
    process::Command,
};

use crate::{DirenvArgs, InitArgs};

pub trait DirPath {
    fn parse_path(&self) -> Result<&Path, io::Error>;
}
pub trait Lang {
    fn parse_lang(&self) -> Result<&str, io::Error>;
}

impl DirPath for Option<PathBuf> {
    fn parse_path(&self) -> Result<&Path, std::io::Error> /*Result<&Path, std::io::Error> */ {
        match self.as_ref() {
            Some(target) => Ok(target),
            None => Err(io::Error::new(io::ErrorKind::NotFound, "target is missing")),
        }
    }
}
impl Lang for Option<PathBuf> {
    fn parse_lang(&self) -> Result<&str, io::Error> {
        Ok("")
    }
}

impl DirPath for &InitArgs {
    fn parse_path(&self) -> Result<&Path, std::io::Error> {
        match self.target.as_ref() {
            Some(target) => Ok(target),
            None => Err(io::Error::new(io::ErrorKind::NotFound, "target is missing")),
        }
    }
}

impl Lang for &InitArgs {
    fn parse_lang(&self) -> Result<&str, io::Error> {
        match self.language.as_ref() {
            Some(target) => Ok(target.as_str()),
            None => Err(io::Error::new(io::ErrorKind::NotFound, "target is missing")),
        }
    }
}

impl DirPath for &DirenvArgs {
    fn parse_path(&self) -> Result<&Path, std::io::Error> {
        match self.target.as_ref() {
            Some(target) => Ok(target.as_path()),
            None => Err(io::Error::new(io::ErrorKind::NotFound, "target is missing")),
        }
    }
}

pub fn direnv_allow<T: DirPath>(arg: T) -> Result<(), io::Error> {
    let path = arg
        .parse_path()
        .map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))?;
    Command::new("direnv")
        .arg("allow")
        .arg(path.to_path_buf())
        .spawn()?;
    Ok(())
}

pub fn direnv_revoke<T: DirPath>(arg: T) -> Result<(), io::Error> {
    let path = arg
        .parse_path()
        .map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))?;
    Command::new("direnv").arg("revoke").arg(path).spawn()?;
    Ok(())
}

pub fn cargo_init<T: DirPath + Lang>(args: T) -> Result<(), io::Error> {
    let lang = args.parse_lang()?;
    let path = args.parse_path()?;
    if !lang.is_empty() && (lang.contains("rust") | lang.contains("rs")) {
        Command::new("direnv")
            .arg("exec")
            .arg(path)
            .arg("cargo")
            .arg("init")
            .spawn()?;
    } else if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_path = entry.path();
            if file_path.is_file() {
                let mut contents = String::new();
                if let Ok(mut file) = File::open(file_path) {
                    if file.read_to_string(&mut contents).is_ok()
                        && contents.contains("cargo") | contents.contains("rust")
                    {
                        Command::new("direnv")
                            .arg("exec")
                            .arg(path)
                            .arg("cargo")
                            .arg("init")
                            .spawn()?;
                    }
                }
            }
        }
    }
    Ok(())
}
