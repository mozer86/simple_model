/*
MIT License
Copyright (c) 2021 Germán Molina
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use crate::Float;
use crate::scanner::make_error_msg;

#[derive(Clone,Copy)]
pub struct Token<'a> {
    pub line: usize,    
    pub length: usize,
    pub start: usize,          
    pub token_type: TokenType,
    pub txt: &'a [u8],
}


#[derive(Debug, Clone,Copy, Eq, PartialEq)]
pub enum TokenType{
    // single char
    Colon,
    Comma,    
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Hash,    
    Underscore,
    Star,
    Bang,    
    CodeBoundary,
    LeftParen,
    RightParen,

    // double char
    ColonColon,
    StarStar,
    
    // triple char
    HorizontalRule,
    SimpleBuildingBlockLimit,
    
    // keywords
    Use,
    
    // mixed words
    TokenEnumName,
    Identifier,
    TokenString,
    Number,
    Word,
    
    // special
    ControlBlockLimit,
        
    // other
    EOF,
    Error,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            // Single char
            TokenType::Colon => write!(f, ":"),
            TokenType::Comma=> write!(f, ","),
            TokenType::LeftBrace=> write!(f, "{{"),
            TokenType::RightBrace=> write!(f, "}}"),
            TokenType::LeftBracket=> write!(f, "["),
            TokenType::RightBracket=> write!(f, "]"),
            TokenType::Hash=> write!(f, "#"),
            TokenType::Underscore=> write!(f, "_"),
            TokenType::Star=> write!(f, "*"),
            TokenType::Bang=> write!(f, "!"),
            TokenType::CodeBoundary=> write!(f, "`"),
            TokenType::LeftParen=> write!(f, "("),
            TokenType::RightParen=> write!(f, ")"),

            // double char
            TokenType::ColonColon=> write!(f, "::"),
            TokenType::StarStar=> write!(f, "**"),
            
            // triple char
            TokenType::HorizontalRule=> write!(f, "==="),
            TokenType::SimpleBuildingBlockLimit=> write!(f, "```"),
            
            // keywords
            TokenType::Use=> write!(f, "use"),
            
            // mixed words
            TokenType::TokenEnumName=> write!(f, "EnumName"),
            TokenType::Identifier=> write!(f, "Identifier"),
            TokenType::TokenString=> write!(f, "String"),
            TokenType::Number=> write!(f, "Number"),
            TokenType::Word=> write!(f, "Word"),
            
            // special
            TokenType::ControlBlockLimit=> write!(f, "ControlBlockLimit"),
                
            // other
            TokenType::EOF=> write!(f, "EOF"),
            TokenType::Error=> write!(f, "Error"),
        }
    }
}

impl <'a>Token<'a>{
    pub fn resolve_as_float(&self) -> Result<Float,String> {
        let txt = std::str::from_utf8(self.txt).unwrap();
        if let TokenType::Number = self.token_type{
             match txt.parse::<Float>(){
                 Ok(fvalue)=>Ok(fvalue),
                 Err(e)=>Err(format!("This is a bug, please report it: {}", e))
             }             
        }else{
            Err(format!("Token '{}' cannot be transformed into a float", txt))
        }
    }

    pub fn resolve_as_usize(&self) -> Result<usize,String> {
        let txt = std::str::from_utf8(self.txt).unwrap();
        if let TokenType::Number = self.token_type{
             match txt.parse::<usize>(){
                 Ok(fvalue)=>Ok(fvalue),
                 Err(_)=>Err(make_error_msg(format!("value '{}' does not seem to be a positive integer. Hint: remove dots, e.g., don't write '12.', write '12'", txt), self.line))
             }             
        }else{
            Err(make_error_msg(format!("Token '{}' cannot be transformed into a positive integer", txt), self.line))
        }
    }

    pub fn resolve_as_string(&self) -> Result<String,String> {
        let txt = std::str::from_utf8(self.txt).unwrap();
        if let TokenType::TokenString = self.token_type{
             Ok(txt[1..txt.len()-1].to_string())         
        }else{
            Err(make_error_msg(format!("Token '{}' cannot be transformed into a String", txt), self.line))
        }
    }
}