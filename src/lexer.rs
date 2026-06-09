
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