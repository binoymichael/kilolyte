use crossterm::terminal;
use std::io::{self, Read, Stdin, Write};

fn main() -> Result<(), crossterm::ErrorKind> {
    terminal::enable_raw_mode()?;
    let mut stdin = io::stdin();
    loop {
        let c = editor_key(&mut stdin);
        match c {
            '\x00' => (),
            '\x11' => break,
            _ => {
                print!("{}", c);
                io::stdout().flush().unwrap();
            }
        }
    }
    terminal::disable_raw_mode()?;
    Ok(())
}

fn editor_key(stdin: &mut Stdin) -> char {
    let mut character = [0];
    &stdin.read(&mut character);
    character[0] as char
}
