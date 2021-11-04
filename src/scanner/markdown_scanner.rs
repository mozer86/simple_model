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
// use crate::{SimpleModel, SimulationStateHeader};
use crate::scanner::tokens::*;
use crate::scanner::simple_scanner::make_error_msg;


enum ChunkClass{
    Header(usize),
}

struct Chunk{
    line: usize,
    start: usize,
    end: usize,
    class: ChunkClass
}


pub struct MDScanner<'a> {
    
    pub line : usize,    

    source: &'a [u8],

    current_index: usize,

    start_index: usize,

    pub error_msg: String,
    
    finished: bool, 
        
}



impl <'a>MDScanner<'a> {
    
    /// Creates a new [`MDScanner`]
    pub fn new(source : &'a [u8], line: usize)->Self{
        Self {
            finished: source.is_empty(),
            source,
            line,                        
            current_index: 0,
            start_index: 0,
            error_msg : "".to_string(),
        }
    }

    pub fn borrow_slice(&self, start: usize, end: usize)->&[u8]{
        &self.source[start..end]
    }

    pub fn update_start_index(&mut self){
        self.start_index = self.current_index;
    }

    /// Builds a [`Token`] corresponding to the current position of 
    /// the scanner (e.g., `start_index` and `current_index`, `line`, 
    /// etc.)
    fn make_token(&self, token_type: TokenType)->Token<'a>{
        
        let txt = &self.source[self.start_index..self.current_index];        
        Token {
            token_type,            
            txt,
            line: self.line,
            length: self.current_index - self.start_index,
            start: self.start_index,
        }
    }

    /// Checks whether the `char` at the `current_index` in the `source`
    /// is equal to the `expected` char. If it is, it returns `true` and 
    /// advances one step. If not, it returns `false` and does not 
    /// advance. 
    fn match_char(&mut self, expected : char)->bool{
        if self.finished {
            return false;
        }
        
        let c = self.source[self.current_index] as char;
        if c != expected {
            return false
        }

        self.current_index += 1;
        
        true
                
    }

    /// Advances one `char` in the `source`, returning the consumed
    /// `char` inside of an `Option`. If finished, it will mark the 
    /// [`MDScanner`] as finished and return `None`
    fn advance(&mut self )->Option<char>{
        if let Some(v) = self.source.get(self.current_index){

            self.current_index += 1;
            if self.current_index == self.source.len(){
                self.finished = true;
            }
            Some(*v as char)
        }else{
            self.finished = true;
            None      
        }            
    }

    /// Gets the `char` at the `current_index`. Returns `\0` if 
    /// finished.
    pub fn peek(&self)->char{
        if self.finished {
            return '\0';
        }
        self.source[self.current_index] as char
    }

    /// Gets the `char` following the `current_index`. Returns `\0` if 
    /// finished.
    fn peek_next(&self)->char{
        if self.finished || self.current_index + 1 == self.source.len() {
            return '\0';
        }
        
        self.source[self.current_index+1] as char // .clone().add(1) as char;        
    }

   
    /* MD SCANNER AND SIMPLESCANNER ARE THE SAME UP TO HERE */

   
    /// Consumes a whole word
    fn word(&mut self)->Token<'a>{        
        
        // scan the whole thing, until we find something that
        // is not a number, letter or '_'       
        let mut c = self.peek();
        while !c.is_ascii_whitespace() && c != ']' && c != ')' && c != ':'{
            match self.advance(){
                Some(_) => {c = self.peek()},
                None => return self.make_token(TokenType::EOF)
            }
        }

        self.make_token(TokenType::Word)
        
    }

    fn skip_white_space(&mut self){

        // Prevent segfault
        if self.finished{
            return;
        }

        loop {                                    
            match self.peek(){
                ' '  => {self.advance().unwrap();},
                '\r' => {self.advance().unwrap();},
                '\t' => {self.advance().unwrap();},
                '\n' => {
                    self.line += 1;
                    self.advance().unwrap();
                },                
                _ => return 
            };
            
        }
    }// end markdown skip white space

    /// Scans a [`Token`] in `Markdown` read mode
    pub fn scan_token(&mut self)->Token<'a>{
        self.skip_white_space();        
        self.start_index = self.current_index;

        let c = match self.advance(){
            Some(v)=>v,                
            None=> return self.make_token( TokenType::EOF)
        };
        
        // Check if any special character
        match c {
            
            // Single character            
            '#' => return self.make_token( TokenType::Hash),
            ':' => return self.make_token( TokenType::Colon),

            '_' => return self.make_token( TokenType::Underscore),
            '[' => return self.make_token( TokenType::LeftBracket),
            ']' => return self.make_token( TokenType::RightBracket),
            '(' => return self.make_token( TokenType::LeftParen),
            ')' => return self.make_token( TokenType::RightParen),
            '*' => {
                if self.match_char('*'){
                    return self.make_token(TokenType::StarStar)
                }else{
                    return self.make_token(TokenType::Star)
                }
            },
            '-' => {
                if self.peek() == '-' && self.peek_next() == '-'{
                    self.advance().unwrap();
                    self.advance().unwrap();
                    return self.make_token(TokenType::HorizontalRule)
                }
            },
            '!'=>{
                return self.make_token(TokenType::Bang)
            },
            '`' =>{
                if self.peek() == '`' && self.peek_next() == '`'{
                    self.advance().unwrap();
                    self.advance().unwrap();
                    self.start_index = self.current_index;
                    if self.line_contains(b"control"){                          
                        return self.make_token(TokenType::ControlBlockLimit)
                    }else{                        
                        return self.make_token(TokenType::SimpleBuildingBlockLimit)
                    }
                }else{
                    return self.make_token(TokenType::CodeBoundary)
                }
            }
            '\0' =>{
                return self.make_token(TokenType::EOF)
            },

            // Error            
            _ => {
                // unreachable!()
                // self.error_msg = format!("Unexpected character '{}' at line {} ",c, self.line);
                // return self.make_token(TokenType::Error)
            }
        };
        
        // Otherwise, return a Word
        self.word()        
    }

    
    
    
    /// CONSUMES all the words of the line, checking
    /// if a `keyword` is in them.
    fn line_contains(&mut self, keyword: &[u8])->bool{
        while self.peek() != '\n' && self.peek() != '\0' {
            if self.peek() == ' '{
                self.advance().unwrap();
            }else{    
                // if match, consume the line and continue            
                let txt = self.word().txt;
                if txt == keyword {
                    while self.peek() != '\n' && self.peek() != '\0'{
                        self.advance();
                        self.start_index = self.current_index;                
                    }
                    return true
                }
            }        
            self.start_index = self.current_index;                
        }
        false
    }


    /// Scans a header
    fn header(&mut self)-> Chunk {
        let mut token = self.scan_token();
        // Count the number of hash
        let mut h = 1;
        let mut start = self.current_index;
        while token.token_type == TokenType::Hash{            
            h +=1;
            self.update_start_index();
            start = self.current_index;
            token = self.scan_token();
        }

        // Now scan the text of the header        
        while self.peek() != '\n' && self.peek() != '\n' {
            self.advance();
        }
        let end = self.current_index;

        Chunk{
            line: self.line,
            start,
            end,
            class: ChunkClass::Header(h)
        }

    } 

    /// Parses a markdown file, returning a vector of Chunks.
    fn parse(&mut self)->Result< Vec<Chunk>, String>{

        let mut chunks : Vec<Chunk>= Vec::new();

        while !self.finished{
            let token = self.scan_token();

            let chunk = match token.token_type{
                TokenType::Hash =>{
                    self.header()
                }
                _ => {
                    return Err(make_error_msg(format!("Unexpected token '{}'", token.token_type), self.line))
                }
            };

            chunks.push(chunk);
        }

        Ok(chunks)
    }

}


