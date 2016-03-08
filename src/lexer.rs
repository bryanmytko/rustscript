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
    pos: usize,
}

impl<I: Iterator<Item=char>> Tokenizer<I> {
    pub fn new(iter: I) -> Tokenizer<I> {
        Tokenizer {
            iter: iter.peekable(),
            buf: k
            pos: 0,
        }
    }

    fn parse_word(&mut self) -> String {
        // Need to parse until non alpha
        "foo".to_string()
    }
}

impl<I: Iterator<Item=char>> Iterator for Tokenizer<I> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {

        if self.iter.size_hint().1.unwrap() >= self.pos {
            self.pos += 1;
        } else {
            return None;
        }

        println!("{:?}", self.pos);

        /* Need to match over something other than "next". Buffer position? */
        match self.iter.next().unwrap() {
            '\n' => Some(Token::Ln("\n".to_string())),
            'a'...'z' => Some(Token::Atom(self.parse_word())),
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
