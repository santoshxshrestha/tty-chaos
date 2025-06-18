#![allow(unused)]
use crossterm::style::Print;
use crossterm::{execute, terminal};
use std::io::stdout;
use std::thread::sleep;
use std::{io, time};

fn main() -> io::Result<()> {
    //blocks certain features of the terminal
    // https://docs.rs/crossterm/latest/crossterm/terminal/index.html#raw-mode
    terminal::enable_raw_mode()?;
    execute!(stdout(), Print("wtf".to_string()))?;
    sleep(time::Duration::from_secs(5));
    print!("lol eof");

    //if this is not done then the terminal will malfunction and remains same untill reopened
    terminal::disable_raw_mode()?;
    Ok(())
}
