use std::io::{Result};

pub fn pwd_command() -> Result<()> {
    let path = std::env::current_dir()?;
    let pwd = path.to_str().unwrap_or(".");
    println!("{}", pwd);
    return Ok(());
}
