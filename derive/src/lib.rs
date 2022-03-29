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
use crate::simulation_state_behaviour::*;


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

fn object_has_api(typename: String)->bool{    
    let typename_bytes = typename.as_bytes();
    match typename_bytes {
        b"Space" |
        b"Surface" |
        b"Fenestration" |
        b"HVAC" |
        b"Luminaire" => true,
        // b"Construction" => false,
        // b"Substance" => false,
        // b"Material" => false,
        // b"Building" => false,
        _ => false
    }
}




mod object;
mod object_struct;
mod field;
mod common_path;
mod object_enum;
mod docs;
mod simulation_state_behaviour;


use object::Object;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use crate::docs::get_docs;

#[proc_macro_derive(StateElements, attributes(personal, operational, physical))]
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






#[proc_macro_derive(ObjectIO)]
pub fn derive_input_output(input: TokenStream) -> TokenStream {

    
    let ast = parse_macro_input!(input as DeriveInput);
    
    let docs = get_docs(&ast.attrs);
    let obj = Object::new(ast.clone(), docs);
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

    // docs
    let docs = obj.gen_docs();
    
    // return
    TokenStream::from(quote!(
        impl #object_name {

                        
            #from_bytes
            
            # docs

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




#[proc_macro_derive(GroupIO)]
pub fn derive_group_input_output(input: TokenStream) -> TokenStream {

    let ast = parse_macro_input!(input as DeriveInput);
    let docs = get_docs(&ast.attrs);
    let obj = Object::new(ast.clone(), docs);
    
    let q = obj.gen_group_behaviour();
    TokenStream::from(q)   
}




#[proc_macro_derive(ObjectAPI, attributes(operational, physical))]
pub fn derive_object_api(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let docs = get_docs(&ast.attrs);
    let obj = Object::new(ast.clone(), docs);
    TokenStream::from(obj.gen_object_api())
}


#[proc_macro_derive(GroupAPI)]
pub fn derive_group_api(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let docs = get_docs(&ast.attrs);
    let obj = Object::new(ast.clone(), docs);
    TokenStream::from(obj.gen_group_api())

}



#[proc_macro_derive(GroupMemberAPI, attributes(operational, physical))]
pub fn derive_group_member_api(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let docs = get_docs(&ast.attrs);
    let obj = Object::new(ast.clone(), docs);
    TokenStream::from(obj.gen_group_member_api())
}





