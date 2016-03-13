use std::iter::Peekable;
use regex::Regex;

/* @TODO This is just random junk. Will need a real grammer eventually */
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Token {
    Atom(String),
    Integer(String),
    Variable(String),
    Separator(String),
    Implies(String),
    EqComp(String),
    Assignment,
    ParenL,
    ParenR,
    Plus,
    Minus,
    Div,
    Mult,
    Mod,
    Ln,
    Whitespace,
}

#[derive(Clone)]
pub struct Tokenizer<I: Iterator<Item=char>> {
    iter: Peekable<I>,
    buf: String,
    pos: usize,
}

impl<I: Iterator<Item=char>> Tokenizer<I> {
    pub fn new(iter: I, buf: String) -> Vec<Token> {
        let tokenizer = Tokenizer {
            iter: iter.peekable(),
            buf: buf,
            pos: 0,
        };

        let mut tokens = vec![];

        for token in tokenizer {
            tokens.push(token);
        }

        tokens
    }

    fn match_pattern(&mut self, pattern: Regex) -> &str {
        let(start, end) =
            match pattern.find(&self.buf[self.pos..]) {
                Some((s, e)) => (s + self.pos, e + self.pos),
                None => { return "" },
            };

        self.pos = end;
        &self.buf[start..end]
    }

    fn parse_word(&mut self) -> Option<Token> {
        let result = self.match_pattern(Regex::new(r"[:alpha:]*").unwrap());
        Some(Token::Atom(result.to_owned()))
    }

    fn parse_digit(&mut self) -> Option<Token> {
        let result = self.match_pattern(Regex::new(r"\d*").unwrap());
        Some(Token::Integer(result.to_owned()))
    }

    fn parse_eq(&mut self) -> Option<Token> {
        let result = self.match_pattern(Regex::new(r"=*").unwrap());
        match result {
            "="  => Some(Token::Assignment),
            "==" => Some(Token::EqComp("==".to_owned())),
            _    => None,
        }
    }
}

impl<I: Iterator<Item=char>> Iterator for Tokenizer<I> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.pos >= self.iter.size_hint().1.unwrap() {
            return None;
        }

        match self.buf.char_at(self.pos) {
            ' '  => { self.pos += 1; Some(Token::Whitespace) },
            '\n' => { self.pos += 1; Some(Token::Ln) },
            '+'  => { self.pos += 1; Some(Token::Plus) },
            '-'  => { self.pos += 1; Some(Token::Minus) },
            '/'  => { self.pos += 1; Some(Token::Div) },
            '*'  => { self.pos += 1; Some(Token::Mult) },
            '%'  => { self.pos += 1; Some(Token::Mod) },
            '='  => self.parse_eq(),
            'A'...'z' => self.parse_word(),
            '1'...'9' => self.parse_digit(),
            _   => None,
        }
    }
}
