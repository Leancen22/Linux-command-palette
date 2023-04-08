use std::io::Result;
use std::path::{Path, PathBuf};

pub fn cd_command(dir: &str) -> Result<()> {
    let path = Path::new(dir);
    if !path.is_dir() {
        if dir == ".." {
            let mut current_path = PathBuf::from(std::env::current_dir()?);
            current_path.pop();
            std::env::set_current_dir(current_path)?;
            return Ok(());
        } else {
            println!("No such directory '{dir}'");
            return Ok(());
        }
    }
    std::env::set_current_dir(&path)?;
    Ok(())
}
