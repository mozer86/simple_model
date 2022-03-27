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

use crate::legacy::common::*;

pub fn get_object_fields(stru: &syn::DataStruct) -> syn::punctuated::Punctuated<syn::Field, syn::token::Comma> {
    if let syn::DataStruct{fields:syn::Fields::Named(syn::FieldsNamed{ ref named, ..}), ..} = stru {
        named.clone()
    }else{
        panic!("#name ::: weird");
    }
}

#[allow(dead_code)]
pub fn classify_fields(fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>)->(Vec<syn::Field>, Vec<syn::Field>, Vec<syn::Field>){
    let mut required_fields:Vec<syn::Field> = Vec::new();
    let mut optional_fields:Vec<syn::Field> = Vec::new();
    let mut state_fields:Vec<syn::Field> = Vec::new();
    
    // Classify fields
    for f in fields.iter(){                                
        if let syn::Type::Path(t) = &f.ty {
            let this_field = f.clone();
            let att_names : Vec<String> = this_field.attrs.iter().map(|a|{
                format!("{}",a.path.segments[0].ident)
            }).collect();

            // let first_segment = &t.path.segments[0];
            if att_names.contains(&"state".to_string()){
                state_fields.push(this_field);
            }else if path_is_option(&t.path) {
                optional_fields.push(this_field);
            }else{
                required_fields.push(this_field);
            }                    
        }else{
            panic!("Unknonwn syn::Type")
        }
    }
    (required_fields, optional_fields, state_fields)
}