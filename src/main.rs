use std::io;
use std::io::Read;
use std::os::unix::io::AsRawFd;
use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

fn main() {
    println!("Hello, world!");

    // FIXME : Should stdin by mut?
    let mut stdin = io::stdin();

    let fd = stdin.as_raw_fd();

    let mut termios = Termios::from_fd(fd).unwrap();
    termios.c_lflag &= !(ECHO | ICANON);
    tcsetattr(fd, TCSANOW, &termios).unwrap();

    let mut character = [0];
    loop {
        match stdin.read(&mut character) {
            Ok(n) => {
                println!("{} bytes read", n);
                println!("{:?}", character[0] as char);
            }
            Err(error) => println!("{}", error),
        }
    }
}
