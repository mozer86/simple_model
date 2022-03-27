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


pub fn get_enum_variants(ast: &syn::DeriveInput)->syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>{
    if let syn::Data::Enum(syn::DataEnum {
        ref variants,
        ..
    }) = ast.data
    {
        variants.clone()
    } else {
        panic!("THIS IS A BUG: Expecting an Enum");
    }
        
}

pub fn contains_attr(v:&syn::Variant, att: &str)->bool{
    let att_names : Vec<String> = v.attrs.iter().map(|a|{
        format!("{}",a.path.segments[0].ident)
    }).collect();
    att_names.contains(&att.to_string())
}