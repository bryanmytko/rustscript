# Rustscript (name pending)

Writing a scripting language in Rust for the purpose of getting better at Rust.


This is not meant to be useful outside of personal education.
There is currently no real grammar. That will be fleshed out as some real rules for the parser become apparent.

### Lexer

`$ cargo run test.rsc`

Running `target/debug/rustscript test.rsc`

    Integer("1")
    Plus("+")
    Whitespace(" ")
    Integer("1")
    Equal("=")
    Whitespace(" ")
    Integer("2")
    Ln("\n")
    Atom("Bryan")
    Atom("Mytko")
    Ln("\n")
    Atom("def")
    Atom("foobar")
    Whitespace(" ")
    Whitespace(" ")
    Atom("puts")
    Atom("test")
    Atom("end")

### Parser

*Currently in development*

    [
      Rule { name: "Var", pattern: ^([:lower:][:alnum:]*)$ },
      Rule { name: "Integer", pattern: ^([:digit]+)$ },
      Rule { name: "AssignVar", pattern: ^([:lower:][:alnum:]*)=(.*)$ },
      Rule { name: "Add", pattern: ^(.*)+(.*)$ },
      Rule { name: "Subtract", pattern: ^(.*)-(.*)$ }
    ]
