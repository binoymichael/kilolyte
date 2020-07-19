use std::io;

fn main() {
    println!("Hello, world!");
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{} bytes read", n);
                println!("{}", input);
            }
            Err(error) => println!("{}", error),
        }
    }
}
