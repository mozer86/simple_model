//! # SIMPLE model derive macro
//! 
//! The final behaviour of this macro should be relatively simple.
//! On the one hand, all SIMPLE-model objects—enums or structs—should be 
//! * Representable in text format (i.e., readable by the scanner)
//! * Have a function that creates the documentation (used only for building automatic documentation)
//! 
//! On the other hand, the SimulationState elements should be classifiable as either
//! * `operational` (is a window open?), `physical` (e.g., solar radiation over a wall) or `personal` (e.g., the amount of clothing weared by a person)
//! 
//! This is handled by several macros.
//! 
//! # Deriving Struct behaviour:
//! 
//! There are two main kinds of fields in structs: `Optional` and `Mandatory` 

use std::collections::HashMap;





fn object_location(typename: String)->Option<&'static str>{
    let mapping = HashMap::from([
        ("Substance", "substances"),
        ("Material", "materials"),
        ("Construction", "constructions"),        
        ("Surface", "surfaces"),
        ("Space", "spaces"), 
        ("Building", "buildings"),
        ("Fenestration", "fenestrations"),
        ("HVAC", "hvacs"),
        ("Luminaire", "luminaires"),
    ]);

    if let Some(v) = mapping.get( &typename.as_str() ){
        Some(v)
    }else{
        None
    }

}




mod object;
mod object_struct;
mod field;
mod common_path;
mod object_enum;

use object::Object;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(SimpleInputOutput)]
pub fn derive_input_output(input: TokenStream) -> TokenStream {

    
    let ast = parse_macro_input!(input as DeriveInput);
    
    let obj = Object::new(ast.clone());
    let object_name = &ast.ident;
    let name_str = format!("{}",object_name);



    // From Bytes    
    let from_bytes = obj.gen_from_bytes();

    // New
    let new = obj.gen_new();

    // name
    let name = obj.gen_name();

    // State getters and setters
    let state_getters_setters = obj.gen_state_getters_setters();
    
    // return
    TokenStream::from(quote!(
        impl #object_name {

                        
            #from_bytes
            
            // # docs

            /// Retrieves the type of object as a `&'static str`. 
            /// 
            /// This method is useful for debuging models that contain multiple objects
            pub fn object_type(&self) -> &str{
                #name_str
            }

            #name

            #new
            
            #state_getters_setters


            
        }
    ))
}











/* LEGACY */
mod legacy;
use crate::legacy::object_api::*;
use crate::legacy::common::*;
use crate::legacy::enum_parser::*;
use crate::legacy::object_parser::*;
use crate::legacy::simulation_state::*;

#[proc_macro_derive(GroupSimpleRhaiAPI)]
pub fn derive_group_api(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let object_name = &ast.ident;
    let name_str = format!("{}",object_name);
    let name_str_lower = name_str.to_lowercase();

    let location_str = object_location_str(format!("{}",object_name)).unwrap();
    let location = syn::Ident::new(location_str, proc_macro2::Span::call_site());

    let not_found_err = format!("Could not find {} '{{}}'", object_name);
    let negative_index_err = format!("Impossible to get {} using a negative index ({{}} was given)", object_name);
    let out_of_bounds_err = format!("Trying to access {} number {{}}... but the last index is {{}}", object_name);
    if let syn::Data::Enum(_data) = &ast.data{

        let variants = get_enum_variants(&ast);  
        let mut name_match_statement = quote!();
        let mut index_match_statement = quote!();
        for v in variants.iter(){
            let v_ident = v.ident.clone();

            name_match_statement = quote!(
                #name_match_statement

                #object_name::#v_ident(s)=>{
                    if s.name == name {
                        let d = rhai::Dynamic::from(std::rc::Rc::clone(s));
                        return Ok(d)
                    }
                }
            ); 

            index_match_statement = quote!(
                #index_match_statement

                #object_name::#v_ident(s)=>{
                    let d = rhai::Dynamic::from(std::rc::Rc::clone(s));
                    return Ok(d)                    
                }

            )
        }      
        
        TokenStream::from(quote!(
            impl #object_name {
                pub fn register_api(engine : &mut rhai::Engine, model: &std::rc::Rc<SimpleModel>, state: &std::rc::Rc<std::cell::RefCell<crate::SimulationState>>, research_mode: bool){

                    // By name
                    let new_mod = std::rc::Rc::clone(model);                      
                    engine.register_result_fn(#name_str_lower, move |name: &str | {

                        for s in new_mod.#location.iter(){  
                            match s {
                                #name_match_statement
                            }                      
                        }
                        return Err(format!(#not_found_err, name).into());
                    });

                    // By index
                    let new_mod = std::rc::Rc::clone(model);                      
                    engine.register_result_fn(#name_str_lower, move |index: rhai::INT| {

                        let len = new_mod.#location.len();
                        if index < 0 {
                            return Err(format!(#negative_index_err, index).into())
                        }
                        if index >= len as i64 {
                            return Err(format!(#out_of_bounds_err, index, len - 1).into());
                        } 
                        match &new_mod.#location[index as usize]{
                            #index_match_statement
                        }
                        
                    });

                }
            }
        ))
        

    } else{
        panic!("GroupSimpleRhaiApi ::: can only be derived for Enums");
    }
}

