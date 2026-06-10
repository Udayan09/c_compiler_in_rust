use std::iter::Peekable;
use std::vec::IntoIter;

use crate::lexer::Token;

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug)]
pub enum UnaryOperation {
    Negation,
    Complement,
    LogicalNegation,
}

#[derive(Debug)]
pub enum BinaryOperation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Debug)]
pub enum Expression {
    Constant(i32),
    UnOp(UnaryOperation, Box<Expression>),
    BinOp(BinaryOperation, Box<Expression>, Box<Expression>),
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

pub fn parse_program(token_vec: Vec<Token>) -> Program{
    let mut iter = token_vec.into_iter().peekable();

    Program {
        function: parse_function(&mut iter),
    }
    
}

pub fn parse_function(iter: &mut Peekable<IntoIter<Token>>) -> Function{
    expect_token(iter, Token::KeywordInt);

    // expect_token(iter, Token::Identifier("main".to_string()));
    let name = expect_identifier(iter);

    expect_token(iter, Token::OpenParen);
    expect_token(iter, Token::CloseParen);

    expect_token(iter, Token::OpenBrace);

    let body_statement = parse_statement(iter);

    expect_token(iter, Token::CloseBrace);

    Function { 
        name: name, 
        body: body_statement 
    }
}

pub fn parse_statement(iter: &mut Peekable<IntoIter<Token>>) -> Statement {
    expect_token(iter, Token::KeywordReturn);

    let exp = parse_exp(iter);

    expect_token(iter, Token::Semicolon);

    Statement::Return(exp)
}

// pub fn parse_expression(iter: &mut Peekable<IntoIter<Token>>) -> Expression {
//     match iter.next() {
//         Some(Token::Negation) => {
//             let inner_exp = parse_expression(iter);
//             Expression::UnOp(UnaryOperation::Negation, Box::new(inner_exp))
//         },
//         Some(Token::BitwiseComplement) => {
//             let inner_exp = parse_expression(iter);
//             Expression::UnOp(UnaryOperation::Complement, Box::new(inner_exp))
//         },
//         Some(Token::LogicalNegation) => {
//             let inner_exp = parse_expression(iter);
//             Expression::UnOp(UnaryOperation::LogicalNegation, Box::new(inner_exp))
//         },
//         Some(Token::IntLiteral(int_str)) => {
//             let int_value = int_str.parse::<i32>().expect("Failed to parse integer");
//             Expression::Constant(int_value)
//         },
//         Some(wrong_token) => {
//             panic!("Syntax Error: Expected an expression, but found {:?}", wrong_token);
//         },
//         None => {
//             panic!("Syntax Error: Unexpected End of File while parsing expression.");
//         },
//     }
// }

pub fn parse_exp(iter: &mut Peekable<IntoIter<Token>>) -> Expression{
    let mut left_term = parse_term(iter);

    loop {
        match iter.peek() {
            Some(&Token::Addition) => {
                iter.next();
                let right_term = parse_term(iter);
                left_term = Expression::BinOp(BinaryOperation::Addition, Box::new(left_term), Box::new(right_term));
            },
            
            Some(&Token::Negation) => {
                iter.next();
                let right_term = parse_term(iter);
                left_term = Expression::BinOp(BinaryOperation::Subtraction, Box::new(left_term), Box::new(right_term));
            },
            
            _ => break,
            
            None => {
                panic!("Syntax Error: Unexpected End of File while parsing expression.");
            },
        }
    }
    left_term
}

pub fn parse_term(iter: &mut Peekable<IntoIter<Token>>) -> Expression{
    let mut left_factor = parse_factor(iter);
    
    loop {
        match iter.peek() {
            Some(&Token::Multiplication) => {
                iter.next();
                let right_factor = parse_factor(iter);
                left_factor = Expression::BinOp(BinaryOperation::Multiplication, Box::new(left_factor), Box::new(right_factor));
            },

            Some(&Token::Division) => {
                iter.next();
                let right_factor = parse_factor(iter);
                left_factor = Expression::BinOp(BinaryOperation::Division, Box::new(left_factor), Box::new(right_factor));
            },

            _ => break,

            None => {
                panic!("Syntax Error: Unexpected End of File while parsing expression.");
            },
        }
    }
    left_factor
}

pub fn parse_factor(iter: &mut Peekable<IntoIter<Token>>) -> Expression{
    
    match iter.next() {
        Some(Token::IntLiteral(int_str)) => {
            let int_value = int_str.parse::<i32>().expect("Failed to parse integer");
            Expression::Constant(int_value)
        }

        Some(Token::OpenParen) => {
            let exp = parse_exp(iter);
            if iter.next() != Some(Token::CloseParen) {
                panic!("Parenthesis left unclosed");
            }
            exp
        }

        Some(Token::Negation) => {
            let inner_exp = parse_factor(iter);
            Expression::UnOp(UnaryOperation::Negation, Box::new(inner_exp))
        },

        Some(Token::BitwiseComplement) => {
            let inner_exp = parse_factor(iter);
            Expression::UnOp(UnaryOperation::Complement, Box::new(inner_exp))
        },

        Some(Token::LogicalNegation) => {
            let inner_exp = parse_factor(iter);
            Expression::UnOp(UnaryOperation::LogicalNegation, Box::new(inner_exp))
        },

        Some(wrong_token) => {
            panic!("Syntax Error: Expected an expression, but found {:?}", wrong_token);
        },

        None => {
            panic!("Syntax Error: Unexpected End of File while parsing expression.");
        },
    }
}