use rlox::parser;
use rlox::parser::Parser;
use rlox::scanner::Scanner;
use std::env;
use std::fs;

fn main() {
    match env::args().nth(1) {
        Some(arg) => run_file(arg),
        None => start_interactive_mode()
    };

    println!("Done.");
}

fn start_interactive_mode() {

}

fn run_file(filename: String) {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    run(&contents);
}

fn run(source: &String) {
    let mut scanner = Scanner::new(source.to_owned());
    scanner.scan_tokens();
    let mut parser = Parser::new(scanner.tokens);
    let expr = parser.parse();
    let ast_dump = parser::ast_dump(&expr);

    println!("{}", ast_dump);
}
