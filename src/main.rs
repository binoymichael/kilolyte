use std::io::{self, Read, Stdin, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use termios::{tcsetattr, Termios, ECHO, ICANON, ICRNL, IEXTEN, ISIG, OPOST, TCSANOW, VMIN, VTIME};

fn main() {
    let mut stdin = io::stdin();
    enable_raw_mode(&stdin);

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
}

fn editor_key(stdin: &mut Stdin) -> char {
    let mut character = [0];
    &stdin.read(&mut character);
    character[0] as char
}

fn enable_raw_mode(stdin: &Stdin) {
    let fd = stdin.as_raw_fd();
    let mut termios = Termios::from_fd(fd).unwrap();
    termios.c_lflag &= !(ECHO | ICANON | ISIG | IEXTEN);
    termios.c_iflag &= !(ICRNL);
    termios.c_oflag &= !(OPOST);
    termios.c_cc[VMIN] = 0;
    termios.c_cc[VTIME] = 1;
    tcsetattr(fd, TCSANOW, &termios).unwrap();
}
