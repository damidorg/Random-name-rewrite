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
        buffer.read_line(&mut input).expect("invalid input");

        match input.trim() {
            "done" => break,
            "" => println!("enter a name!"),
            _ => names.push(input.to_string()),
        }
    }
    println!("what mode?\n(1)last standing\n(2)random name ");
    print!("> ");
    io::stdout().flush().expect("PROGRAM BROC");
    let buffer = io::stdin();
    let mut input = String::new();
    buffer.read_line(&mut input).expect("bad input");

    match input.trim() {
        "1" => last_standing(names),
        "2" => random_name(names),
        _ => names.push(input.to_string()),
    }
}
fn last_standing(mut names: Vec<String>) {
    let mut place: u32 = 1;
    loop {
        let randomnumber = rand::rng().random_range(..names.len());
        println!("{}.{}", place, names[randomnumber]);
        names.remove(randomnumber);
        place += 1;
        if names.len() < 1 {
            break;
        }
    }
}
fn random_name(names: Vec<String>) {
    let randomnumber = rand::rng().random_range(..names.len());
    println!("{}", names[randomnumber])
}
