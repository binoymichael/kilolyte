use std::io::{self, Read, Write};
use std::os::unix::io::AsRawFd;
use termios::{tcsetattr, Termios, ECHO, ICANON, ICRNL, IEXTEN, ISIG, OPOST, TCSANOW, VMIN, VTIME};

fn main() {
    println!("Hello, world!");

    // FIXME : Should stdin by mut?
    let mut stdin = io::stdin();

    let fd = stdin.as_raw_fd();

    // FIXME : What does unwrap do?
    let mut termios = Termios::from_fd(fd).unwrap();
    termios.c_lflag &= !(ECHO | ICANON | ISIG | IEXTEN);
    termios.c_iflag &= !(ICRNL);
    termios.c_oflag &= !(OPOST);
    termios.c_cc[VMIN] = 0;
    termios.c_cc[VTIME] = 1;
    tcsetattr(fd, TCSANOW, &termios).unwrap();

    // FIXME : What is happening here?
    let mut character = [0];
    loop {
        character[0] = 0;
        match stdin.read(&mut character) {
            Ok(_) => {
                //print!("{} bytes read\r\n", n);
                if character[0].is_ascii_control() {
                    print!("{:?}\r\n", character[0] as char); // FIXME: How does 'as char' work?
                } else {
                    print!("{:?}\r\n", character[0] as char); // FIXME: How does 'as char' work?
                                                              // FIXME - how do I write 'q' instead of 113 here
                    if character[0] == 113 {
                        break;
                    }
                }
                //print!("{:?}\r\n", character[0]); // FIXME : Why do I have to use {:?}
            }
            Err(error) => print!("{}\r\n", error),
        }
        io::stdout().flush().unwrap();
    }
}
