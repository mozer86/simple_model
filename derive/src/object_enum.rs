
use quote::quote;
use proc_macro2::TokenStream as TokenStream2;

use crate::field::Field;

#[derive(Clone)]
pub struct VariantData {
    pub ident: syn::Ident,
    pub attributes: Vec<String>,
    pub fields : Vec<Field>,
    // pub ty : syn::Type,
}

#[derive(Clone)]
enum Variant {
    Unit(VariantData),
    Unnamed(VariantData),
    // Named // unimplemented!()
}

impl Variant{

    fn data(&self)->VariantData{
        match self{
            Self::Unit(d)=>d.clone(),
            Self::Unnamed(d)=>d.clone(),
        }
    }

    pub fn new(variant: syn::Variant)-> Self {

        let ident = variant.ident.clone();

        let attributes : Vec<String> = variant.attrs.iter().map(|a|{
            format!("{}",a.path.segments[0].ident)
        }).collect();

        let mut data = VariantData{
            ident,
            attributes,
            fields: Vec::new()
        };
        match &variant.fields {
            syn::Fields::Unit => {
                Self::Unit(data)
            },
            syn::Fields::Unnamed(fields)=>{
                data.fields = fields.unnamed.iter().map(|x| Field::new(x.clone()) ).collect();
                Self::Unnamed(data)
            },
            syn::Fields::Named(_named)=>{
                panic!("syn::Fields::Named have not been implemented");
            }
        }
    }


    fn resolve(&self, object_name: &syn::Ident)->TokenStream2{
        let self_ident = self.data().ident.clone();
        let self_ident_bytes = format!("{}",self_ident);
        match self {
            Self::Unit(_) => {
                // If it is unnamed (e.g., Boundary::Ground) then just return it
                quote!(                    
                    #self_ident_bytes => {                        
                        #object_name::#self_ident
                    },
                )
            },
            Self::Unnamed(data)=>{
                // If it contains data, then we should expect a left parenthesis
                let mut in_match = quote!(                        
                    let left_paren = scanner.scan_token();
                    if left_paren.token_type != crate::scanner::TokenType::LeftParen{
                        return Err(crate::scanner::make_error_msg(format!("Expecting '(' when opening fields in Enum... found '{}'", left_paren.token_type), scanner.line));
                    }
                );
                // resolve each field
                let mut fieldlist = quote!();
                let field_value = syn::Ident::new("field_value", proc_macro2::Span::call_site());
                for (i,f) in data.fields.iter().enumerate(){
                    let this_resol = f.get_resolution(&field_value, false);
                    let this_iden = syn::Ident::new(&format!("field{}", i), proc_macro2::Span::call_site());
                    in_match = quote!(
                        #in_match

                        let field_value = scanner.scan_token();
                        let #this_iden = #this_resol;                        
                    );
                    if i == 0 {
                        fieldlist = quote!(#this_iden);
                    }else{
                        fieldlist = quote!(#fieldlist,#this_iden);
                    }
                }
                // Consume final token
                in_match = quote!(
                    #in_match

                    if scanner.scan_token().token_type != crate::scanner::TokenType::RightParen{
                        return Err(format!("Expecting ')' when closing fields in Enum"));
                    }

                    #object_name::#self_ident(#fieldlist)

                );

                // return
                quote!(                    
                    #self_ident_bytes => {                                                                            
                        #in_match
                    },
                )
            }

        }
    }
}








pub struct EnumObject {
    ident:  syn::Ident,
    variants : Vec<Variant>, 
}

impl EnumObject{
    pub fn new(ident: syn::Ident, stru: syn::DataEnum)->Self{
        
        let variants : Vec<Variant> = stru.variants.iter().map(|x| Variant::new(x.clone())).collect();
        
        Self{
            variants,
            ident
        }
    }


    pub fn gen_from_bytes(&self)-> TokenStream2 {
        
        
        let ret = quote!(        
            let mut scanner = crate::scanner::SimpleScanner::new(bytes, line);
            let colon_colon = scanner.scan_token();
            if colon_colon.token_type != crate::scanner::TokenType::ColonColon{
                return Err(crate::scanner::make_error_msg(format!(" Expecting '::' after EnumName, found {} '{}'", colon_colon.token_type, std::str::from_utf8(colon_colon.txt).unwrap()), scanner.line))
            }        
            let obj_name = scanner.scan_token();
            if obj_name.token_type != crate::scanner::TokenType::Identifier{
                return Err(crate::scanner::make_error_msg(format!(" Expecting 'Identifier' after '::', found {} '{}'", obj_name.token_type, std::str::from_utf8(obj_name.txt).unwrap()), scanner.line))
            }
        );
        
        let object_name = &self.ident;
        
        let mut match_statement = quote!();
        let unexpected_field_err = format!("Object '{{}}' is not in enum '{}'", object_name);
        
        
        for var in &self.variants {
            
            let this_resolve = var.resolve(&object_name);
            match_statement = quote!(
                #match_statement

                #this_resolve
            )
        }

        match_statement = quote!(
            #match_statement
            _ => {
                let obj_name = std::str::from_utf8(obj_name.txt).unwrap();
                return Err(crate::scanner::make_error_msg(format!(#unexpected_field_err, obj_name), scanner.line))
            },
        );
    
        let from_bytes_docstring=format!(" Creates a new [`{}`] by scanning bytes (i.e., a `&[u8]`) acquired from a text file", object_name);
        quote!(
            #[doc = #from_bytes_docstring] 
            pub fn from_bytes(line: usize, bytes: &[u8], model: &crate::model::SimpleModel)->Result<Self, String>{
                #ret
    
                let obj_name_str = std::str::from_utf8(obj_name.txt).unwrap();
                let obj = match obj_name_str {
                    #match_statement
                };
                Ok(obj)
            }
        )
        
    }
}







