use rand::Rng;
use std::io;
use std::io::Write;

fn main() {
    eprintln!("write youre names (write `quit` to exit )");

    let mut names: Vec<String> = Vec::new();
    loop {
        print!("> ");
        io::stdout().flush().expect("PROGRAM BROC"); //asking for names
        let buffer = io::stdin();
        let mut input = String::new();
        buffer.read_line(&mut input).expect("invalid input");

        match input.trim() {
            //cheking the input
            "done" => break,
            "" => println!("enter a name!"),
            "quit" => return,
            _ => names.push(input.to_string()),
        }
    }
    loop {
        match mode_input().trim() {
            "1" => break last_standing(names),
            "2" => break random_name(names),
            "quit" => return,
            _ => {
                println!("pick from the options!!!");
                mode_input();
            }
        }
    }
}
fn mode_input() -> String {
    println!("what mode?\n(1)last standing\n(2)random name ");
    print!("> ");
    io::stdout().flush().expect("PROGRAM BROC");
    let buffer = io::stdin();
    let mut input = String::new();
    buffer.read_line(&mut input).expect("bad input");
    input
}

fn last_standing(mut names: Vec<String>) {
    //writing names and their places
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