#[proc_macro_derive(GroupMemberSimpleRhaiAPI, attributes(operational, physical))]
pub fn derive_group_member_api(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let object_name = &ast.ident;
    let name_str = format!("{}",object_name);

    if let syn::Data::Struct(stru) = &ast.data{


        let fields = get_object_fields(&stru);
        // Classify fields according to their function
        let (physical_fields, operational_fields) = classify_api_fields(&fields);
        
        let register_type = register_type(object_name);
        // let access_from_model = register_access_from_model(object_name);

        let field_getters = api_field_getters(&object_name, &physical_fields, &operational_fields);
        let field_setters = api_field_setters(&object_name, &physical_fields, &operational_fields);
        let api_doc_fn = get_api_docs( &physical_fields, &operational_fields);
        
        TokenStream::from(quote!(
            impl #object_name {
                                
                pub fn register_api(engine : &mut rhai::Engine, model: &std::rc::Rc<SimpleModel>, state: &std::rc::Rc<std::cell::RefCell<crate::SimulationState>>, research_mode: bool){
                    
                    #register_type
                             
                    // #access_from_model

                    #field_getters

                    #field_setters
                }
                
                                    
                #[cfg(debug_assertions)]
                pub fn print_api_doc(dir: &str, summary: &mut String)->std::io::Result<()>{
                    let api_doc = #api_doc_fn;
                    let filename = format!("{}.md", #name_str).to_lowercase();
                    let full_filename = format!("{}/{}", dir, filename);                        

                    let doc = std::fs::read_to_string(full_filename.clone())
                        .expect("Something went wrong reading the documentation file");
                                                                                               
                    std::fs::write(&full_filename, format!("{}\n\n{}", doc, api_doc))?;

                    Ok(())
                }
            
            }
        ))

    } else{
        panic!("GroupMemberSimpleRhaiApi ::: can only be derived for Structs");
    }
}


#[proc_macro_derive(SimpleRhaiAPI, attributes(operational, physical))]
pub fn derive_object_api(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let object_name = &ast.ident;
    let name_str = format!("{}",object_name);

    if let syn::Data::Struct(stru) = &ast.data{


        let fields = get_object_fields(&stru);
        // Classify fields according to their function
        let (physical_fields, operational_fields) = classify_api_fields(&fields);
        
        let register_type = register_type(object_name);
        let access_from_model = register_access_from_model(object_name);

        let field_getters = api_field_getters(&object_name, &physical_fields, &operational_fields);
        let field_setters = api_field_setters(&object_name, &physical_fields, &operational_fields);
        let api_doc_fn = get_api_docs( &physical_fields, &operational_fields);
        
        TokenStream::from(quote!(
            impl #object_name {
                                
                pub fn register_api(engine : &mut rhai::Engine, model: &std::rc::Rc<SimpleModel>, state: &std::rc::Rc<std::cell::RefCell<SimulationState>>, research_mode: bool){
                    
                    #register_type
                             
                    #access_from_model

                    #field_getters

                    #field_setters
                }
                
                                    
                #[cfg(debug_assertions)]
                pub fn print_api_doc(dir: &str, summary: &mut String)->std::io::Result<()>{
                    let api_doc = #api_doc_fn;
                    let filename = format!("{}.md", #name_str).to_lowercase();
                    let full_filename = format!("{}/{}", dir, filename);                        

                    let doc = std::fs::read_to_string(full_filename.clone())
                        .expect("Something went wrong reading the documentation file");
                                                                                               
                    std::fs::write(&full_filename, format!("{}\n\n{}", doc, api_doc))?;

                    Ok(())
                }
            
            }
        ))

    } else{
        panic!("SimpleAPI ::: can only be derived for Structs");
    }
}

