use std::env;
use scanlex::{Scanner, Token};

fn main() {

    match env::args().nth(1) {
        Some(arg) => run_file(arg),
        None => println!("Pass a file to interpret")
    };
        
    println!("Done.");
}


fn run_file(filename: String) {
    run(&filename);
    println!("{}", filename);
}

fn run(source: &String) {
    let scanner = Scanner::new(source);
    for token in scanner {
        println!("{:?}", token);
    }
}