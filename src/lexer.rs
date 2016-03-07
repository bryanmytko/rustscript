use std::iter::Peekable;
use regex::Regex;

/* @TODO This is just random junk. Will need a real grammer eventually */
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Token {
    Atom(String),
    ParenL(String),
    ParenR(String),
    Separator(String),
    Implies(String),
    Integer(String),
    Variable(String),
    Equal(String),
    Whitespace(String),
    Plus(String),
    Minus(String),
    Div(String),
    Mult(String),
    Mod(String),
    Ln(String),
}

#[derive(Clone)]
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
        // let mut token = Some(Token::Atom(self.iter.by_ref().take(1).collect()));

        match *self.iter.peek().unwrap() {
            'a'...'z' => {
                let mut optional = Some(Token::Atom(self.iter.by_ref().take(1).collect()));
                // while let Some(i) = optional {
                //     if i == Token::Atom("foo".to_string()){
                //         optional = Some(Token::Atom("".to_string()));
                //     } else {
                //         break;
                //     }
                // }
                self.iter.peek();
                optional
            },
            _   => None,
        }
    }
}
    //     match *self.iter.peek().unwrap_or(&'♥') {
    //         '(' => Some(Token::ParenL(self.iter.by_ref().take(1).collect())),
    //         ')' => Some(Token::ParenR(self.iter.by_ref().take(1).collect())),
    //         ',' => Some(Token::Separator(self.iter.by_ref().take(1).collect())),
    //         '=' => Some(Token::Equal(self.iter.by_ref().take_while(|&c| c == '=').collect())),
    //         '+' => Some(Token::Plus(self.iter.by_ref().take(1).collect())),
    //         '-' => Some(Token::Minus(self.iter.by_ref().take(1).collect())),
    //         '/' => Some(Token::Div(self.iter.by_ref().take(1).collect())),
    //         '*' => Some(Token::Mult(self.iter.by_ref().take(1).collect())),
    //         '%' => Some(Token::Mod(self.iter.by_ref().take(1).collect())),
    //         'a'...'z' => Some(Token::Atom(self.iter.by_ref().take_while(|&c| c.is_alphabetic()).collect())),
    //         'A'...'Z' => Some(Token::Atom(self.iter.by_ref().take_while(|&c| c.is_alphabetic()).collect())),
    //         '1'...'9' => Some(Token::Atom(self.iter.by_ref().take_while(|&c| c.is_alphabetic()).collect())),
    //         '\r' | '\n' => Some(Token::Ln(self.iter.by_ref().take(1).collect())),
    //         ' ' => Some(Token::Whitespace(self.iter.by_ref().take(1).collect())),
    //         '♥' => None,
    //         _   => None, // @TODO Do we want None on invalid syntax?
    //     }
    // }