#[proc_macro_derive(SimpleGroupInputOutput)]
pub fn derive_group_input_output(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let object_docs = get_docs(&ast.attrs);
    let object_name = &ast.ident;
    // let object_name_str = format!("{}", object_name);
    // let trait_name = syn::Ident::new(&trait_name, proc_macro2::Span::call_site());
    // let trait_name_str = format!("{}", trait_name);
    let err_colon_colon = format!("Expecting '::' after group name '{}'", object_name);
    
    let mut from_bytes = quote!();
    let mut name_fn = quote!();

    // let mut sub_docs = quote!();
    let mut doc_string = String::new();    
    if let syn::Data::Enum(_) = &ast.data{
        let variants = get_enum_variants(&ast);
        doc_string.push_str(&format!("# {}\n\n{}\n\n", object_name, object_docs));
        
        for variant in variants{
            let ident = variant.ident;
            let ident_str = format!("{}", ident);
            let variant_docs = get_docs(&variant.attrs);

            name_fn = quote!(
                #name_fn

                #object_name::#ident(o) => {
                    &o.name
                }
            );

            // Extend from_bytes() match statement
            from_bytes = quote!(
                #from_bytes

                #ident_str => {
                    // println!("Variant is {}", #ident);
                    let ret = #ident::from_bytes(scanner.line, slice, model)?;
                    Ok(Self::#ident(std::rc::Rc::new(ret)))
                }
            );

            // Extend docs
            doc_string.push_str(&format!("* **{}**: {}\n", ident, variant_docs));

            
        }
        // Api access.
        let object_name_str = format!("{}", object_name);
        if object_has_api(object_name_str.clone()){
            let name_str_lower = object_name_str.to_lowercase();
            doc_string += &format!("\n\n## API Access\n\n```rs\n// by name\nlet my_{} = {}(string);\n// by index\nlet my_{} = {}(int);\n```", name_str_lower, name_str_lower, name_str_lower, name_str_lower)
        }

        // From bytes
        from_bytes = quote!(
            pub fn from_bytes(line: usize, bytes: &[u8], model: &SimpleModel)->Result<#object_name, String>{
                let mut scanner = crate::scanner::SimpleScanner::new(bytes, line);
                let colon_colon = scanner.scan_token();
                if colon_colon.token_type != crate::scanner::TokenType::ColonColon{
                    return Err(crate::scanner::make_error_msg(format!(#err_colon_colon), scanner.line));
                }
                let kind = scanner.scan_token();
                let kind = std::str::from_utf8(kind.txt).unwrap();
                scanner.update_start_index();
                let (start,end) = scanner.get_object_slice();
                let slice = scanner.borrow_slice(start, end);  
                match kind{
                    #from_bytes
                    _ => {
                        Err(crate::scanner::make_error_msg(format!("Unknown kind of HVAC '{}'", kind), scanner.line))
                    }
                }
            }
        );

        name_fn = quote!(
            /// Borrows the name
            pub fn name(&self)->&String{
                match self{
                    #name_fn
                }
            }            
        );

        let print_doc = quote!(
            
            #[cfg(debug_assertions)]
            pub fn print_doc(dir: &str, summary: &mut String)->std::io::Result<()>{
                let doc = #doc_string.as_bytes() ;
                
                let filename = format!("{}.md", #object_name_str).to_lowercase();
                let full_filename = format!("{}/{}", dir, filename);                                        
                summary.push_str(&format!("- [{}](./{})\n",#object_name_str, filename));

                std::fs::write(&full_filename, doc)?;
                Ok(())

            }
        );

        TokenStream::from(quote!(
            impl #object_name {

                #from_bytes

                #name_fn

                #print_doc
            }
        ))

    }else{
        panic!("Group of objects needs to be an Enum")
    }
}


#[proc_macro_derive(SimpleSimulationStateBehaviour, attributes(personal, operational, physical))]
pub fn derive_simulation_state_behaviour(input: TokenStream) -> TokenStream {
    let mut out = input.clone();

    let ast = parse_macro_input!(input as DeriveInput);
    let enum_name = &ast.ident;
    match ast.data {        
        syn::Data::Enum(_)=>{
            let variants = get_enum_variants(&ast);

          
            let derive_kind_variants = match derive_enum_kind(&ast, &variants){
                Ok(s)=>s,
                Err(e)=>{                    
                    out.extend(TokenStream::from(e.to_compile_error()));
                    return out
                }
            };
        
            // Gather everything
            TokenStream::from(quote!(
                impl #enum_name {

                    
                    #derive_kind_variants
                }
            ))
        },
        _ => {
            panic!("SimulationStateBehaviour ::: can only be derived for Enums");
        }
    }

}

