use crate::cli::InitArgs;
use dircpy::*;
use std::io;

pub fn rec_copy(args: &InitArgs) -> Result<(), io::Error> {
    if let (Some(target), Some(language)) = (args.target.as_ref(), args.language.as_ref()) {
        let source = format!(
            "/persist/home/elias-ainsworth/projects/dotfiles/templates/{}",
            language.trim()
        );
        CopyBuilder::new(source, target).run()?;
    }
    Ok(())
}
