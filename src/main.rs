use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

mod lexer;
mod parser;

use lexer::Tokenizer;
use parser::*;

fn main(){
    let file_name = env::args().nth(1).unwrap();
    let file = load_file(file_name);

    let tokenizer = Tokenizer::new(file.chars());


    for token in tokenizer{
        println!("{:?}", token);
    }

    // let mut current_expression = Vec::new();
}

fn load_file<P: AsRef<Path>>(path: P) -> String {
    let mut file = File::open(path).unwrap();
    let mut file_buffer = String::new();
    file.read_to_string(&mut file_buffer).unwrap();
    file_buffer
}
