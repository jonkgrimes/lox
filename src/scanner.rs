use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::iter::{Enumerate, Peekable};

use crate::token::{Token, TokenType};

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut keywords = HashMap::new();
        keywords.insert("and", TokenType::And);
        keywords.insert("class", TokenType::Class);
        keywords.insert("else", TokenType::Else);
        keywords.insert("false", TokenType::False);
        keywords.insert("for", TokenType::For);
        keywords.insert("fun", TokenType::Fun);
        keywords.insert("if", TokenType::If);
        keywords.insert("nil", TokenType::Nil);
        keywords.insert("or", TokenType::Or);
        keywords.insert("print", TokenType::Print);
        keywords.insert("return", TokenType::Return);
        keywords.insert("super", TokenType::Super);
        keywords.insert("this", TokenType::This);
        keywords.insert("true", TokenType::True);
        keywords.insert("var", TokenType::Var);
        keywords.insert("while", TokenType::While);
        keywords
    };
}

pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
        }
    }

    pub fn scan(&mut self) -> Result<&Vec<Token>, ParserError> {
        let mut line = 0;
        let mut iter = self.source.chars().enumerate().peekable();

        while let Some((_i, c)) = iter.next() {
            let scanned_token = scan_token(c, &mut line, &mut iter)?;
            if let Some(token) = scanned_token {
                self.tokens.push(token);
            }
        }
        self.tokens.push(Token::new("".to_string(), TokenType::Eof));
        Ok(&self.tokens)
    }
}

#[derive(Debug)]
pub struct ParserError {
    line: u32,
    character: char,
}

impl ParserError {
    fn new(line: u32, character: char) -> ParserError {
        ParserError { line, character }
    }

    pub fn line(&self) -> u32 {
        self.line
    }
}

impl Error for ParserError {
    fn description(&self) -> &str {
        "Encountered an unparseable character"
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Encountered an unparseable character on line {}",
            self.line
        )
    }
}

type ScannerIterator<'a> = Peekable<Enumerate<std::str::Chars<'a>>>;

fn scan_token(
    c: char,
    line: &mut u32,
    iter: &mut ScannerIterator,
) -> Result<Option<Token>, ParserError> {
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
                iter.next();
                ("!=".to_string(), TokenType::BangEqual)
            } else {
                ("!".to_string(), TokenType::Bang)
            }
        }
        '=' => {
            if let Some((_, '=')) = iter.peek() {
                iter.next();
                ("==".to_string(), TokenType::EqualEqual)
            } else {
                ("=".to_string(), TokenType::Equal)
            }
        }
        '<' => {
            if let Some((_, '=')) = iter.peek() {
                iter.next();
                ("<=".to_string(), TokenType::LessEqual)
            } else {
                ("<".to_string(), TokenType::Less)
            }
        }
        '>' => {
            if let Some((_, '=')) = iter.peek() {
                iter.next();
                (">=".to_string(), TokenType::GreaterEqual)
            } else {
                (">".to_string(), TokenType::Greater)
            }
        }
        '/' => {
            if let Some((_, '/')) = iter.peek() {
                // it's a comment advance to end of line
                while let Some((_, c)) = iter.next() {
                    if c == '\n' {
                        break;
                    }
                }
                return Ok(None);
            } else {
                ("/".to_string(), TokenType::Slash)
            }
        }
        ' ' | '\t' | '\r' => return Ok(None),
        '\n' => {
            *line += 1;
            return Ok(None);
        }
        '"' => (scan_string(iter), TokenType::String),
        c => {
            if c.is_numeric() {
                (scan_number(c, iter), TokenType::Number)
            } else if c.is_alphabetic() || c == '_' {
                scan_identifier(c, iter)
            } else {
                return Err(ParserError::new(*line, c));
            }
        }
    };
    Ok(Some(Token::new(token.to_string(), token_type)))
}

fn scan_string(iter: &mut ScannerIterator) -> String {
    let mut string = String::new();
    while let Some((_, c)) = iter.next() {
        if c == '"' {
            break;
        }
        string.push(c);
    }
    string
}

fn scan_number(starting_char: char, iter: &mut ScannerIterator) -> String {
    let mut string = String::new();
    string.push(starting_char);
    while let Some((_, c)) = iter.peek() {
        if *c != '.' && !c.is_numeric() {
            break;
        }
        let (_, number) = iter.next().unwrap();
        string.push(number);
    }
    string
}

fn scan_identifier(starting_char: char, iter: &mut ScannerIterator) -> (String, TokenType) {
    let mut string = String::new();
    string.push(starting_char);
    while let Some((_, c)) = iter.peek() {
        if !c.is_alphabetic() && !c.is_numeric() {
            break;
        }
        let (_, character) = iter.next().unwrap();
        string.push(character);
    }

    match KEYWORDS.get(string.as_str()) {
        Some(keyword) => (string, *keyword),
        None => (string, TokenType::Identifier),
    }
}
