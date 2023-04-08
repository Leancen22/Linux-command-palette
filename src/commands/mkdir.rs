use std::{io::Result};
use std::fs;
use std::path::{Path};

pub fn mkdir_command(dir: &str) -> Result<()> {
    let path = Path::new(dir);
    if !path.is_dir() {
        fs::create_dir_all(dir)?;
    } else {
        println!("Directorio '{dir}' ya existe");
        return Ok(());
    }
    Ok(())
}

