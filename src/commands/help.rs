use termion::color::{Fg, Blue};

pub fn help_command() {
    println!("Comandos disponibles:");
    println!("- {}cd{}", Fg(Blue), Fg(termion::color::Reset));
    println!("- {}ls{}", Fg(Blue), Fg(termion::color::Reset));
    println!("- {}mkdir{}", Fg(Blue), Fg(termion::color::Reset));
    println!("- {}pwd{}", Fg(Blue), Fg(termion::color::Reset));
    println!("- {}touch{}", Fg(Blue), Fg(termion::color::Reset));
    println!("- {}rm{}", Fg(Blue), Fg(termion::color::Reset));
    println!("- {}mv{}", Fg(Blue), Fg(termion::color::Reset));

}   