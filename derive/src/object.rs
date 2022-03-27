// use proc_macro::TokenStream;
// use syn::{parse_macro_input, DeriveInput};
use crate::object_struct::StructObject;
use crate::object_enum::EnumObject;
use proc_macro2::TokenStream as TokenStream2;

use quote::*;




/// A SIMPLE object
pub enum Object {
    Enum(EnumObject),
    StructObject(StructObject),
}




impl Object {
    pub fn new(ast: syn::DeriveInput)->Self{
        
        
        match ast.data {        
            syn::Data::Struct(stru) => {
                let data = StructObject::new(ast.ident.clone(), stru);
                Object::StructObject(data)
            },
            syn::Data::Enum(stru)=>{
                let data = EnumObject::new(ast.ident.clone(), stru);
                Object::Enum(data)
            },
            _ => {
                panic!("SIMPLE MAcros only work for Enums and StructObjects");
            }
        }

    }

    pub fn gen_name(&self)->TokenStream2{
        match self{
            Self::StructObject(_s)=>{
                quote!(
                    /// Borrows the name
                    pub fn name(&self)->&String{
                        &self.name
                    }
                )
            },
            Self::Enum(_)=>quote!() // don't have names
        }
    }

    pub fn gen_from_bytes(&self)-> TokenStream2 {
        match self{
            Self::StructObject(s)=>s.gen_from_bytes(),
            Self::Enum(s) => s.gen_from_bytes()
        }
    }

    pub fn gen_new(&self)-> TokenStream2 {
        match self{
            Self::StructObject(s)=>{
                s.gen_new()
            },
            Self::Enum(_s) => quote!() // don't have these
        }
    }

    pub fn gen_state_getters_setters(&self)-> TokenStream2{
        match self{
            Self::StructObject(s)=>{
                s.gen_state_getters_setters()
            },
            Self::Enum(_s) => quote!() // don't have these
        }
    }
}