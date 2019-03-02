use std::fmt;
use std::fmt::Display;
use std::iter::{Peekable, Enumerate};

use crate::token::{Token, TokenType};

pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner { source: source, tokens: Vec::new() }
    }

    pub fn scan(&mut self) {
        let mut line = 0;
        let mut iter = self.source.chars().enumerate().peekable();

        while let Some((_i, c)) = iter.next() {
            if let Some(token) = scan_token(c, line, &mut iter) {
                self.tokens.push(token);
            }
        }

        self.tokens.push(Token::new("".to_string(), TokenType::Eof))
    }
}

fn scan_token(c: char, mut line: u32, iter: &mut Peekable<Enumerate<std::str::Chars>>) -> Option<Token> {
    let (token, token_type) = match c {
        '(' => ("(".to_string(), TokenType::LeftParen),
        ')' => (")".to_string(), TokenType::RightParen),
        '{' => ("{".to_string(), TokenType::LeftBrace),
        '}' => ("}".to_string(), TokenType::RightBrace),
        ',' => (",".to_string(), TokenType::Comma),
        '.' => (".".to_string(), TokenType::Dot),
        '-' => ("-".to_string(), TokenType::Minus),
        '+' => ("+".to_string(), TokenType::Plus),
        ';' => (";".to_string(), TokenType::Semicolon),
        '*' => ("*".to_string(), TokenType::Star),
        '!' => {
            if let Some((_, '=')) = iter.peek() {
                ("!=".to_string(), TokenType::BangEqual)
            } else {
                ("!".to_string(), TokenType::Bang)
            }
        },
        '=' => {
            if let Some((_, '=')) = iter.peek() {
                ("==".to_string(), TokenType::EqualEqual)
            } else {
                ("=".to_string(), TokenType::Equal)
            }
        },
        '<' => {
            if let Some((_, '=')) = iter.peek() {
                ("<=".to_string(), TokenType::LessEqual)
            } else {
                ("<".to_string(), TokenType::Less)
            }
        },
        '>' => {
            if let Some((_, '=')) = iter.peek() {
                (">=".to_string(), TokenType::GreaterEqual)
            } else {
                ("<".to_string(), TokenType::Greater)
            }
        },
        '/' => {
            if let Some((_, '/')) = iter.peek() {
                // it's a comment advance to end of line
                while let Some((_, c)) = iter.next() {
                    if c == '\n' {
                        break;
                    }
                }
                return None
            } else {
                ("/".to_string(), TokenType::Slash)
            }
        },
        ' ' | '\t' | '\r'  => {
            return None
        },
        '\n' => {
            line += 1;
            return None
        },
        '"' => {
            (scan_string(iter), TokenType::String)
        },
        _ => {
            return None
        }
    };
    Some(Token::new(token.to_string(), token_type))
}
    
fn scan_string(iter: &mut Peekable<Enumerate<std::str::Chars>>) -> String {
    let mut string = String::new();
    while let Some((_, c)) = iter.next() {
        if c == '"' {
          break;   
        }
        string.push(c);
    }
    string
}