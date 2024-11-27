use crate::{cargo_init, cli::InitArgs, direnv::direnv_allow};
use clap::error::Result;
use dircpy::*;
use std::io;

pub fn initialize(args: &InitArgs) -> Result<(), io::Error> {
    if let (Some(target), Some(language)) = (args.target.as_ref(), args.language.as_ref()) {
        if let Some(source) = template_path(language) {
            CopyBuilder::new(source, target).run()?;
        }
    }
    direnv_allow(args)?;
    cargo_init(args)?;
    Ok(())
}

fn template_path(language: &str) -> Option<String> {
    let template_dir = "templates";
    let template = match language {
        "rs" | "rust" => "rust",
        "py" | "python" => "python",
        "js" | "ts" | "javascript" => "javascript",
        "cpp" | "cplusplus" => "cpp",
        // TODO: Add more languages.
        _ => "",
    };
    Some(format!("{}/{}", template_dir, template))
}
