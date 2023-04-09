use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub fn vi_command(file: &str) -> io::Result<()> {
    let dir = std::env::current_dir()?;
    let path = Path::new(file);
    let mut contents = fs::read_to_string(&path)?;

    loop {
        print!("{}", contents);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            ":w" => {
                fs::write(&path, contents)?;
                break;
            }
            ":q" => break,
            _ => contents = input,
        }
    }

    Ok(())
}
