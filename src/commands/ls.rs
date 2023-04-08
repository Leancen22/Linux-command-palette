use std::io::Result;
use std::fs;
use termion::{color, style};

pub fn ls_command(dir: &str) -> Result<()> {
    let entries = fs::read_dir(dir)?;
    let mut names: Vec<String> = entries
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect();
    names.sort();
    for name in names {
        let path = std::path::Path::new(&name);
        if path.is_dir() {
            print!("{}", color::Fg(color::Green));
        }
        print!("{} ", name);
        print!("{}", style::Reset);
    }
    println!("");
    Ok(())
}
