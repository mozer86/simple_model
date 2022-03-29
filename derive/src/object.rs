// use proc_macro::TokenStream;
// use syn::{parse_macro_input, DeriveInput};
use crate::object_enum::EnumObject;
use crate::object_struct::StructObject;
use proc_macro2::TokenStream as TokenStream2;

use quote::*;

/// A SIMPLE object
pub enum Object {
    Enum(EnumObject),
    StructObject(StructObject),
}

impl Object {
    pub fn new(ast: syn::DeriveInput, docs: String) -> Self {
        match ast.data {
            syn::Data::Struct(stru) => {
                let data = StructObject::new(ast.ident.clone(), stru, docs);
                Object::StructObject(data)
            }
            syn::Data::Enum(stru) => {
                let data = EnumObject::new(ast.ident.clone(), stru, docs);
                Object::Enum(data)
            }
            _ => {
                panic!("SIMPLE MAcros only work for Enums and StructObjects");
            }
        }
    }

    pub fn gen_name(&self) -> TokenStream2 {
        match self {
            Self::StructObject(_s) => {
                quote!(
                    /// Borrows the name
                    pub fn name(&self) -> &String {
                        &self.name
                    }
                )
            }
            Self::Enum(_) => quote!(), // don't have names
        }
    }

    pub fn gen_from_bytes(&self) -> TokenStream2 {
        match self {
            Self::StructObject(s) => s.gen_from_bytes(),
            Self::Enum(s) => s.gen_from_bytes(),
        }
    }

    pub fn gen_new(&self) -> TokenStream2 {
        match self {
            Self::StructObject(s) => s.gen_new(),
            Self::Enum(_s) => quote!(), // don't have these
        }
    }

    pub fn gen_state_getters_setters(&self) -> TokenStream2 {
        match self {
            Self::StructObject(s) => s.gen_state_getters_setters(),
            Self::Enum(_s) => quote!(), // don't have these
        }
    }

    pub fn gen_docs(&self) -> TokenStream2 {
        let (ident, docs) = match self {
            Self::StructObject(s) => (s.ident.clone(), s.gen_docs()),
            Self::Enum(s) => (s.ident.clone(), s.gen_docs()),
        };
        let name_str = format!("{}", ident);
        quote!(
            #[cfg(debug_assertions)]
            pub fn print_doc(dir: &str, summary: &mut String)->std::io::Result<()>{
                let doc = #docs.as_bytes();
                let filename = format!("{}.md", #name_str).to_lowercase();
                let full_filename = format!("{}/{}", dir, filename);
                summary.push_str(&format!("- [{}](./{})\n",#name_str, filename));
                std::fs::write(&full_filename, doc)?;
                Ok(())
            }
        )
    }

    pub fn gen_group_behaviour(&self) -> TokenStream2 {
        match self {
            Self::Enum(s) => s.gen_group_behaviour(),
            Self::StructObject(_) => panic!("Group of objects needs to be an Enum"),
        }
    }

    pub fn gen_group_api(&self) -> TokenStream2 {
        match self {
            Self::Enum(s) => s.gen_group_api(),
            Self::StructObject(_) => panic!("Group of objects needs to be an Enum"),
        }
    }

    pub fn gen_group_member_api(&self) -> TokenStream2 {
        match self {
            Self::Enum(_s) => {
                panic!("Enums are not yet supported as members of a group")
            }
            Self::StructObject(s) => s.gen_group_member_api(),
        }
    }

    pub fn gen_object_api(&self) -> TokenStream2 {
        match self {
            Self::Enum(_s) => {
                panic!("API does not yet support Enums")
            }
            Self::StructObject(s) => s.gen_object_api(),
        }
    }
}
