use regex::Regex;

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
        let mut rules = Vec::new();
        rules.push(Rule { name: "Test".to_string(), pattern: Regex::new("[a-z]").unwrap() });
        Parser { rules: rules }
    }
}


// for token in tokenizer {
    //     match token {
    //         Token::Integer(_) => current_expression.push(token),
    //         Token::Plus(_)    => current_expression.push(token),
    //         Token::Minus(_)   => current_expression.push(token),
    //         Token::Div(_)     => current_expression.push(token),
    //         Token::Mod(_)     => current_expression.push(token),
    //         Token::Equal(_)   => current_expression.push(token),
    //         Token::Ln(_)      => { execute_line(&current_expression); current_expression.clear() },
    //         _                 => println!("skip"),
    //     }
    //
    //     // println!("{:?}", token);
    // }
