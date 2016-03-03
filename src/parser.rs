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
        let var = "([:lower:][:alnum:]*)";
        let int = "([:digit]+)";
        let exp = "(.*)";

        let patterns = vec![
            ("Var", vec![var]),
            ("Integer", vec![int]),
            ("AssignVar", vec![var, "=", exp]),
            ("Add", vec![exp, "+", exp]),
            ("Subtract", vec![exp, "-", exp]),
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
