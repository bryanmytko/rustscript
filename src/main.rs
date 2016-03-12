#![feature(str_char)]
#![feature(plugin)]

#![plugin(regex_macros)]
extern crate regex;

use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

mod lexer;
mod parser;
mod eval;

use lexer::*;
use parser::*;
use eval::*;

fn main(){
    let file_name = env::args().nth(1).unwrap();
    let file = load_file(file_name);
    let file_string = String::from(file.chars().as_str());

    let tokens = Tokenizer::new(file.chars(), file_string);
    let parser = Parser::new();

    parser.parse_tokens(&tokens);

    // for token in tokens {
    //     println!("{:?}", token);
    // }
    //
    // println!("{:?}", parser.rules);
}

fn load_file<P: AsRef<Path>>(path: P) -> String {
    let mut file = File::open(path).unwrap();
    let mut file_buffer = String::new();
    file.read_to_string(&mut file_buffer).unwrap();
    println!("{:?}", file_buffer);
    file_buffer
}
