use std::fs;
use std::env;
mod lexer;

fn main() {
    let _file = env::args().nth(1);

    let file = if let Some(f) = _file {
        f
    } else {
        panic!("ERR >> Expected a file to be provided in arguments.");
    };

    if !file.ends_with(".rbl") {
        println!("Unable to open a non-Ruble file");
    }

    let _source = fs::read_to_string(file);

    let source = if _source.is_ok() {
        _source.unwrap()
    } else {
        panic!("ERR >> Could not open file, this probably means the path is incorrect. Try again with absolute path.");
    };

    let mut lexer = lexer::Lexer::new(source, String::from("MAIN"));
    lexer.lex();
}