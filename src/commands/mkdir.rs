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

/*use std::fs;

fn main() -> std::io::Result<()> {
    let path = "test";

    let entries = fs::read_dir(".")?;
    let mut is_file = false;
    let mut is_dir = false;

    for entry in entries {
        let entry = entry?;
        if entry.file_name() == path {
            let metadata = entry.metadata()?;
            if metadata.is_file() {
                is_file = true;
            } else if metadata.is_dir() {
                is_dir = true;
            }
        }
    }

    if is_file {
        println!("'{}' se refiere a un archivo.", path);
        // Código para crear un archivo
    } else if is_dir {
        println!("'{}' se refiere a una carpeta.", path);
        // Código para crear una carpeta
    } else {
        println!("'{}' no se refiere a un archivo ni a una carpeta.", path);
        // Código para manejar el caso en que no se encuentre el archivo o la carpeta
    }

    Ok(())
}
 */