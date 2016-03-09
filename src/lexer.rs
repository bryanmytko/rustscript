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
    buf: String,
    pos: usize,
}

impl<I: Iterator<Item=char>> Tokenizer<I> {
    pub fn new(iter: I, buf: String) -> Tokenizer<I> {
        Tokenizer {
            iter: iter.peekable(),
            buf: buf,
            pos: 0,
        }
    }

    fn parse_word(&mut self) -> Option<Token> {
        // Need to parse until non alpha
        // self.char_at(self.pos).to_string()
        // let (start, end) =
        //             match (regex!(r"^'([^']|\\')*'")).find(self.buf[self.pos..]) {
        //                             Some((s, e)) => (s + self.pos, e + self.pos),
        //                                         None => return None
        //                                                     };
        //     self.pos = end;
        //         Some(Token::StrLit(self.buf[start + 1..end - 1]).to_string()))
        let(start, end) =
            match (regex!(r"^'([^']|\\')*'")).find(&self.buf[self.pos..]) {
                Some((s, e)) => (s + self.pos, e + self.pos),
                None => { return None },
            };

        self.pos = end;
        Some(Token::Atom(self.buf[start + 1..end - 1].to_string()))
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

        println!("Position: {:?}", self.pos);

        /* Need to match over something other than "next". Buffer position? */
        // match self.iter.next().unwrap() {
        match self.buf.char_at(self.pos) {
            '\n' => Some(Token::Ln("\n".to_string())),
            'a'...'z' => self.parse_word(),
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
