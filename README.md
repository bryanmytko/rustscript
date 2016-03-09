# Rustscript (name pending)

Writing a scripting language in Rust for the purpose of getting better at Rust.


This is not meant to be useful outside of personal education.
There is currently no real grammar. That will be fleshed out as some real rules for the parser become apparent.

### Lexer

`$ cargo run test.rsc`

Running `target/debug/rustscript test.rsc`

    Atom("abc")
    Whitespace
    Eq("=")
    Whitespace
    Integer("4")
    Ln
    Atom("b")
    Whitespace
    Eq("=")
    Whitespace
    Atom("a")
    Whitespace
    Plus
    Whitespace
    Integer("1")
    Ln
    Ln
    Atom("farm")
    Whitespace
    Eq("=")
    Whitespace
    Integer("4")
    Ln
    Ln
    Atom("if")
    Whitespace
    Atom("a")
    Whitespace
    Eq("==")
    Whitespace
    Atom("b")
    Ln
    Whitespace
    Whitespace
    Integer("1")
    Ln
    Atom("else")
    Ln
    Whitespace
    Whitespace
    Integer("2")
    Ln
    Atom("end")
    Ln

### Parser

*Currently in development*

    [
      Rule { name: "Var", pattern: ^([:lower:][:alnum:]*)$ },
      Rule { name: "Integer", pattern: ^([:digit]+)$ },
      Rule { name: "AssignVar", pattern: ^([:lower:][:alnum:]*)=(.*)$ },
      Rule { name: "Add", pattern: ^(.*)+(.*)$ },
      Rule { name: "Subtract", pattern: ^(.*)-(.*)$ },
      Rule { name: "Comparison", pattern: ^(.*)==(.*)$ },
      Rule { name: "!Comparison", pattern: ^(.*)!=(.*)$ },
      Rule { name: "Keyword", pattern: ^(def|end|if|else|elsif|while|do)$ }
    ]

### Evaluator

    ¯\_(ツ)_/¯
