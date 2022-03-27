// use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use crate::field::Field;


pub struct StructObject {
    ident:  syn::Ident,
    fields : Vec<Field>, 
    // optional_fields: Vec<Field>, 
    // state_fields: Vec<Field>,

}




impl StructObject {
    pub fn new(ident: syn::Ident, stru: syn::DataStruct)->Self{
        

        let fields : Vec<syn::Field> = Self::get_object_fields(&stru).into_iter().map(|x| x).collect();

        let fields : Vec<Field> = fields.into_iter().map(|x| Field::new(x)).collect();

        StructObject{
            ident,
            fields
        }

    }


    pub fn gen_from_bytes(&self)-> TokenStream2 {
        let object_name = &self.ident;

         // Create scanner.
        let mut from_bytes = quote!(
            let mut scanner = crate::scanner::SimpleScanner::new(bytes, line);
            let left_brace = scanner.scan_token();
            if left_brace.token_type != crate::scanner::TokenType::LeftBrace{
                return Err(crate::scanner::make_error_msg(format!(" Expecting left brace ('{{') when opening Object, found '{}' ({})", left_brace.token_type, std::str::from_utf8(left_brace.txt).unwrap()), scanner.line))
            }
        );

        /*********************** */
        /* INITIALIZE ALL FIELDS */
        /*********************** */        
        // `Vec` are initialized as an empty vector; `Options` are 
        // initialized as None; and all others `type` are initialized
        // as an `Option<type> = None`... Options are optional and thus
        // they won't be checked... The others should be there and thus 
        // will be `unwraped`
        for f in &self.fields {
            // Skip State elements and index
            if let Field::State(_) = f{
                continue;
            }            
            let f = f.get_initialization();
            from_bytes = quote!(
                #from_bytes                
                #f
            )
        }

        
        /************************************************ */
        /* CREATE A MATCH STATEMENT THAT FILLS ALL FIELDS */
        /************************************************ */
        // Those given will go from None to Some(x)
        let field_value = syn::Ident::new("field_value", proc_macro2::Span::call_site());
        let mut the_match_content = quote!();
        for f in self.fields.iter(){
            if let Field::State(_) = f{
                continue;
            }
            let f_ident = f.data().ident.clone().unwrap();
            
            let field_name = format!("{}",f_ident);
            if field_name == "index"{
                continue; // this should not be input
            }

            let resolution = f.get_resolution(&field_value, false);
            if let Field::Vec(_) = f{
                the_match_content = quote!(
                    #the_match_content
                    
                    #field_name => {
                        #f_ident = #resolution;
                    },
                );
            }else{
                the_match_content = quote!(
                    #the_match_content
                    
                    #field_name => {
                        #f_ident = Some(#resolution);
                    },
                );
            }
        }
        
        let unexpected_field_err = format!("unexpected field '{{}}' in object '{}'", self.ident);

        the_match_content = quote!(
            #the_match_content
            _ => {                
                return Err(crate::scanner::make_error_msg(format!(#unexpected_field_err, txt_str), scanner.line))
            },
        );
        
        // Put everything in a Match statement and loop
        from_bytes = quote!(
            #from_bytes
            
            loop {
                let (field_name, field_value) = scanner.get_field()?;
                let txt_str = std::str::from_utf8(field_name.txt).unwrap();
                match txt_str {
                    #the_match_content
                }
                scanner.skip_white_space();
                match scanner.peek(){
                    ',' => {
                        scanner.scan_token();// scan the comma
                        scanner.skip_white_space();
                        if scanner.peek() == '}'{
                            break;
                        }
                    },// no problem with this.
                    '}' => {break},// we are done
                    '{' => {},// don't do anything... we need to scan that object.
                    '\0' => {return Err(crate::scanner::make_error_msg("unexpected end of file ... ".to_string(), scanner.line))}            
                    _ => {
                        return Err(crate::scanner::make_error_msg(format!("fields must be separated by commas (i.e., ',') .... found character '{}' ", scanner.peek()), scanner.line))
                    }                
                }
                
            }
        );
        
        /********************** */
        /*    VERIFY VALUES     */
        /********************** */    
        for f in self.fields.iter(){
            // Skip State elements and index
            if let Field::State(_) = f{
                continue;
            }
            
            let verification = f.get_verification( &object_name);
            from_bytes = quote!(
                #from_bytes

                #verification

            )
        }


        /************** */
        /* BUILD OBJECT */
        /************** */
        // As per the default object behavior, constructior 
        // takes required parameters... optionals and vectors 
        // are filled later        
        let req_field_names = self.collect_required_field_names();

        from_bytes = quote!(
            #from_bytes

            let mut ret = #object_name ::new( #req_field_names );
        );
        
        // Fill with optional values
        for f in self.fields.iter() {
            match f {
                Field::Option(d) | Field::Vec(d) => {
                    let f_ident = d.ident.clone().unwrap();                
                    // let f_ident = "car";
                    from_bytes = quote!(
                        #from_bytes
                
                        ret.#f_ident = #f_ident;
                    );  
                },
                _ => { /* Others are mandatory... were passed into the constructors */ }            
            }
            
        }
        
        /* RETURN */
        let from_bytes_docstring=format!(" Creates a new [`{}`] by scanning bytes (i.e., a `&[u8]`) acquired from a text file", object_name);
        quote!(
            #[doc = #from_bytes_docstring] 
            pub fn from_bytes(line: usize, bytes: &[u8], model: &crate::model::SimpleModel)->Result<Self, String>{
                #from_bytes

                Ok(ret)
            }
        )  
    }

    pub fn gen_new(&self)->TokenStream2{
        let req_field_names = self.collect_required_fields();
        let new_docstring=format!(" Creates a new [`{}`]", self.ident);        
        let mut content = quote!();

        // Initialize all values

        for f in self.fields.iter(){
            let fname = f.data().ident.clone().unwrap();
            
            match f {
                Field::State(_d)=> {                    
                    content = quote!(
                        #content
                        #fname : std::cell::RefCell::new(None),
                    )
                },
                Field::Option(_) =>{
                    content = quote!(
                        #content
                        #fname : None,
                    )
                },
                Field::Vec(_) =>{
                    content = quote!(
                        #content
                        #fname : Vec::new(),
                    )
                },
                _ =>{
                    content = quote!(
                        #content
                        #fname,
                    )
                }
            }
        }
        

        quote!(
            #[doc = #new_docstring]
            /// 
            /// All the required fields are asked by the constructor. The Optional
            /// fields default to `None`
            pub fn new(#req_field_names)->Self{
                Self{
                    #content
                }
            }
        )
    }

    fn collect_required_field_names(&self)->TokenStream2{
        let mut req_field_names = quote!(); 
        self.fields.iter().enumerate().for_each(|(i,f)|{
            match f {
                Field::Option(_) | Field::State(_) | Field::Vec(_) =>{}  
                _ =>{
                    let f_ident = f.data().ident.clone();
                    if i == 0{
                        req_field_names = quote!(#f_ident);
                    }else{
                        req_field_names = quote!(#req_field_names, #f_ident);
                    }
                }
            }            
        }); 
        req_field_names
    }

    fn collect_required_fields(&self)->TokenStream2{
        let mut req_field_names = quote!(); 
        self.fields.iter().enumerate().for_each(|(i,f)|{
            match f {
                Field::Option(_) | Field::State(_) | Field::Vec(_) => {},
                _ =>{
                    let data = f.data();
                    let f_ident = data.ident.clone();
                    let f_type = data.ty;
                    if i == 0{
                        req_field_names = quote!(#f_ident: #f_type);
                    }else{
                        req_field_names = quote!(#req_field_names, #f_ident : #f_type);
                    }
                }
            }            
        }); 
        req_field_names
    }


    fn get_object_fields(stru: &syn::DataStruct) -> syn::punctuated::Punctuated<syn::Field, syn::token::Comma> {
        if let syn::DataStruct{fields:syn::Fields::Named(syn::FieldsNamed{ ref named, ..}), ..} = stru {
            named.clone()
        }else{
            panic!("Unhandled object when get_object_fields... {:?}", quote!(stru) );
        }
    }


    pub fn gen_state_getters_setters(&self)->TokenStream2{
        let mut gets : TokenStream2 = quote!();
        let mut sets : TokenStream2 = quote!();

        for f in self.fields.iter(){
            // name of the field
            let f_ident = f.data().ident.clone().unwrap();        
            match f {
                Field::State(_d) => {
                    /* SET THE INDEX OF THE OBJECT */
                    // name of the 'set_index_' method
                    let set_ident = format!("set_{}_index", f_ident);
                    let set_ident = syn::Ident::new(&set_ident,f_ident.span());
                    // doc of the 'set_index_' method        
                    let sets_index_doc_string = format!(" Sets the index of the [`SimulationStateElement`] representing the `{}` within the [`SimulationState`]. Will panic if the object already has an index assigned for this field.", f_ident);        
                    let panic_msg = format!("Field '{}' has already been asigned", f_ident);
                    sets = quote!(
                        #sets
        
                        #[doc = #sets_index_doc_string]
                        pub fn #set_ident(&self, i: usize){
                            if self.#f_ident.borrow().is_some(){                    
                                panic!(#panic_msg)
                            }
                            *self.#f_ident.borrow_mut() = Some(i);                        
                        }
                    );
        
                    /* SET THE STATE */
                    let set_ident = format!("set_{}", f_ident);
                    let set_ident = syn::Ident::new(&set_ident,f_ident.span());
                    // doc of the 'set_' method        
                    let sets_doc_string = format!(" Changes the value of the [`SimulationStateElement`] associated with the `{}` within the [`SimulationState`] .", f_ident);        
                    let panic_msg = format!(" Impossible to change the state of object because `{}` has no value", f_ident);
                    sets = quote!(
                        #sets
        
                        #[doc = #sets_doc_string]
                        pub fn #set_ident(&self, state: &mut crate::simulation_state::SimulationState, v : crate::Float){
                            let a = self.#f_ident.borrow();
                            if let Some(i) = *a {
                                state[i] = v;
                            }else{
                                panic!("{}", #panic_msg)
                            }                                       
                        }
                    );
        
                    /* GET THE INDEX OF THE OBJECT */
                    // name of the 'get_index_' method
                    let get_index_ident = format!("{}_index", f_ident);
                    let get_index_ident = syn::Ident::new(&get_index_ident,f_ident.span());        
        
                    // doc of the 'get_index_' method        
                    let get_index_doc_string = format!(" Gets the index of the [`SimulationStateElement`] representing the `{}` within the [`SimulationState`].", f_ident);        
                    
                    gets = quote!(
                        #gets
                        
                        #[doc = #get_index_doc_string]
                        pub fn #get_index_ident(&self) -> Option<usize> {
                            *self.#f_ident.borrow()
                        }
                    );
        
                    /* GET THE STATE OF THE OBJECT */
                    
                    // doc of the 'get_index_' method        
                    let get_doc_string = format!(" Gets the value of the [`SimulationStateElement`] representing the `{}` within the [`SimulationState`].", f_ident);                
                    gets = quote!(
                        #gets
                        
                        #[doc = #get_doc_string]            
                        pub fn #f_ident(&self, state: &crate::simulation_state::SimulationState) -> Option<crate::Float> {
                            if let &Some(i) = &*self.#f_ident.borrow(){
                                Some(state[i])
                            }else{
                                None
                            }                                       
                        }
                    );
                },
                Field::Option(d)=>{
                    let f_ident_str = format!("{}", f_ident);
                    // Type T inside the Option<T>
                    // let ty = extract_type_from_option(&f.ty).expect("When bulding build_optional_get_set() 0");
                    let ty = d.child.clone().unwrap().data().ty;
                    // let ty = syn::Ident::new("f32", proc_macro2::Span::call_site());
                    // ident.unwrap();
                    
                    // Name of the 'set_' method
                    let set_ident = format!("set_{}", f_ident);
                    let set_ident = syn::Ident::new(&set_ident,f_ident.span());
                    // Doc for the 'set_' method
                    let sets_doc_string = format!(" Sets the `{}` field.", f_ident);
                    sets = quote!(
                        #sets

                        #[doc = #sets_doc_string]
                        pub fn #set_ident(&mut self, v: #ty)->&mut Self{
                            self.#f_ident = Some(v);        
                            self
                        }
                    );

                    // doc for the 'get_' method... 
                    // the ident of this method is the name of the field itself
                    let gets_doc_string = format!(" Gets the `{}` field. Returns a `Result` because this field is optional and thus it might not be there.", f_ident);
                    gets = quote!(
                        #gets
                        
                        #[doc = #gets_doc_string]
                        pub fn #f_ident(&self) -> Result<&#ty, String> {
                            match &self.#f_ident {
                                Some(v) => Ok(v),
                                None => Err(format!("{} called '{}' has not been assigned any value for field '{}'", self.object_type(), self.name, #f_ident_str)),
                            }
                        }
                    );
                },                
                _ => { /* Do nothing */}

            }// End of match                
        } // end of fields.iter()


        quote!(
            #gets

            #sets
        )
    }

    
}



