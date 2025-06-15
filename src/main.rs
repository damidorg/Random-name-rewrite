use std::io;

use std::io::Write;

fn main() {
    eprintln!("write youre names");
    print!("> ");
    io::stdout().flush().expect("PROGRAM BROC");
    let buffer = io::stdin();
    let mut input = String::new();
    buffer.read_line(&mut input).expect("bad input");
    println!("{input}")
}
