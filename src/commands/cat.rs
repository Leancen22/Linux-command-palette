use std::path::{Path};
use std::io::{BufRead, BufReader};

pub fn cat_command(dir: &str) -> std::io::Result<()> {
    let path = Path::new(dir);
    let data = std::fs::metadata(path);

    if !path.is_file() {
        println!("El valor proporcionado no corresponde a un archivo");
        return Ok(())
    } else {

        let file = std::fs::File::open(path)?;
        let lector = BufReader::new(file);

        for line in lector.lines() {
            println!("{}", line?);
        }
    }
    Ok(())
}