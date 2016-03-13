use regex::Regex;
use lexer::*;

pub struct Parser {
    pub rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct Rule {
    name: String,
    pattern: Regex,
}

impl Parser {
    pub fn new() -> Parser {
        let var = "([:lower:][:alnum:]*)";
        let int = "([:digit]+)";
        let exp = "(.*)";
        let keyword = "(def|end|if|else|elsif|while|do)";

        let patterns = vec![
            ("Var", vec![var]),
            ("Integer", vec![int]),
            ("AssignVar", vec![var, "=", exp]),
            ("Add", vec![exp, "+", exp]),
            ("Subtract", vec![exp, "-", exp]),
            ("Comparison", vec![exp, "==", exp]),
            ("!Comparison", vec![exp, "!=", exp]),
            ("Keyword", vec![keyword]),
        ];

        let mut rules = Vec::new();

        for pattern in patterns.iter() {
            let name = pattern.0;
            let ref pattern_group = pattern.1;

            let mut str = String::from("^");
            for p in pattern_group.iter() {
                str.push_str(*p);
            }
            str.push_str("$");

            rules.push(
                Rule {
                    name: String::from(name),
                    pattern: Regex::new(&str).unwrap(),
                }
            )
        }

        Parser { rules: rules }
    }

    pub fn parse_tokens(&self, tokens: &Vec<Token>) -> String {
        for token in tokens {
            match *token {
                // @TODO replace println! macro with actual parse pattern comparison
                Token::Atom(ref s) => println!("Atom: {}", s),
                Token::Integer(ref i) => println!("Integer: {}", i),
                Token::Variable(ref s) => println!("Variable: {}", s),
                Token::Separator(ref s) => println!("Separator: {}", s),
                Token::Implies(ref s) => println!("Implies: {}", s),
                Token::EqComp(ref s) => println!("EqComp: {}", s),
                Token::Assignment => println!("="),
                Token::ParenL => println!("("),
                Token::ParenR => println!(")"),
                Token::Plus => println!("+"),
                Token::Minus => println!("-"),
                Token::Div => println!("/"),
                Token::Mult => println!("*"),
                Token::Mod => println!("%"),
                Token::Ln => println!("newline"),
                Token::Whitespace => println!(" "),
                // @TODO Currently exhaustive, I think we can rely on tokens
                // from the lexer to be valid. Syntax errors should be handled there.
                // _ => println!("NOT IMPLEMENTED YET."),

            }
        }

        "test return value".to_owned()
    }
}
