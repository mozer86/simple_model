use syn;
use quote::quote;
use crate::common_path::*;
use proc_macro2::TokenStream as TokenStream2;

pub const STATE_ELEMENT_TYPE : &str = "StateElementField";

#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: String,
    pub value: Option<String>,
}

impl Attribute {
    pub fn new(a: &syn::Attribute)->Self{
        if let syn::AttrStyle::Inner(_) = &a.style{
            panic!("Expecing Outer style attribute")
        }
        let name = a.path.segments[0].ident.clone();
        let mut value : Option<String> = None;
        a.tokens.clone().into_iter().for_each(|token|{
            if let proc_macro2::TokenTree::Group(g) = token {                            
                g.stream().into_iter().for_each(|literal|{
                    if let proc_macro2::TokenTree::Literal(lit) = literal{                        
                        value = Some(format!("{}", lit));
                    }
                })
                
            }
        });
        // return
        Self{
            name: format!("{}", name),
            value
        }        
    }
}






#[derive(Clone)]
pub struct FieldData{
    pub ident: Option<syn::Ident>,
    pub attributes: Vec<Attribute>,
    pub child: Option<Box<Field>>,
    pub ty : syn::Type,
    pub docs: Option<String>,
    pub api_alias: Option<String>,
}



#[derive(Clone)]
pub enum Field {
    Float(FieldData),
    Int(FieldData),
    Bool(FieldData),
    String(FieldData),

    Option(FieldData),
    Vec(FieldData),
    Rc(FieldData),

    Object(FieldData),
    State(FieldData),
}

impl Field {

    /// Creates a new Field object, with no ident and no attributes.
    /// 
    /// This method is meant to be used for nested typs. E.g. the `usize` in `Vec<usize>`
    pub fn from_type(ty: &syn::Type)->Self {
        
        let mut data = FieldData{
            ident: None,
            attributes: Vec::new(),
            child: None,
            ty: ty.clone(),
            docs: None, // This is nested... the Docs should be in the parent
            api_alias: None, // This is nested... the Docs should be in the parent
        };

        if let syn::Type::Path(p) = ty {
            if path_is_float(&p.path){
                Self::Float(data)
            }else if path_is_int(&p.path){
                Self::Int(data)
            }else if path_is_bool(&p.path){
                Self::Bool(data)
            }else if path_is_string(&p.path){
                Self::String(data)
            }else if path_is_option(&p.path){
                let ty = extract_type_from_path(&p.path).unwrap();
                data.child = Some(Box::new(Self::from_type( &ty)));                        
                Self::Option(data)                            
            }else if path_is_vec(&p.path){
                let ty = extract_type_from_path(&p.path).unwrap();
                data.child = Some(Box::new(Self::from_type( &ty)));                        
                Self::Vec(data)                            
            }else if path_is_rc(&p.path){
                let ty = extract_type_from_path(&p.path).unwrap();
                data.child = Some(Box::new(Self::from_type( &ty)));                        
                Self::Rc(data)                            
            }else{                
                let ty_str = path_to_string(&p.path);
                
                if ty_str == STATE_ELEMENT_TYPE {
                    return Self::State(data)
                }
                // Therea re object—e.g., enums—that are stored in other object, not in models
                Self::Object(data)
                
                // if let Some(_location) = crate::object_location(ty_str.clone()){
                //     // data.child = Some(Box::new(Self::from_type(&ident.clone(), &ty)));                        
                // }else{
                //     panic!("When Field::from_type(): Unregistered Object type '{}' in field called '{}'", ty_str, ident.clone())                    
                // }                
            }
        }else{
            panic!("TODO: Error handling 3")
        }
        
    }

