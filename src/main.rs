use std::io::{self, Write};
use std::str::Chars;
use std::cmp::PartialEq;

#[derive(Debug,PartialEq)]
enum TokenType {
    INTEGER,
    ADD,
    SUBTRACT,
    MULT,
    DIV,
    EOF,
}

#[derive(Debug)]
struct Token {
    value: Option<String>,
    t_type: TokenType,
}

struct Interpreter<'a> {
    lexer: Lexer<'a>,
    current_token: Option<Token>
}

struct Lexer<'a> {
    text: Chars<'a>,
    len: usize,
    pos: usize,
    current_token: Option<Token>,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {

    fn new(text: &'a mut String) -> Lexer<'a> {
        let mut lex = Lexer {
            text: text.chars(),
            len: text.len(),
            pos: 0,
            current_token: None,
            current_char: None,
        };
        let mut cur_char = lex.text.next();
        lex.current_char = cur_char;
        return lex;
    }
    
    fn integer(&mut self) -> String {
        let mut digit = String::new();
        loop {
            let cur_digit = self.current_char;
            match cur_digit {
                Some(x) => {
                    //println!("Checking {} for digit", x);
                    if x.is_digit(10) {
                        //println!("Found digit!");
                        digit.push(x)
                    }
                    else {
                        break;
                    }
                },
                None => {
                    break;
                },
            }
            self.advance();
        }
        //println!("Returning digit..");
        return digit;
    }

    fn advance(&mut self) {
        let pos = self.pos;
        let len = self.len;
        //println!("Len is {}, pos is {}", len, pos);
        self.pos += 1;
        if pos > (len - 2) {
            self.current_char = None;
        }
        else {
            match self.text.next() {
                Some(x) => {
                    self.current_char = Some(x);
                    //println!("Successfully unwrapped next_char: {}", x);
                },
                None => {
                    //println!("error unwrapping next_char");
                    self.current_char = None;
                },
            }
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            //println!("Checking whitespace...");
            match self.current_char {
                Some(x) => {
                    if x.is_whitespace() {
                        //println!("Skipping whitespace!");
                        self.advance();
                        continue;
                    }
                    else {
                        break;
                    }
                },
                None => {
                    break;
                },
            }
        }
        //println!("Exiting skip_whitespace");
    }

    fn get_next_token(&mut self) -> Token {
        loop {
            let mut current_char: char;

            match self.current_char {
                Some(ref tok) => {
                    current_char = *tok;
                },
                None => {
                    break;
                }
            }
            //println!("Current char (in get_next_token): {:#?}", current_char);
            
            if current_char.is_whitespace() {
                //println!("In get_next_token, found a whitespace");
                self.skip_whitespace();
                continue;
            }

            if current_char.is_numeric() {
                let tok = self.integer();
                //self.advance();
                return Token {
                    t_type: TokenType::INTEGER,
                    value: Some(tok),
                };
            }

            match current_char {
                '+' => {
                    //println!("Found +");
                    self.advance();
                    return Token {
                        t_type: TokenType::ADD,
                        value: Some(current_char.to_string()),
                    };
                },
                '-' => {
                    self.advance();
                    return Token {
                        t_type: TokenType::SUBTRACT,
                        value: Some(current_char.to_string()),
                    };
                },
                '*' => {
                    self.advance();
                    return Token {
                        t_type: TokenType::MULT,
                        value: Some(current_char.to_string()),
                    };
                },
                _ => {
                    panic!("Invalid character found!");
                },
            }
        }
        return Token {
            t_type: TokenType::EOF,
            value: None
        };
    }
}

impl<'a> Interpreter<'a> {

    fn new(mut lex: Lexer<'a>) -> Interpreter<'a> {
        let mut token = lex.get_next_token();
        Interpreter {
            lexer: lex,
            current_token: Some(token),
        }
    }

    fn eat(&mut self, token: TokenType) {
        let mut tok: Token;
        match self.current_token {
            Some(ref x) => {
                if x.t_type == token {
                    println!("Valid token");
                }
                else {
                    println!("Invalid token: Wanted {:#?} got {:#?}", token, x.t_type);
                }
            },
            None => {
                panic!("No current token");
            },
        }
        let mut next = self.lexer.get_next_token();
        self.current_token = Some(next);
    }
    fn eval(&mut self) {
        let mut result: i32;
        
        match self.current_token {
            Some(ref x) => {
                match x.value {
                    Some(ref y) => {
                        result = y.parse::<i32>().unwrap();
                    },
                    None => {},
                }
            },
            None => {
            }
        }
    }

}

fn main() {
    let mut input = String::new();
    loop {
        print!("calc> ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let mut lexer = Lexer::new(&mut input);
                let mut interpreter = Interpreter::new(lexer);    
                interpreter.eat(TokenType::INTEGER);
                interpreter.eat(TokenType::ADD);
                interpreter.eat(TokenType::INTEGER);
            },
            Err(error) => println!("Error: {}", error),
        }
    }
}
