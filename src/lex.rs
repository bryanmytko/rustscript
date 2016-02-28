extern crate regex;

use std::io::prelude::*;
use std::fs::File;
use std::env;
use regex::Regex;
use std::iter::Peekable;

//let mut file = try!(File::open(""))

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Token {
    DelimL(String),
    DelimR(String),
    Separator(String),
    Implies(String),
    Variable(String),
    Atom(String),
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
        match *self.iter.peek().unwrap_or(&'.') {
            '('       => {Some(Token::DelimL("(".to_string())); self.next() },
            // ')'       => {Some(Token::DelimR(self.iter.by_ref().collect()))},
            // ','       => {Some(Token::Separator(self.iter.by_ref().collect()))},
            // '='       => {Some(Token::Implies(self.iter.by_ref().collect()))},
            '1'...'9' => Some(Token::Atom(self.iter.by_ref().collect())),
            'A'...'Z' => Some(Token::Atom(self.iter.by_ref().take_while(|&c| c.is_uppercase()).collect())),
            'a'...'z' => Some(Token::Atom(self.iter.by_ref().take_while(|&c| c.is_lowercase()).collect())),
            '.' => None,
            _ => self.next(),
        }
    }
}

fn main(){
    let args: Vec<_> = env::args().collect();
    // let re = Regex::new(r"\.rsc$");
    // assert!(re.unwrap().is_match(&args[0]));

    let input = "aa (aabbB)".chars();
    let tokenizer = Tokenizer::new(input);

    for token in tokenizer {
        println!("{:?}", token);
    }
}