    /// Creates a new Field object based on the tokens in an actual Struct.    
    pub fn new(field: syn::Field)-> Self {


        let ident = field.ident.clone();        
        
        if let syn::Type::Path(t) = &field.ty {
            
            
            let attributes : Vec<Attribute> = field.attrs.iter().map(|a|{
                Attribute::new(a)
            }).collect();
            
            let mut api_alias : Option<String> = None;
            for a in attributes.iter(){
                if a.name == "physical" || a.name == "operational"{
                    api_alias = a.value.clone();
                    break;
                }
            };

            let path = t.path.clone();
            let ty  = field.ty.clone();
            let docs = Some(crate::docs::get_docs(&field.attrs));

            let mut data = FieldData{
                ident: ident.clone(),
                attributes,
                child: None,
                ty: ty.clone(),
                docs,
                api_alias,
            };

            if path_is_float(&path){
                Self::Float(data)
            }else if path_is_int(&path){
                Self::Int(data)
            }else if path_is_bool(&path){
                Self::Bool(data)
            }else if path_is_string(&path){
                Self::String(data)
            }else if path_is_option(&path){
                let ty = extract_type_from_path(&path).unwrap();
                data.child = Some(Box::new(Self::from_type( &ty)));                        
                Self::Option(data)                            
            }else if path_is_vec(&path){
                let ty = extract_type_from_path(&path).unwrap();
                data.child = Some(Box::new(Self::from_type( &ty)));                        
                Self::Vec(data)                            
            }else if path_is_rc(&path){
                let ty = extract_type_from_path(&path).unwrap();
                data.child = Some(Box::new(Self::from_type( &ty)));                        
                Self::Rc(data)                            
            }else{                
                let ty_str = path_to_string(&path);                
                if ty_str == STATE_ELEMENT_TYPE {
                    return Self::State(data)
                }
                // Therea re object—e.g., enums—that are stored in other object, not in models                
                Self::Object(data)
                // if let Some(_location) = crate::object_location(ty_str.clone()){
                //     // data.child = Some(Box::new(Self::from_type(&ident.clone().unwrap(), &ty)));                        
                // }else{
                //     panic!("When Field::new(): Unregistered Object type '{}' in field called '{}'", ty_str, field.ident.clone().unwrap())                    
                // }
            
            }
                          
        }else{
            panic!("Unhandled syn::Type '{:?}' when Field::new()", quote!(field.ty))
        }
    }



    pub fn data(&self)->FieldData{
        match self {
            Self::Float(d) |
            Self::Int(d) |
            Self::Bool(d) |
            Self::String(d) |
            Self::Vec(d) |
            Self::Rc(d) |
            Self::Option(d) |
            Self::Object(d) |
            Self::State(d) => d.clone(),
        }
    }

