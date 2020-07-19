use std::io;
use std::os::unix::io::AsRawFd;
use termios::*;

fn main() {
    println!("Hello, world!");
    let stdin = io::stdin();
    let fd = stdin.as_raw_fd();

    let mut termios = Termios::from_fd(fd).unwrap();
    termios.c_lflag &= !(ECHO);
    tcsetattr(fd, TCSANOW, &termios).unwrap();

    let mut input = String::new();
    loop {
        match stdin.read_line(&mut input) {
            Ok(n) => {
                println!("{} bytes read", n);
                println!("{}", input);
            }
            Err(error) => println!("{}", error),
        }
    }
}
