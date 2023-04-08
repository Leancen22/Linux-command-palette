use std::io::{self, Write};

mod commands {
    pub mod ls;
    pub mod cd;
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
            Some("exit") => {
                break;
            },
            Some(command) => {
                println!("Unknown command: {}", command);
            },
            None => {},
        }
    }

    Ok(())
}
