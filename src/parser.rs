use std::iter::Peekable;
use std::vec::IntoIter;

use crate::lexer::Token;

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug)]
pub enum Expression {
    Constant(i32),
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub body: Statement,
}

#[derive(Debug)]
pub struct Program {
    pub function: Function
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
        Some(Token::IntLiteral(int_value)) => int_value.parse::<i32>().expect("Failed to parse integer"),
        
        Some(wrong_token) => {
            panic!("Syntax Error: Expected to see IntLiteral but got {:?}", wrong_token);
        },
        None => {
            panic!("Syntax Error: Unexpected EoF. Expecting IntLiteral");
        },
    }
}

pub fn expect_identifier(iter: &mut Peekable<IntoIter<Token>>) -> String {
    match iter.next() {
        Some(Token::Identifier(name)) => name,
        
        Some(wrong_token) => {
            panic!("Syntax Error: Expected to see Identifier String but got {:?}", wrong_token);
        },
        None => {
            panic!("Syntax Error: Unexpected EoF. Expecting IntLiteral");
        },
    }
}

pub fn program_parser(token_vec: Vec<Token>) -> Program{
    let mut iter = token_vec.into_iter().peekable();

    Program {
        function: function_parser(&mut iter),
    }
    
}

pub fn function_parser(iter: &mut Peekable<IntoIter<Token>>) -> Function{
    expect_token(iter, Token::KeywordInt);

    // expect_token(iter, Token::Identifier("main".to_string()));
    let name = expect_identifier(iter);

    expect_token(iter, Token::OpenParen);
    expect_token(iter, Token::CloseParen);

    expect_token(iter, Token::OpenBrace);

    let body_statement = statement_parser(iter);

    expect_token(iter, Token::CloseBrace);

    Function { 
        name: name, 
        body: body_statement 
    }
}

pub fn statement_parser(iter: &mut Peekable<IntoIter<Token>>) -> Statement {
    expect_token(iter, Token::KeywordReturn);

    let exp = expression_parser(iter);

    expect_token(iter, Token::Semicolon);

    Statement::Return(exp)
}

pub fn expression_parser(iter: &mut Peekable<IntoIter<Token>>) -> Expression {
    let const_int = expect_int(iter);
    Expression::Constant(const_int)
}
