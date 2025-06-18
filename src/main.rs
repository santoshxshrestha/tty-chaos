use crossterm::terminal;
use std::thread::sleep;
use std::{io, time};

fn main() -> io::Result<()> {
    //blocks certain features of the terminal
    // https://docs.rs/crossterm/latest/crossterm/terminal/index.html#raw-mode
    terminal::enable_raw_mode()?;
    sleep(time::Duration::from_secs(5));

    Ok(())
}
