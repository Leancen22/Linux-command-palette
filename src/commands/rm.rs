use std::fs;
use std::path::{Path};

pub fn rm_command(dir: &str) -> std::io::Result<()> {
    
    let path = Path::new(dir);

    if path.is_file() {
        fs::remove_file(path).expect("Error");
        return Ok(())
    } else if path.is_dir() {
        fs::remove_dir_all(path).expect("No se pudo borrar la carpeta");
        return Ok(())
    }
    Ok(())
}

