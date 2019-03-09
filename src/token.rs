use std::fmt;
use std::fmt::Display;

#[derive(Copy, Clone, PartialEq)]
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

    pub fn minus() -> Token {
        Token { lexeme: String::from("-"), token_type: TokenType::Minus }
    }

    pub fn star() -> Token {
        Token { lexeme: String::from("*"), token_type: TokenType::Star }
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.lexeme)
    }
}
