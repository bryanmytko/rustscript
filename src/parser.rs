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

//   impl<I: Iterator<Item=char>> Tokenizer<I> {
//   pub struct Tokenizer<I: Iterator<Item=char>> {
    pub fn parse_tokens(&self, tokens: &Iterator<Item=Token>) -> String {
        // error: the trait `core::marker::Sized` is not implemented for 
        // for token in *tokens {
        //     println!("test");
        // }
        "Test".to_string()
    }
}
