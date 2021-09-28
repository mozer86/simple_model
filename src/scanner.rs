use crate::model::SimpleModel;

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


/// An `enum` indicating which kind of file 
/// are we reading.
#[derive(Debug,Eq,PartialEq)]
pub enum ReadMode{
    Markdown,
    SimpleBuilding,
    Rhai
}

#[derive(Clone,Copy)]
pub struct Token<'a> {
    pub line: usize,    
    pub length: usize,
    pub start: usize,          
    pub token_type: TokenType,
    pub txt: &'a [u8],
}

impl <'a>Token<'a>{
    pub fn resolve_as_float(&self) -> Result<f64,String> {
        let txt = std::str::from_utf8(self.txt).unwrap();
        if let TokenType::Number = self.token_type{
             match txt.parse::<f64>(){
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
                 Err(_)=>Err(format!("value '{}' does not seem to be a positive integer. Hint: remove dots, e.g., don't write '12.', write '12'", txt))
             }             
        }else{
            Err(format!("Token '{}' cannot be transformed into a positive integer", txt))
        }
    }

    pub fn resolve_as_string(&self) -> Result<String,String> {
        let txt = std::str::from_utf8(self.txt).unwrap();
        if let TokenType::TokenString = self.token_type{
             Ok(txt[1..txt.len()-1].to_string())         
        }else{
            Err(format!("Token '{}' cannot be transformed into a String", txt))
        }
    }
}

pub struct Scanner<'a> {
    
    line : usize,    

    source: &'a [u8],

    current_index: usize,

    start_index: usize,

    pub error_msg: String,
    
    finished: bool, 
    
    pub read_mode: ReadMode,
}



impl <'a>Scanner<'a> {

