use std::io::{self, Write};
use std::str::Chars;

enum TokenType {
    INTEGER,
    ADD,
    SUBTRACT,
    MULT,
    DIV,
    EOF,
}

struct Token {
    value: String,
    t_type: TokenType,
}

struct Lexer<'a> {
    text: Chars<'a>,
    pos: usize,
    current_token: Option<Token>,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {

    fn new(text: &'a mut String) -> Lexer<'a> {
        Lexer {
            text: text.chars(),
            pos: 0,
            current_token: None,
            current_char: None,
        }
    }
    
    fn integer(&mut self) -> i32 {
        let mut digit = String::new();
        loop {
            let cur_digit = self.text.nth(self.pos);
            self.pos += 1;
            match cur_digit {
                Some(x) => {
                    if x.is_digit(10) {
                        digit.push(x)
                    }
                    else {
                        // return parsed digit
                        break;
                    }
                },
                None => {
                    // return parsed digit
                },
            }
        }
        return 0;
    }

    fn advance(&mut self) {
        let pos = self.pos;
        let (len, _) = self.text.size_hint();
        self.pos += 1;
        if pos > len - 1 {
            self.current_char = None;
        }
        else {
            self.current_char = Some(self.text.nth(self.pos).unwrap());
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.current_char {
                Some(x) => {
                    if x == ' ' {
                        self.advance();
                    }
                },
                None => {
                    break;
                },
            }
        }
    }
}

fn main() {
    let mut input = String::new();

    print!("calc> ");
    
    io::stdout().flush().unwrap();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {

        }
        Err(error) => println!("Error: {}", error)
    }
}
