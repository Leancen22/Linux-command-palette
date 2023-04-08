use std::path::{Path};

pub fn mv_command(file: &str, directory: &str) -> std::io::Result<()> {
    let full_path = format!("{}/{}", directory, file);
    let path_file = Path::new(file);
    let path_dir = Path::new(directory);

    if (path_file.is_file() || path_file.is_dir()) && path_dir.is_dir() {
        std::fs::rename(file, full_path).expect("Error reading the line");
        return Ok(())
    } else if !path_dir.is_dir(){
        println!("{directory} no es un directorio valido");
        return Ok(()) 
    } else {
        println!("Error al mover, archivo o directorio no encontrado.");
    }
    Ok(())
}