    /// Creates a new [`Scanner`]
    pub fn new(source : &'a [u8])->Self{
        Self {
            finished: source.is_empty(),
            source,
            line: 1,                        
            current_index: 0,
            start_index: 0,
            error_msg : "".to_string(),
            read_mode: ReadMode::SimpleBuilding,
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
    /// [`Scanner`] as finished and return `None`
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

    /// Skips the white spaces and the comments and all 
    /// those things.
    pub fn skip_white_space(&mut self){
        match self.read_mode{
            ReadMode::Markdown => self.markdown_skip_white_space(),
            _ => self.simple_building_skip_white_space()
        }
    }

    fn markdown_skip_white_space(&mut self){

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

    /// Skips the white spaces and the comments and all 
    /// those things.
    fn simple_building_skip_white_space(&mut self){        
        
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
    }// end of skip white space SimpleBuilding mode


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
    fn string(&mut self)->Token<'a>{        
        // Token will have this line reported
        let start_line = self.line;
                      
        let mut next = self.peek();
        

        // Advance as much as possible
        while next != '"' && !self.finished{                        
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
            let ret = self.make_token(TokenType::TokenEnumName);
            self.advance();
            self.advance();            

            ret                      

        }else{
            self.make_token(TokenType::Identifier)
        }
        
    }

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
    
    /// Checks whether the `char`s between the [`Scanner`]s `start_index` and `current_index`  
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

    /// Scans a [`Token`] in `SimpleBuilding` read mode
    pub fn scan_simple_building_token(&mut self)->Token<'a>{
        debug_assert_eq!(self.read_mode, ReadMode::SimpleBuilding);

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
            '"' => { self.string() },
            '`' =>{
                if self.peek() == '`' && self.peek_next() == '`'{
                    self.advance().unwrap();
                    self.advance().unwrap();                    
                    self.start_index = self.current_index;
                    // Go back to markdown
                    self.read_mode=ReadMode::Markdown;
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
                self.error_msg = format!("Unexpected character '{}' at line {} ",c, self.line);
                self.make_token(TokenType::Error)
            }
        }        
    }

    /// Scans a [`Token`] in `Markdown` read mode
    pub fn scan_markdown_token(&mut self)->Token<'a>{
        debug_assert_eq!(self.read_mode, ReadMode::Markdown);

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
                // self.error_msg = format!("Unexpected character '{}' at line {} ",c, self.line);
                // return self.make_token(TokenType::Error)
            }
        };
        
        // Otherwise, return a Word
        self.word()        
    }

    

    /// Scans a whole token.
    pub fn scan_token(&mut self) -> Token<'a> {
        self.skip_white_space();        
        
        
        self.start_index = self.current_index;
        match self.read_mode{
            ReadMode::Markdown =>{
                self.scan_markdown_token()
            },
            ReadMode::Rhai => {
                unimplemented!()
            },
            ReadMode::SimpleBuilding =>{
                self.scan_simple_building_token()
            }
        }
    }

    /// Consumes an object and returns the start and end of that object.
    pub fn get_object_slice(&mut self)->(usize,usize){
        let mut levels = 0;
        let mut started = false;
        while levels > 0 || !started  {            
            if self.peek() == '{'{
                levels += 1;
                started = true;
            }
            if self.peek() == '}'{
                levels -= 1;
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
            return Err(format!(" Expecting field identifier, found {:?} '{}'", field_name.token_type, std::str::from_utf8(field_name.txt).unwrap()))
        }

        let colon = self.scan_token();
        if colon.token_type != TokenType::Colon{
            return Err(format!(" Unexpected token '{}'... expecting ':' separating field names and values", std::str::from_utf8(field_name.txt).unwrap()))
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
            return Err(format!("Expecting fieldname to be an Identifier... found '{}'", fieldname.token_type))
        }
        let colon = self.scan_token();
        if colon.token_type != TokenType::Colon {
            return Err(format!("Expecting colon (':') to separate field names and values... found '{}'", colon.token_type))
        }
        let fieldvalue = self.scan_token();
        
        Ok((fieldname, fieldvalue))        
    }
    
    pub fn parse_building(&mut self)->Result<SimpleModel,String>{
        
        
        // identify elements in the code
        let mut buildings : Vec<(usize,usize)> = Vec::with_capacity(1);
        let mut substances : Vec<(usize,usize)> = Vec::new();
        let mut materials : Vec<(usize,usize)> = Vec::new();
        let mut constructions : Vec<(usize,usize)> = Vec::new();

        let mut surfaces : Vec<(usize,usize)> = Vec::new();
        let mut spaces : Vec<(usize,usize)> = Vec::new();
        let mut fenestrations : Vec<(usize,usize)> = Vec::new();
        let mut hvacs : Vec<(usize,usize)> = Vec::new();
        let mut luminaires : Vec<(usize,usize)> = Vec::new();

        while !self.finished {
            let identifier = self.scan_token();
            // fail if it is not an identifier
            if identifier.token_type != TokenType::Identifier {
                return Err(format!("Unexpected token of type {:?}", identifier.token_type));
            }
            match identifier.txt {
                b"SimpleModel" => {
                    // only one building allowed
                    if buildings.len() > 0{
                        return Err(format!("More than one 'SimpleModel' found"))
                    }                    
                    buildings.push(self.get_object_slice());
                },
                b"Substance"=>{
                    substances.push(self.get_object_slice());
                },
                b"Material"=>{
                    materials.push(self.get_object_slice());
                },
                b"Construction"=>{
                    constructions.push(self.get_object_slice());
                },
                b"Surface"=>{
                    surfaces.push(self.get_object_slice());
                },                
                b"Space"=>{
                    spaces.push(self.get_object_slice());
                },
                b"Fenestration"=>{
                    fenestrations.push(self.get_object_slice());
                },
                b"HVAC"=>{
                    hvacs.push(self.get_object_slice());
                },
                b"Luminaire"=>{
                    luminaires.push(self.get_object_slice());
                }
                _ => {
                    let typename = std::str::from_utf8(identifier.txt).unwrap();
                    return Err(format!("Unknown Object type {}", typename));
                }
            }
        }
    
        // NOW BUILD
        // let building = match buildings.get(0){
        //     Some(b)=>SimpleModel::new("as".to_string()),//SimpleModel::from_slice(b),
        //     None =>{return Err(format!("No building found..."))}
        // };
        
        //

        // Return        
        // Ok(building)
        Ok(SimpleModel::new("a".to_string()))
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
        
        let mut scan = Scanner::new(source);

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
        let mut scanner = Scanner::new(source);

        assert!(scanner.finished);

        let token = scanner.scan_token();
        assert_eq!(TokenType::EOF, token.token_type);        
    }

    #[test]
    fn test_identifier(){
        ////////////////////////////
        // FIRST, USING IDENTIFIER
        ////////////////////////////
        let source = b"auto perro avion\n";
        let mut scanner = Scanner::new(source);
        scanner.read_mode = ReadMode::SimpleBuilding;
        
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
        let mut scanner = Scanner::new(source);
        scanner.read_mode = ReadMode::SimpleBuilding;


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

    #[test]
    fn test_word(){
        ////////////////////////////
        // FIRST, USING IDENTIFIER
        ////////////////////////////
        let source = b"auto perro avion\n";
        let mut scanner = Scanner::new(source);
        scanner.read_mode = ReadMode::Markdown;
        
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
        let mut scanner = Scanner::new(source);
        scanner.read_mode = ReadMode::Markdown;


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
        let mut scanner = Scanner::new(source);
        assert!(scanner.line_contains(b"control"));
        let word = scanner.scan_token();
        assert_eq!(word.txt, b"espejo");
        

        let source = b" ``` aa ee  asd\ncontrol espejo con azucar\nperro";
        let mut scanner = Scanner::new(source);
        assert!(!scanner.line_contains(b"control"));
        let word = scanner.scan_token();
        assert_eq!(word.txt, b"control");
        
    }

    fn check_token_types(read_mode: ReadMode, source: &[u8], exp_token_types: Vec<TokenType>) -> Result<(),String>{        
        let mut scanner = Scanner::new(&source);
        scanner.read_mode = read_mode;
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
            ReadMode::SimpleBuilding, source, 
            vec![TokenType::Colon, TokenType::Comma,TokenType::LeftBrace, TokenType::RightBrace, TokenType::LeftBracket, TokenType::RightBracket]
        ).unwrap();

        let source = b"[]:,{}[]";
        check_token_types(
            ReadMode::SimpleBuilding, source, 
            vec![TokenType::LeftBracket, TokenType::RightBracket,TokenType::Colon, TokenType::Comma,TokenType::LeftBrace, TokenType::RightBrace, TokenType::LeftBracket, TokenType::RightBracket]
        ).unwrap();
    }// End of test_single_char

    #[test]
    fn test_markdownmode_scan_single_character(){
        
        let source = b"[]#_*!()`:";
        check_token_types(
            ReadMode::Markdown, source, 
            vec![TokenType::LeftBracket, TokenType::RightBracket, TokenType::Hash, TokenType::Underscore, TokenType::Star,TokenType::Bang,  TokenType::LeftParen, TokenType::RightParen, TokenType::CodeBoundary, TokenType::Colon]
        ).unwrap();
    }// End of test_single_char

    #[test]
    fn test_simplemode_scan_double_character(){
        
        let source = b":::";
        check_token_types(
            ReadMode::SimpleBuilding, source, 
            vec![TokenType::ColonColon, TokenType::Colon]
        ).unwrap();


    }// End of test_single_char

    #[test]
    fn test_markdownmode_scan_double_character(){
        
        let source = b"***";
        check_token_types(
            ReadMode::Markdown, source, 
            vec![TokenType::StarStar, TokenType::Star]
        ).unwrap();
    }// End of test_single_char

    #[test]
    fn test_simplemode_scan_triple_character(){
        
        let source = b"```";
        check_token_types(
            ReadMode::SimpleBuilding, source, 
            vec![TokenType::SimpleBuildingBlockLimit]
        ).unwrap();
        let mut scanner = Scanner::new(source);
        scanner.read_mode = ReadMode::SimpleBuilding;
        scanner.scan_token();        
        // we should have returned to markdown
        assert_eq!(scanner.read_mode, ReadMode::Markdown);


    }// End of test_single_char

    #[test]
    fn test_markdownmode_scan_triple_character(){
        
        let source = b"---```";
        check_token_types(
            ReadMode::Markdown, source, 
            vec![TokenType::HorizontalRule, TokenType::SimpleBuildingBlockLimit]
        ).unwrap();

        let source = b"---``` control";
        check_token_types(
            ReadMode::Markdown, source, 
            vec![TokenType::HorizontalRule, TokenType::ControlBlockLimit]
        ).unwrap();

        let source = b"---```control";
        check_token_types(
            ReadMode::Markdown, source, 
            vec![TokenType::HorizontalRule, TokenType::ControlBlockLimit]
        ).unwrap();
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
        let mut scanner = Scanner::new(src);

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
        let mut scanner = Scanner::new(src);

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
        let mut scanner = Scanner::new(src);
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

}