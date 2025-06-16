use rand::Rng;
use std::io;
use std::io::Write;

fn main() {
    eprintln!("write youre names");

    let mut names: Vec<String> = Vec::new();
    loop {
        loop {
            print!("> ");
            io::stdout().flush().expect("PROGRAM BROC");
            let buffer = io::stdin();
            let mut input = String::new();
            buffer.read_line(&mut input).expect("bad input");

            match input.trim() {
                "done" => break,
                "" => println!("enter a name!"),
                _ => names.push(input.to_string()),
            }
        }
        let randomnumber = rand::rng().random_range(..names.len());
        println!("{}", names[randomnumber])
    }
}
