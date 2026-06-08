use std::env;
use std::fs;
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug, PartialEq)]
pub enum Token {
    KeywordInt,
    KeywordReturn,
    Identifier(String),
    IntLiteral(String),
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Semicolon,
}

#[derive(Debug)]
pub enum Statement {
    Return,
}

#[derive(Debug)]
pub enum Expression {
    Constant(i32),
}

#[derive(Debug)]
pub struct Function {
    name: String,
    body: Statement,
}

#[derive(Debug)]
pub struct Program {
    function: Function
}


fn main() {
    let args: Vec<String> = env::args().collect();
    
    let file_path= &args[1];

    println!("File Path: {file_path}");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    let tokens:Vec<Token> = lex(&contents);

    println!("Tokens (Pretty): {:#?}", tokens);

    let prog = program_parser(tokens);
    // dbg!(args);
}

pub fn lex(source_code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = source_code.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {      //Ignores whitespace
            chars.next();
            continue;
        }

        //Handling Symbols
        match c {
            '{' => {
                tokens.push(Token::OpenBrace);
                chars.next();
                continue;
            },
            '}' => {
                tokens.push(Token::CloseBrace);
                chars.next();
                continue;
            },
            '(' => {
                tokens.push(Token::OpenParen);
                chars.next();
                continue;
            },
            ')' => {
                tokens.push(Token::CloseParen);
                chars.next();
                continue;
            },
            ';' => {
                tokens.push(Token::Semicolon);
                chars.next();
                continue;
            },
            _ => {}
        }

        //Handling Numbers
        if c.is_ascii_digit() {
            let mut num_str = String::new();
            while let Some(&inner_c) = chars.peek() {
                if inner_c.is_ascii_digit() {
                    num_str.push(inner_c);
                    chars.next();
                }
                else {
                    break;
                }
            }
            tokens.push(Token::IntLiteral(num_str));
            continue;
        }

        
        //Handling Indentifiers
        if c.is_alphabetic() || c == '_' {
            let mut word_str = String::new();
            while let Some(&inner_c) = chars.peek() {
                if inner_c.is_alphanumeric() || inner_c == '_'{
                    word_str.push(inner_c);
                    chars.next();
                }
                else {
                    break;
                }
            }
            match &word_str[..] {
                "return" => {
                    tokens.push(Token::KeywordReturn);
                    continue;
                }
                "int" => {
                    tokens.push(Token::KeywordInt);
                    continue;
                }
                _ => {
                    tokens.push(Token::Identifier(word_str));
                    continue;
                }
            }
            
        }

        panic!("Lexer error: Unrecognized character '{}'", c);

    }

    tokens
    
}

pub fn expect_token(iter: &mut Peekable<IntoIter<Token>>, expected_token: Token) {
    match iter.next() {
        Some(actual_token) => {
            if actual_token != expected_token {
                panic!("Syntax Error: Expected to see {:?} but got {:?}", expected_token, actual_token);
            }
        }
        None => {
            panic!("Syntax Error: Unexpected EoF. Expecting {:?}", expected_token);
        }
    }
}

pub fn expect_int(iter: &mut Peekable<IntoIter<Token>>) -> i32 {
    match iter.next() {
        Some(Token::IntLiteral(int_value)) => int_value.parse().unwrap(),
        
        Some(wrong_token) => {
            panic!("Syntax Error: Expected to see IntLiteral but got {:?}", wrong_token);
        },
        None => {
            panic!("Syntax Error: Unexpected EoF. Expecting IntLiteral");
        },
    }
}

pub fn program_parser(tokenVec: Vec<Token>) -> Program{
    let mut iter = tokenVec.into_iter().peekable();

    Program {
        function: function_parser(&mut iter),
    }
    
}

pub fn function_parser(iter: &mut Peekable<IntoIter<Token>>) -> Function{
    expect_token(iter, Token::KeywordInt);

    expect_token(iter, Token::Identifier("main".to_string()));

    expect_token(iter, Token::OpenParen);
    expect_token(iter, Token::CloseParen);

    expect_token(iter, Token::OpenBrace);

    let body_statement = statement_parser(iter);
}

pub fn statement_parser(iter: &mut Peekable<IntoIter<Token>>) -> Statement {
    expect_token(iter, Token::KeywordReturn);

    let exp = 
}

pub fn expression_parser(iter: &mut Peekable<IntoIter<Token>>) -> Expression {
    expect_token(iter, Token::IntLiteral("2".to_string()));
}
