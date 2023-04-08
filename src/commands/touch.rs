use std::fs;
use std::path::{Path};

pub fn touch_command(dir: &str) -> std::io::Result<()> {
    let path = Path::new(dir);
    if !path.is_file() {
        fs::File::create(path)?;
    } else {
        println!("The name '{dir}' already exist, select other.");
        return Ok(());
    }
    return Ok(());
}