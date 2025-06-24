use rand::Rng;
use std::io;
use std::io::Write;

fn main() {
    eprintln!("write youre names");

    let mut names: Vec<String> = Vec::new();
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

    print!("> ");
    io::stdout().flush().expect("PROGRAM BROC");
    let buffer = io::stdin();
    let mut input = String::new();
    buffer.read_line(&mut input).expect("bad input");

    match input.trim() {
        "1" => last_standing(names),
        _ => names.push(input.to_string()),
    }
}
fn last_standing(mut names: Vec<String>) {
    loop {
        let randomnumber = rand::rng().random_range(..names.len());
        println!("{}", names[randomnumber]);
        names.remove(randomnumber);
        if names.len() < 1 {
            break;
        }
    }
}
fn random_name(names: Vec<String>) {
    let randomnumber = rand::rng().random_range(..names.len());
    println!("{}", names[randomnumber])
}
