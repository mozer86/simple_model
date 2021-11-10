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
use crate::model::SimpleModel;
use crate::simulation_state::SimulationStateHeader;

use crate::substance::Substance;
use crate::material::Material;
use crate::construction::Construction;
use crate::surface::Surface;
use crate::fenestration::Fenestration;

use crate::scanner::tokens::*;


pub fn make_error_msg(msg: String, ln: usize)->String{
    format!("Error [in line {}]: {}", ln, msg)
}






pub struct SimpleScanner<'a> {
    
    pub line : usize,    

    source: &'a [u8],

    current_index: usize,

    start_index: usize,

    pub error_msg: String,
    
    finished: bool, 
        
}



impl <'a>SimpleScanner<'a> {

    /// Creates a new [`SimpleScanner`]
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
    /// [`SimpleScanner`] as finished and return `None`
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


    /// Consumes a number, producing a [`Token`]
    fn number(&mut self)->Token<'a>{        
        
        // Scan the first part
        while self.peek().is_ascii_digit(){            
            self.advance();            
        }
        if self.peek() == '.' && self.peek_next().is_ascii_digit(){            
            // Consume the .            
            self.advance();
            while self.peek().is_ascii_digit() {                
                self.advance();                
            }        
        }
        

        self.make_token(TokenType::Number)
        
    }

    /// Consumes a string (i.e., a term in "quotes")
    fn string(&mut self, delimiter: char )->Token<'a>{        
        // Token will have this line reported
        let start_line = self.line;
                      
        let mut next = self.peek();
        

        // Advance as much as possible
        while next != delimiter && !self.finished{                        
            if next == '\n' {
                self.line +=1 ;                
            }            
            next = match self.advance(){
                Some(v) => v,
                None => {
                    self.error_msg = format!("Unterminated string, started at line {}", start_line);
                    return self.make_token(TokenType::Error);
                }
            };            
        }
        

        let mut ret = self.make_token(TokenType::TokenString);
        ret.line = start_line;
        ret
    }

    /// Consumes a whole Identifier
    fn identifier(&mut self)->Token<'a>{        
        
        // scan the whole thing, until we find something that
        // is not a number, letter or '_'       
        let mut c = self.peek();
        while c.is_ascii_alphabetic() || c.is_ascii_digit() || c == '_' {
            match self.advance(){
                Some(_) => {c = self.peek()},
                None => return self.make_token(TokenType::EOF)
            }
        }

        // Check if this is a keyword
        let c = self.source[self.start_index];
        match c as char {            
            'u' =>{
                if self.check_keyword("use"){
                    return self.make_token(TokenType::Use);
                }
            },            
            _ => {/*JUST GET OUT OF THIS MATCH*/}
        }
        
        // If not a keyword, but the next two are ':', this 
        // is an EnumName (e.g., Infiltration::Blast)        
        if self.peek() == ':' && self.peek_next()==':'{
            self.make_token(TokenType::TokenEnumName)                
        }else{
            self.make_token(TokenType::Identifier)
        }
        
    }

    
    /// Checks whether the `char`s between the [`SimpleScanner`]s `start_index` and `current_index`  
    /// match a certain `str` (i.e., a keyword).
    fn check_keyword(&self, word: &str)-> bool {
        
        let length = self.current_index - self.start_index;
        
        // If they are of different length, don't bother
        // checking... they are not the same word
        if word.len() != length {
            return false;
        }
                  
        let mut i = self.start_index;
        // For each character in keyword
        for ch in word.bytes() {                                                
            if self.source[i] != ch {
                return false;
            }            
            // Move one char ahead
            i+=1;
        }
        
        true
    }

    /// Skips the white spaces and the comments and all 
    /// those things.
    pub fn skip_white_space(&mut self){
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
                '/' => {                        
                    if self.peek_next() == '/'{
                        // Single line comment
                        while self.peek() != '\n' && !self.finished {
                            self.advance().unwrap();
                        }                                   
                    }else if self.peek_next() == '*'{
                        // Consume slash and star
                        self.advance().unwrap();
                        self.advance().unwrap();
                        // Block comment
                        loop {                                
                            // Check if it is end
                            if self.finished {                                 
                                return;
                            }

                            // Check if end of block comment
                            if self.peek() == '*' && self.peek_next() == '/' {                                                                    
                                // Consume slash and star
                                self.advance().unwrap();
                                self.advance().unwrap();                                                                                                
                                break; // get out of the block comment loop                                
                                
                            }
                            if let '\n' = self.advance().unwrap(){
                                self.line += 1;                                 
                            }
                        }
                    }else{
                        return;
                    }
                }
                _ => return 
            };
            
        }
    }

    
    


    

    /// Scans a `Token` in `SimpleBuilding` read mode
    pub fn scan_token(&mut self)->Token<'a>{
        self.skip_white_space();        
        self.start_index = self.current_index;

        let c = match self.advance(){
            Some(v)=>v,                
            None=> return self.make_token( TokenType::EOF)
        };
        
        // Alphabetic or underscore allowed
        if c.is_ascii_alphabetic() || c == '_'{            
            return self.identifier();
        }  

        // 0..9 allowed
        if c.is_ascii_digit(){
            return self.number();
        }

        match c {
            
            // Single character            
            '{' => self.make_token( TokenType::LeftBrace),
            '}' => self.make_token( TokenType::RightBrace),
            '(' => self.make_token( TokenType::LeftParen),
            ')' => self.make_token( TokenType::RightParen),
            '[' => self.make_token( TokenType::LeftBracket),
            ']' => self.make_token( TokenType::RightBracket),
            ',' => self.make_token( TokenType::Comma),                        
            ':' =>{
                if self.match_char(':'){
                    self.make_token(TokenType::ColonColon)
                }else{
                    self.make_token(TokenType::Colon)
                }
            },                        
            '"' => { self.string('"') },
            '\'' => { self.string('\'') },
            '`' =>{
                if self.peek() == '`' && self.peek_next() == '`'{
                    self.advance().unwrap();
                    self.advance().unwrap();                    
                    self.start_index = self.current_index;
                    // Go back to markdown                    
                    return self.make_token(TokenType::SimpleBuildingBlockLimit)
                    
                }else{
                    return self.make_token(TokenType::CodeBoundary)
                }
            }

            '\0' =>{
                self.make_token(TokenType::EOF)
            },

            // Error            
            _ => {
                self.error_msg = make_error_msg(format!("Unexpected character '{}' ",c), self.line);
                self.make_token(TokenType::Error)
            }
        }        
    }

    
    /// Consumes an object and returns the start and end of that object.
    pub fn get_object_slice(&mut self)->(usize,usize){
        let mut levels = 0;
        let mut started = false;
        // Check if we are solving an Enum or an Object
        let (open,close) = if self.peek() == ':' && self.peek_next()==':'{
            // If we start with Colon Colon, then it is an enum
            ('(',')')
        }else{
            // Otherwise, it is  an object
            ('{','}')
        };        

        while levels > 0 || !started  {        
            let next = self.peek();

            if next == open {
                levels += 1;
                started = true;
            }
            if next == close {
                levels -= 1;
            }
            if next == '\n' {
                self.line +=1 ;                
            }     
            self.current_index +=1;

            if self.current_index == self.source.len(){
                self.finished = true;
                break;
            }
        }        
        
        // return
        (self.start_index, self.current_index)        
    }

    pub fn scan_field(&mut self)->Result<(Token,Token),String>{
        

        // Scan the name of the field
        let field_name = self.scan_token();
        if field_name.token_type != TokenType::Identifier{
            return Err(make_error_msg(format!("uxpecting field identifier, found {:?} '{}'", field_name.token_type, std::str::from_utf8(field_name.txt).unwrap()), self.line))
        }

        let colon = self.scan_token();
        if colon.token_type != TokenType::Colon{
            return Err(make_error_msg(format!("unexpected token '{}'... expecting ':' separating field names and values", std::str::from_utf8(field_name.txt).unwrap()), self.line))
        }

        let value = self.scan_token();


        self.start_index = self.current_index;

        return Ok((field_name, value))
    }

    /// Checks whether a certain field is in the 
    /// source code. 
    /// 
    /// Note that this function is not meant to be 
    /// fast, as it intends to search for a field within a small source.
    pub fn get_field(&mut self)->Result<(Token,Token),String>{
        
        let fieldname = self.scan_token();
        if fieldname.token_type != TokenType::Identifier {
            return Err(
                make_error_msg(format!("expecting fieldname to be an Identifier... found '{}'", fieldname.token_type), self.line)
            )
        }
        let colon = self.scan_token();
        if colon.token_type != TokenType::Colon {
            return Err(
                make_error_msg(format!("Expecting colon (':') to separate field names and values... found '{}'", colon.token_type), self.line)
            )
        }
        let fieldvalue = self.scan_token();
        
        Ok((fieldname, fieldvalue))        
    }
    
    
    pub fn parse_model(&mut self)->Result<(SimpleModel, SimulationStateHeader),String>{
                
        // identify elements in the code
        let mut buildings : Vec<(usize, usize,usize)> = Vec::with_capacity(1);
        let mut substances : Vec<(usize, usize,usize)> = Vec::new();
        let mut materials : Vec<(usize, usize,usize)> = Vec::new();
        let mut constructions : Vec<(usize, usize,usize)> = Vec::new();

        let mut spaces : Vec<(usize, usize,usize)> = Vec::new();
        let mut surfaces : Vec<(usize, usize,usize)> = Vec::new();
        let mut fenestrations : Vec<(usize, usize,usize)> = Vec::new();
        let mut hvacs : Vec<(usize, usize,usize)> = Vec::new();
        let mut luminaires : Vec<(usize, usize,usize)> = Vec::new();

        while !self.finished {
            let identifier = self.scan_token();
            self.update_start_index();
            // fail if it is not an identifier
            if identifier.token_type != TokenType::Identifier {
                
                eprintln!("{}",
                    make_error_msg(format!("Unexpected token of type {:?}", identifier.token_type), self.line)
                );    
                
            }
            let line = self.line;
            let (start, end) = self.get_object_slice();
            match identifier.txt {     
                b"Building"=>{                    
                    buildings.push((line, start, end));
                },           
                b"Substance"=>{
                    substances.push((line, start, end));
                },
                b"Material"=>{
                    materials.push((line, start, end));
                },
                b"Construction"=>{
                    constructions.push((line, start, end));
                },
                b"Surface"=>{
                    surfaces.push((line, start, end));
                },                
                b"Space"=>{
                    spaces.push((line, start, end));
                },
                b"Fenestration"=>{
                    fenestrations.push((line, start, end));
                },
                b"HVAC"=>{
                    hvacs.push((line, start, end));
                },
                b"Luminaire"=>{
                    luminaires.push((line, start, end));
                }
                _ => {
                    let typename = std::str::from_utf8(identifier.txt).unwrap();
                    eprintln!("{}",                    
                        make_error_msg(format!("Unknown Object type {}", typename), self.line)
                    );                    
                }
            }
        }
    
        // NOW BUILD
        let mut model = SimpleModel::new("the_model".to_string());
        let  mut state_header = SimulationStateHeader::new();
        
        for (line, start,end) in substances{
            let bytes = self.borrow_slice(start,end);
            match Substance::from_bytes(line, bytes, &model){
                Ok(s)=>{model.add_substance(s);},
                Err(e)=>eprintln!("{}", e)
            };
            
        }

        for (line, start,end) in materials{
            let bytes = self.borrow_slice(start,end);
            match Material::from_bytes(line, bytes, &model){
                Ok(s)=>{model.add_material(s);},
                Err(e)=>eprintln!("{}", e)
            };          
        }

        for (line, start,end) in constructions{
            let bytes = self.borrow_slice(start,end);            
            match Construction::from_bytes(line, bytes, &model){
                Ok(s)=>{model.add_construction(s);},
                Err(e)=>eprintln!("{}", e)
            };
        }

        for (line, start,end) in surfaces {
            let bytes = self.borrow_slice(start,end);            
            match Surface::from_bytes(line, bytes, &model){
                Ok(s)=>{model.add_surface(s);},
                Err(e)=>eprintln!("{}", e)
            };
        }

        for (line, start,end) in fenestrations {
            let bytes = self.borrow_slice(start,end);            
            match Fenestration::from_bytes(line, bytes, &model){
                Ok(s)=>{model.add_fenestration(s, &mut state_header);},
                Err(e)=>eprintln!("{}", e)
            };
        }
        
        

        // Return        
        // Ok(building)
        Ok((model, state_header))
    }

}
/***********/
/* TESTING */
/***********/

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_scanner_advance(){
        let source = b"hello";
        
        let mut scan = SimpleScanner::new(source, 1);

        assert_eq!(scan.line,1);        
        assert_eq!(scan.source[scan.start_index], source[scan.start_index]);
        assert_eq!(scan.source[scan.current_index], source[scan.current_index]);

        for i in 0..source.len() {            
            assert_eq!(scan.current_index,i);

            let c = match scan.advance(){
                Some(v)=>v,
                None => panic!("PANIC!!")
            };                
            assert_eq!(c, source[i] as char);
        }
        assert_eq!(scan.current_index,source.len());        
        match scan.advance(){
            Some(v)=>panic!("Retrieved {}... should not have", v),
            None => assert!(true)
        };
    }

    #[test]
    fn test_scan_empty_string(){
        
        let source = b"";        
        let mut scanner = SimpleScanner::new(source, 1);

        assert!(scanner.finished);

        let token = scanner.scan_token();
        assert_eq!(TokenType::EOF, token.token_type);        
    }

    
    

    fn check_token_types( source: &[u8], exp_token_types: Vec<TokenType>) -> Result<(),String>{        
        let mut scanner = SimpleScanner::new(&source, 1);        
        for (i,exp_type) in exp_token_types.iter().enumerate(){
            let found_token = scanner.scan_token();
            if found_token.token_type != *exp_type{
                return Err(format!("Expecting Token {} to be of type '{:?}'... found '{:?}'", i, exp_type, found_token.token_type));
            }

        }
        Ok(())
    }    

    #[test]
    fn test_simplemode_scan_single_character(){
        
        let source = b":,{}[]";
        check_token_types(
             source, 
            vec![TokenType::Colon, TokenType::Comma,TokenType::LeftBrace, TokenType::RightBrace, TokenType::LeftBracket, TokenType::RightBracket]
        ).unwrap();

        let source = b"[]:,{}[]";
        check_token_types(
            source, 
            vec![TokenType::LeftBracket, TokenType::RightBracket,TokenType::Colon, TokenType::Comma,TokenType::LeftBrace, TokenType::RightBrace, TokenType::LeftBracket, TokenType::RightBracket]
        ).unwrap();
    }// End of test_single_char

    

    #[test]
    fn test_simplemode_scan_double_character(){
        
        let source = b":::";
        check_token_types(
             source, 
            vec![TokenType::ColonColon, TokenType::Colon]
        ).unwrap();


    }// End of test_single_char

    

    #[test]
    fn test_simplemode_scan_triple_character(){
        
        let source = b"```";
        check_token_types(
            source, 
            vec![TokenType::SimpleBuildingBlockLimit]
        ).unwrap();
        let mut scanner = SimpleScanner::new(source, 1);        
        scanner.scan_token();        
        


    }// End of test_single_char

    

    #[test]
    fn test_get_object_slice(){
        let src = b"SimpleModel {
            name: \"The apartment\"
        }
        Substance {
            name : \"Some substance\",
            anonymous_car: Car {
                speed: 100.,                
            }
            thermal_conductivity : 2.31,
        }
        ";
        let mut scanner = SimpleScanner::new(src, 1);

        let ident = scanner.scan_token();
        assert_eq!(ident.token_type, TokenType::Identifier);
        assert_eq!(ident.txt, b"SimpleModel");
        let (start,end) = scanner.get_object_slice();
        let slice = &scanner.source[start..end];
        println!("{}\n====", std::str::from_utf8(slice).unwrap());

        let ident = scanner.scan_token();
        assert_eq!(ident.token_type, TokenType::Identifier);
        assert_eq!(ident.txt, b"Substance");
        let (start,end) = scanner.get_object_slice();
        let slice = &scanner.source[start..end];
        println!("{}\n====", std::str::from_utf8(slice).unwrap());
        
    }

    #[test]
    fn test_scan_field(){
        let src = b"
        Substance {
            name : \"Some substance\",
            thermal_conductivity : 2.31
        }
        ";
        let mut scanner = SimpleScanner::new(src, 1);

        let ident = scanner.scan_token();
        assert_eq!(ident.token_type, TokenType::Identifier);
        assert_eq!(ident.txt, b"Substance");

        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::LeftBrace);
        
        if let Ok((fieldname, value)) = scanner.scan_field(){
            assert_eq!("name", std::str::from_utf8(fieldname.txt).unwrap());
            assert_eq!("Some substance", value.resolve_as_string().unwrap());
        }
        
        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Comma);

        
        if let Ok((fieldname, value)) = scanner.scan_field(){
            assert_eq!("thermal_conductivity", std::str::from_utf8(fieldname.txt).unwrap());
            assert_eq!(2.31, value.resolve_as_float().unwrap());
            assert!(value.resolve_as_usize().is_err());
        }
    }    
    

    #[test]
    fn test_get_field(){
        let src = b"car : 2, bus: \"el bus\"}";
        let mut scanner = SimpleScanner::new(src, 1);
        if let Ok((name, value)) = scanner.get_field(){
            assert_eq!("car".to_string(), std::str::from_utf8(name.txt).unwrap());
            assert_eq!(2, value.resolve_as_usize().unwrap());                     
        }else{
            panic!("Could not read field")
        }

        assert_eq!(scanner.scan_token().token_type, TokenType::Comma);

        
        if let Ok((name, value)) = scanner.get_field(){
            assert_eq!("bus".to_string(), std::str::from_utf8(name.txt).unwrap());
            assert_eq!("el bus".to_string(), value.resolve_as_string().unwrap());                        
        }else{
            panic!("Could not read field")
        }

    }


    #[test]
    fn test_identifier(){
        

        ////////////////////////////
        // THEN, USING SCAN_TOKEN()
        ////////////////////////////
        let source = b"auto perro avion\n";
        let mut scanner = SimpleScanner::new(source, 1);        


        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Identifier);
        let found = std::str::from_utf8(token.txt).unwrap();        
        assert_eq!(found, "auto");
        
        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Identifier);
        let found = std::str::from_utf8(token.txt).unwrap();        
        assert_eq!(found, "perro");
        
        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::Identifier);
        let found = std::str::from_utf8(token.txt).unwrap();        
        assert_eq!(found, "avion");

        let token = scanner.scan_token();
        assert_eq!(token.token_type, TokenType::EOF);
        let found = std::str::from_utf8(token.txt).unwrap();        
        assert_eq!(found, "");
                
        assert!(scanner.finished);

    }
}