extern crate regex;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::env;
use regex::Regex;
use std::iter::Peekable;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Token {
    DelimL(String),
    DelimR(String),
    Separator(String),
    Implies(String),
    Variable(String),
    Atom(String),
    Whitespace(String),
}

pub struct Tokenizer<I: Iterator<Item=char>> {
    iter: Peekable<I>,
}

impl<I: Iterator<Item=char>> Tokenizer<I> {
    pub fn new(iter: I) -> Tokenizer<I> {
        Tokenizer {
            iter: iter.peekable(),
        }
    }
}

impl<I: Iterator<Item=char>> Iterator for Tokenizer<I> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        match *self.iter.peek().unwrap_or(&'♥') {
            // @TODO this is wrong; overflows buffer. Use take(1)
            '('       => Some(Token::DelimL(self.iter.by_ref().collect())),
            // ')'    => {Some(Token::DelimR(self.iter.by_ref().collect()))},
            // ','    => {Some(Token::Separator(self.iter.by_ref().collect()))},
            // '='    => {Some(Token::Implies(self.iter.by_ref().collect()))},
            '1'...'9' => Some(Token::Atom(self.iter.by_ref().take_while(|&c| c.is_numeric()).collect())),
            'A'...'Z' => Some(Token::Atom(self.iter.by_ref().take_while(|&c| c.is_uppercase()).collect())),
            'a'...'z' => Some(Token::Atom(self.iter.by_ref().take_while(|&c| c.is_lowercase()).collect())),
            '+'       => Some(Token::Atom(self.iter.by_ref().take(1).collect())),
            ' '       => Some(Token::Whitespace(self.iter.by_ref().take(1).collect())),
            '♥'       => None, // @TODO remove: unwrap_or condition
            _         => None, // @TODO Currently ends when there is an undefined token. Research skipping.
                               // self.iter.skip(&self);
        }
    }
}

fn main(){
    let file_name = env::args().nth(1).unwrap();
    let file = load_file(file_name);
    let tokenizer = Tokenizer::new(file.chars());

    for token in tokenizer {
        println!("{:?}", token);
    }
}

fn load_file<P: AsRef<Path>>(path: P) -> String {
    // @TODO Check filetype? Is this really necessary?
    // let re = Regex::new(r"\.rsc$");
    // assert!(re.unwrap().is_match(&path);

    let mut file = File::open(path).unwrap();
    let mut file_buffer = String::new();
    file.read_to_string(&mut file_buffer).unwrap();
    file_buffer
}
