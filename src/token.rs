use std::fmt;
use std::fmt::Display;

#[derive(Copy, Clone)]
pub enum TokenType {                                   
  // Single-character tokens.                      
  LeftParen, RightParen, LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

  // One or two character tokens.                  
  Bang, BangEqual,                                
  Equal, EqualEqual,                              
  Greater, GreaterEqual,                          
  Less, LessEqual,                                

  // Literals.                                     
  Identifier, String, Number,                      

  // Keywords.                                     
  And, Class, Else, False, Fun, For, If, Nil, Or,  
  Print, Return, Super, This, True, Var, While,    

  Eof                                              
} 

pub struct Token {
    lexeme: String,
    token_type: TokenType,
}

impl Token {
    pub fn new(lexeme: String, token_type: TokenType) -> Token {
        Token { lexeme: lexeme, token_type: token_type }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.lexeme)
    }
}