#[cfg(test)]
mod testing{
    use super::*;

    #[test]
    fn test_identifier(){
        ////////////////////////////
        // FIRST, USING IDENTIFIER
        ////////////////////////////
        let source = b"auto perro avion\n";
        let mut scanner = MDScanner::new(source, 1);
        
        
        let found = std::str::from_utf8(scanner.word().txt).unwrap();        
        assert_eq!(found, "auto");
        
        // this is called by scan_token()
        scanner.skip_white_space();
        scanner.start_index = scanner.current_index;
        let found = std::str::from_utf8(scanner.word().txt).unwrap();        
        assert_eq!(found, "perro");
        
        // this is called by scan_token()
        scanner.skip_white_space();
        scanner.start_index = scanner.current_index;
        let found = std::str::from_utf8(scanner.word().txt).unwrap();        
        assert_eq!(found, "avion");
        
        // this is called by scan_token()
        scanner.start_index = scanner.current_index;
        scanner.skip_white_space();
        assert!(scanner.finished);

        ////////////////////////////
        // THEN, USING SCAN_TOKEN()
        ////////////////////////////
        let source = b"auto perro avion\n";
        let mut scanner = MDScanner::new(source, 1);


        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Word);
        let found = std::str::from_utf8(token.txt).unwrap();        
        assert_eq!(found, "auto");
        
        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Word);
        let found = std::str::from_utf8(token.txt).unwrap();        
        assert_eq!(found, "perro");
        
        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Word);
        let found = std::str::from_utf8(token.txt).unwrap();        
        assert_eq!(found, "avion");

        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::EOF);
        let found = std::str::from_utf8(token.txt).unwrap();        
        assert_eq!(found, "");
                
        assert!(scanner.finished);
    }

    #[test]
    fn test_word(){
        ////////////////////////////
        // FIRST, USING IDENTIFIER
        ////////////////////////////
        let source = b"auto perro avion\n";
        let mut scanner = MDScanner::new(source, 1);
        
        
        let found = std::str::from_utf8(scanner.word().txt).unwrap();        
        assert_eq!(found, "auto");
        
        // this is called by scan_token()
        scanner.skip_white_space();
        scanner.start_index = scanner.current_index;
        let found = std::str::from_utf8(scanner.word().txt).unwrap();        
        assert_eq!(found, "perro");
        
        // this is called by scan_token()
        scanner.skip_white_space();
        scanner.start_index = scanner.current_index;
        let found = std::str::from_utf8(scanner.word().txt).unwrap();        
        assert_eq!(found, "avion");
        
        // this is called by scan_token()
        scanner.start_index = scanner.current_index;
        scanner.skip_white_space();
        assert!(scanner.finished);

        ////////////////////////////
        // THEN, USING SCAN_TOKEN()
        ////////////////////////////
        let source = b"auto perro avion\n";
        let mut scanner = MDScanner::new(source, 1);
        

        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Word);
        let found = std::str::from_utf8(token.txt).unwrap();        
        assert_eq!(found, "auto");
        
        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Word);
        let found = std::str::from_utf8(token.txt).unwrap();        
        assert_eq!(found, "perro");
        
        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Word);
        let found = std::str::from_utf8(token.txt).unwrap();        
        assert_eq!(found, "avion");

        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::EOF);
        let found = std::str::from_utf8(token.txt).unwrap();        
        assert_eq!(found, "");
                
        assert!(scanner.finished);

    }


    #[test]
    fn test_line_contains(){
        let source = b" ``` aa ee control asd\nespejo con azucar\nperro";
        let mut scanner = MDScanner::new(source, 1);
        assert!(scanner.line_contains(b"control"));
        let word = scanner.scan_token();
        assert_eq!(word.txt, b"espejo");
        

        let source = b" ``` aa ee  asd\ncontrol espejo con azucar\nperro";
        let mut scanner = MDScanner::new(source, 1);
        assert!(!scanner.line_contains(b"control"));
        let word = scanner.scan_token();
        assert_eq!(word.txt, b"control");
        
    }

    fn check_token_types( source: &[u8], exp_token_types: Vec<TokenType>) -> Result<(),String>{        
        let mut scanner = MDScanner::new(&source, 1);        
        for (i,exp_type) in exp_token_types.iter().enumerate(){
            let found_token = scanner.scan_token();
            if found_token.token_type != *exp_type{
                return Err(format!("Expecting Token {} to be of type '{:?}'... found '{:?}'", i, exp_type, found_token.token_type));
            }

        }
        Ok(())
    }   

    #[test]
    fn test_markdownmode_scan_single_character(){
        
        let source = b"[]#_*!()`:";
        check_token_types(
             source, 
            vec![TokenType::LeftBracket, TokenType::RightBracket, TokenType::Hash, TokenType::Underscore, TokenType::Star,TokenType::Bang,  TokenType::LeftParen, TokenType::RightParen, TokenType::CodeBoundary, TokenType::Colon]
        ).unwrap();
    }// End of test_single_char

    #[test]
    fn test_markdownmode_scan_double_character(){
        
        let source = b"***";
        check_token_types(
            source, 
            vec![TokenType::StarStar, TokenType::Star]
        ).unwrap();
    }// End of test_single_char

    #[test]
    fn test_markdownmode_scan_triple_character(){
        
        let source = b"---```";
        check_token_types(
             source, 
            vec![TokenType::HorizontalRule, TokenType::SimpleBuildingBlockLimit]
        ).unwrap();

        let source = b"---``` control";
        check_token_types(
             source, 
            vec![TokenType::HorizontalRule, TokenType::ControlBlockLimit]
        ).unwrap();

        let source = b"---```control";
        check_token_types(
             source, 
            vec![TokenType::HorizontalRule, TokenType::ControlBlockLimit]
        ).unwrap();
    }// End of test_single_char


    #[test]
    fn test_header(){

        let src = b" ## Car with wheels
        this should not be in the header
        ";
        let mut scanner = MDScanner::new(src, 1);
        let head = scanner.header();
        let txt = scanner.borrow_slice(head.start, head.end);
        println!("'{}'", std::str::from_utf8(txt).unwrap());

    }
}