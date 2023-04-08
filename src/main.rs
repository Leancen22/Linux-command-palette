use std::io::{self, Write};

mod commands {
    pub mod ls;
    pub mod cd;
    pub mod mkdir;
    pub mod pwd;
    pub mod touch;
    pub mod help;
    pub mod rm;
    pub mod mv;
}

fn main() -> std::io::Result<()> {
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let mut parts = input.trim().split_whitespace();

        match parts.next() {
            Some("ls") => {
                let dir = parts.next().unwrap_or(".");
                commands::ls::ls_command(dir)?;
            },
            Some("cd") => {
                let dir = parts.next().unwrap_or(".");
                commands::cd::cd_command(dir)?;
            },
            Some("rm") => {
                let dir = parts.next().unwrap_or(".");
                commands::rm::rm_command(dir)?;
            },
            Some("touch") => {
                let dir = parts.next().unwrap_or(".");
                commands::touch::touch_command(dir)?;
            },
            Some("pwd") => {
                commands::pwd::pwd_command().expect("Error reading the line");
            },
            Some("mkdir") => {
                let dir = parts.next().unwrap_or(".");
                commands::mkdir::mkdir_command(dir)?;
            },
            Some("mv") => {
                let file = parts.next().unwrap_or(".");
                let directory = parts.next().unwrap_or(".");
                commands::mv::mv_command(file, directory)?;
            }
            Some("exit") => {
                break;
            },
            Some("help") => {
                commands::help::help_command();
            },  
            Some(command) => {
                println!("Unknown command: {}", command);
            },
            None => {},
        }
    }

    Ok(())
}