    pub fn api_name(&self)->String{
        let data = self.data();
        match data.api_alias {
            Some(a)=>format!("{}", &a[1..a.len()-1]),
            None => format!("{}", &data.ident.clone().unwrap())
        }
    }

    
    pub fn api_getter(&self, object_name: &syn::Ident)->TokenStream2{
        let fieldname = &self.data().ident.unwrap();

        let api_fieldname = self.api_name();        

        let value_not_available_err = format!("{} called '{{}}' has not been assigned a value for property '{}'", object_name, api_fieldname);
        quote!(
            // Getter by name
            let new_mod = std::rc::Rc::clone(model);  
            let new_state = std::rc::Rc::clone(state);              
            engine.register_get_result(#api_fieldname, move |this: &mut std::rc::Rc<#object_name>| {
                let state_ptr = & *new_state.borrow();            
                match this.#fieldname(state_ptr){
                    Some(v)=> {return Ok(v)},
                    None => {return Err(format!(#value_not_available_err, this.name).into());}
                }
            });        
        )
    }



    pub fn api_setter(&self, object_name: &syn::Ident)->TokenStream2{
        let data = self.data();
        let fieldname = &data.ident.clone().unwrap();
        let api_fieldname = match data.api_alias {
            Some(a)=>format!("{}", &a[1..a.len()-1]),
            None => format!("{}", &fieldname)
        };

        let rust_fn = syn::Ident::new(&format!("set_{}", fieldname), proc_macro2::Span::call_site());
        let index_ident = format!("{}_index", fieldname);
        let index_ident = syn::Ident::new(&index_ident, fieldname.span());
        let value_not_available_err = format!("Property '{}' has not been initialized for {} called '{{}}' ", api_fieldname, object_name);
        quote!(
            // Setter by name
            let new_mod = std::rc::Rc::clone(model);  
            let new_state = std::rc::Rc::clone(state);      
            engine.register_set_result(#api_fieldname, move |this: &mut std::rc::Rc<#object_name>, v: crate::Float | {            
                match this.#index_ident(){
                    Some(_)=>{
                        let state_ptr = &mut *new_state.borrow_mut();
                        this.#rust_fn(state_ptr, v);                
                    },
                    None => {
                        return Err(format!(#value_not_available_err, this.name).into());
                    }
                };
                Ok(())
                
            });  
            
            let new_mod = std::rc::Rc::clone(model);  
            let new_state = std::rc::Rc::clone(state);      
            engine.register_set_result(#api_fieldname, move |this: &mut std::rc::Rc<#object_name>, v: rhai::INT | {            
                match this.#index_ident(){
                    Some(_)=>{
                        let state_ptr = &mut *new_state.borrow_mut();
                        this.#rust_fn(state_ptr, v as crate::Float);                
                    },
                    None => {
                        return Err(format!(#value_not_available_err, this.name).into());
                    }
                };
                Ok(())
                
            });  
        )
    }





    /// Gets the code for initializing the value of a field.
    /// 
    /// `Vec` are initialized as an empty vector; `Options` are 
    /// initialized as None; and all others `type` are initialized
    /// as an `Option<type> = None`
    pub fn get_initialization(&self)-> TokenStream2 {        
        match self {
            Field::Float(d) => {
                let ident = &d.ident;
                let ty = &d.ty;
                quote!( let mut #ident : Option<#ty> = None; )
            },
            Field::Int(d)=>{
                let ident = &d.ident;
                let ty = &d.ty;
                quote!( let mut #ident : Option<#ty> = None; )
            },
            Field::Bool(d)=>{
                let ident = &d.ident;
                let ty = &d.ty;
                quote!( let mut #ident : Option<#ty> = None; )
            },
            Field::String(d)=>{
                let ident = &d.ident;
                let ty = &d.ty;
                quote!( let mut #ident : Option<#ty> = None; )
            },
        
            Field::Vec(d)=>{
                let ident = &d.ident;
                let ty = &d.ty;
                quote!( let mut #ident : #ty = Vec::new(); )
            },
            Field::Rc(d)=>{
                let ident = &d.ident;
                let ty = &d.ty;
                quote!( let mut #ident : Option<#ty> = None; )
            },
            Field::Option(d)=>{
                let ident = &d.ident;
                let ty = &d.ty;
                quote!( let mut #ident : #ty = None; )
            },
            Field::Object(d)=>{
                let ident = &d.ident;
                let ty = &d.ty;
                quote!( let mut #ident : Option<#ty> = None; )
            },
            Field::State(_d)=>{
                panic!("Trying to initialize a State field")
            },
        }
    }


    /// Gets the tokens that will resolve a Scanner token and transform
    /// it into a Rust value. Float, Int, Bool and String will just be 
    /// transformed into their corresponding types, but 
    pub fn get_resolution(&self, value: &syn::Ident, parent_is_rc: bool)->TokenStream2{
        match self {
            Field::Float(_d) => {
                quote!(
                    #value.resolve_as_float()?
                )
            },
            Field::Int(_d)=>{
                quote!(
                    #value.resolve_as_usize()?
                )
            },
            Field::Bool(_d)=>{
                quote!(
                    #value.resolve_as_bool()?
                )
            },
            Field::String(_d)=>{
                quote!(
                    #value.resolve_as_string()?
                )
            },
        
            Field::Vec(d)=>{
                let field_value = syn::Ident::new("field_value", proc_macro2::Span::call_site());
                let child = d.child.clone().unwrap();
                let resolution = child.get_resolution(&field_value, false);
                quote!({
                    let mut ret_vector = Vec::new();
                    // Check that we are opening an array
                    if field_value.token_type != crate::scanner::TokenType::LeftBracket{
                        return Err(crate::scanner::make_error_msg(format!("Expecting '[' to open an array ... found '{}'", field_value.token_type), scanner.line))
                    }  
                    loop {
                        let field_value = scanner.scan_token();
                        match field_value.token_type{
                            crate::scanner::TokenType::EOF => return Err(crate::scanner::make_error_msg(format!("Unexpected end of file, when reading an Array"), scanner.line)),
                            crate::scanner::TokenType::RightBracket => {break},
                            _ => {                            
                                let element = #resolution;
                                ret_vector.push(element);                                             
                            }
                        }
                        // Check the comma
                        let should_be_a_comma = scanner.scan_token();
                        if should_be_a_comma.token_type == crate::scanner::TokenType::RightBracket{
                            break
                        }else if should_be_a_comma.token_type != crate::scanner::TokenType::Comma{
                            return Err(crate::scanner::make_error_msg(format!("Elements in a vector must by divided by commas (i.e., ',')... found {}", should_be_a_comma.token_type), scanner.line))
                        }
                    }
                    // This will remain this size... will it not?
                    ret_vector.shrink_to_fit(); 
                    // return the vector
                    ret_vector              
                })
                    
                
            },
            Field::Rc(d)=>{
                let child = d.child.clone().unwrap();
                let child_resolution = child.get_resolution(value, true);
                if let syn::Type::Path(t) = &child.data().ty {
                    let ty_str = path_to_string(&t.path);                                    
                    if let Some(_location) = crate::object_location(ty_str.clone()){
                        // data.child = Some(Box::new(Self::from_type(&ident.clone().unwrap(), &ty)));                        
                        quote!(
                            #child_resolution // This will come Wrapped                
                        )
                    }else{
                        quote!(
                            { // Wrap in an RC
                                let aux = {#child_resolution};
                                std::rc::Rc::new(aux)
                            }                
                        )
                    }
                }else{
                    panic!("Werid RC Field")
                }
                
                
            },
            Field::Option(d)=>{
                let child = &d.child.clone().unwrap();
                let child_resolution = child.get_resolution(value, false);
                quote!(#child_resolution)
            },
            Field::Object(d)=>{                  
                if let syn::Type::Path(t) = &d.ty {
                    let ty_str = path_to_string(&t.path);                                    
                    if ty_str == "Polygon3D"{
                        resolve_polygon()
                    }else{

                        resolve_other_object(d,  &d.ty, parent_is_rc)
                    }
                }else{
                    panic!("Werid Object Field")
                }                                                       
            },
            Field::State(_d)=>{
                panic!("Trying to resolve a State field")
            },
        }
    }

    /// Gets the tokens that will resolve a Scanner token and transform
    /// it into a Rust value. Float, Int, Bool and String will just be 
    /// transformed into their corresponding types, but 
    pub fn get_verification(&self, object_name: &syn::Ident)->TokenStream2{
        
        let f_ident = self.data().ident.clone().unwrap();
        let err_missing = format!("missing required field '{}' on {}",f_ident, object_name);
        let check_mandatory = quote!(
            let #f_ident = match #f_ident{
                Some(v)=>v,
                None => {                                                    
                    return Err(crate::scanner::make_error_msg(format!(#err_missing), scanner.line))
                }
            };
        );
        match self {
            Field::Float(_) => {
                check_mandatory
            },
            Field::Int(_)=>{
                check_mandatory
            },
            Field::Bool(_d)=>{
                check_mandatory
            },
            Field::String(_d)=>{
                check_mandatory
            },
        
            Field::Vec(d)=>{
                let f_ident = d.ident.clone().unwrap();
                let err_empty = format!("empty field '{}' on {}", f_ident, object_name);
                quote!(
                    if #f_ident.is_empty() {
                        return Err(crate::scanner::make_error_msg(format!(#err_empty), scanner.line))                        
                    }
                )
            },
            Field::Rc(_d)=>{
                check_mandatory
            },
            Field::Option(_d)=>{
                quote!() // No need... this can be emptu
            },
            Field::Object(_d)=>{
                check_mandatory
            },
            Field::State(_d)=>{
                panic!("Trying to get verification for State field")
            },
        }
    }

    pub fn get_documentation_type(&self)-> String {
        match self {
            Field::Float(_) => {
                "number".to_string()
            },
            Field::Int(_)=>{
                "int".to_string()
            },
            Field::Bool(_d)=>{
                "boolean".to_string()
            },
            Field::String(_d)=>{
                "string".to_string()
            },
        
            Field::Vec(d)=>{    
                let child_type = d.child.clone().unwrap().get_documentation_type();
                format!("[{}, ...]", child_type)
            },
            Field::Rc(d)=>{
                d.child.clone().unwrap().get_documentation_type()
            },
            Field::Option(d)=>{
                let child_type = d.child.clone().unwrap().get_documentation_type();
                format!("{}, // optional", child_type)
            },
            Field::Object(d)=>{
                if let syn::Type::Path(t) = &d.ty {
                    format!("{}", path_to_string(&t.path))
                }else{
                    panic!("Weird object when getting docs")
                }
                
            },
            Field::State(_d)=>{
                panic!("Trying to doc-type of State field")
            },
        }
    }

    pub fn get_documentation(&self)-> String {
        let f_ident = self.data().ident.clone().unwrap();
        
        match self {
            Field::Float(_) |                
            Field::Int(_) |
            Field::Bool(_) |
            Field::String(_) |
            Field::Vec(_) |                            
            Field::Rc(_) |
            Field::Object(_) |
            Field::Option(_) =>{
                format!("{} : {}", f_ident, self.get_documentation_type())
            },            
            Field::State(_d)=>{
                panic!("Trying to get verification for State field")
            },
        }

    }

}








fn resolve_other_object(d: &FieldData, objtype: &syn::Type, parent_is_rc: bool)->TokenStream2{        
    // let ty = d.ty.clone();
    // let ty_str = if let syn::Type::Path(t) = &ty {
    //     path_to_string(&t.path)
    // }else{
    //     panic!("Weird thing when resolve_other_object")
    // };
    
    let final_wrapping = if parent_is_rc {
        quote!(std::rc::Rc::new(aux))
    }else{
        quote!(aux)
    };

    let ty_str = match &d.ty {
        syn::Type::Path(p) => path_to_string(&p.path),
        _ => panic!("Expecting Path in resolve_other_object")
    };    
    let incorrect_type_msg = format!("expecting '{}', found '{{}}'", ty_str);

    // This creates a new object from scratch    
    let create_other_object = quote!(
        // We need an identifier or an Enum
        if field_value.token_type == crate::scanner::TokenType::Identifier || field_value.token_type == crate::scanner::TokenType::TokenEnumName {            
            // Check that it is the same kind of object 
            let found_id = std::str::from_utf8(field_value.txt).unwrap();
            if #ty_str != found_id {
                return Err(crate::scanner::make_error_msg(format!(#incorrect_type_msg, found_id), scanner.line))
            }
            // Setup scanner
            scanner.update_start_index();
            let (start,end) = scanner.get_object_slice();            
            let these_bytes = scanner.borrow_slice(start, end);            
            // Scan
            let aux = #objtype::from_bytes(scanner.line, these_bytes, model)?;
            // return Wrapped in Rc, or not...            
            #final_wrapping       
        } else {
            // return error
            return Err(crate::scanner::make_error_msg(format!("Expecting an object name (i.e., String) or another Object... found '{}'",field_value.token_type), scanner.line));
        }
    );

    

    if let syn::Type::Path(p) = &d.ty {
        
        
        if let Some(target_vec) = crate::object_location(ty_str.clone()){
            let vec_ident = syn::Ident::new(target_vec, proc_macro2::Span::call_site());                    
            let err_not_found = format!("{} called '{{}}' not found", path_to_string(&p.path));
            

            quote!(        
                // If we find a String, then we search in the Model
                if field_value.token_type == crate::scanner::TokenType::TokenString {            
                    // We need the name
                    let aux_name = field_value.resolve_as_string()?;
                    // Search by name
                    let aux = {
                        let mut aux = None;
                        for e in &model.#vec_ident {
                            if aux_name.eq(e.name()) {
                                aux = Some(e.clone());
                                break
                            }
                        }
                        aux
                    };  

                    // If not found... complain
                    if aux.is_none(){                        
                        return Err(crate::scanner::make_error_msg(format!(#err_not_found, aux_name), scanner.line))
                    }       
                    // otherwise, return the object itself (unwraped)
                    aux.unwrap()
                }else #create_other_object // If this was not a string, attempt to create the object
            )

        }else {
            // If this object is in the model (i.e., cannot be searched by name), 
            // Create it.

            // This happens for objects that are not stored IN the model, but in other objects (e.g., enums that represent options)
            quote!(#create_other_object)
        }
            
        
    }else{
        dbg!("Error here!");
        unreachable!();
    }
}





fn resolve_polygon()->TokenStream2{

    quote!( {
         let mut the_vector = Vec::new();
         // Check that we are opening an array
         if field_value.token_type != crate::scanner::TokenType::LeftBracket{
             return Err(crate::scanner::make_error_msg(format!("Expecting '[' to open a Polygon... found '{}'", field_value.token_type), scanner.line))
         }  
         // Loop through the whole array
         loop {
             let field_value = scanner.scan_token();
             match field_value.token_type{
                 crate::scanner::TokenType::EOF | crate::scanner::TokenType::RightBrace => return Err(crate::scanner::make_error_msg(format!("Polygon definition does not end."), scanner.line)),
                 crate::scanner::TokenType::RightBracket => {break},
                 _ => {
                     
                     let element = field_value.resolve_as_float()?;
                     the_vector.push(element)
                 }
             }
 
             // Check the comma
             let comma = scanner.scan_token();
             if comma.token_type == crate::scanner::TokenType::RightBracket{
                 break
             }else if comma.token_type != crate::scanner::TokenType::Comma{
                 return Err(crate::scanner::make_error_msg(format!("Elements in a vector must by divided by commas (i.e., ',')... found {}", comma.token_type), scanner.line))
             }
         }
 
         // Check coherent input
         let n_numbers = the_vector.len();
         if  n_numbers%3 != 0 {
             return Err(crate::scanner::make_error_msg(format!("The length of the vector defining a Polygon must be divisible by 3... found {}", n_numbers ), scanner.line))
         }   
         if  n_numbers < 9 {
             return Err(crate::scanner::make_error_msg(format!("The length of the vector defining a Polygon must be at least 9 (three 3D vertices)... found {}", n_numbers ), scanner.line))
         }
 
         // Build the polygon
         let n_pts = n_numbers/3;
         let mut the_loop = Loop3D::new();
         for pt_index in 0..n_pts{
             let first = 3 * pt_index;
             let x = the_vector[first];
             let y = the_vector[first + 1];
             let z = the_vector[first + 2];
             the_loop.push(geometry3d::Point3D::new(x,y,z))?;                        
         }                    
         if let Err(_) = the_loop.close(){
             return Err("It seems that some of the vertices in the surface are collinear. You do not have the minimum of 3 non-collinear vertices".to_string())
         }
         let p = Polygon3D::new(the_loop)?;
         p
 
     })
 }
 
 