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

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::spanned::Spanned;

pub fn get_enum_variants(
    ast: &syn::DeriveInput,
) -> syn::punctuated::Punctuated<syn::Variant, syn::token::Comma> {
    if let syn::Data::Enum(syn::DataEnum { ref variants, .. }) = ast.data {
        variants.clone()
    } else {
        panic!("THIS IS A BUG: Expecting an Enum");
    }
}

pub fn contains_attr(v: &syn::Variant, att: &str) -> bool {     
        v.attrs
        .iter()
        .map(|a| format!("{}", a.path.segments[0].ident)).any(|x| x == *att)        
}

pub fn derive_enum_kind(
    ast: &syn::DeriveInput,
    variants: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
) -> Result<TokenStream2, syn::Error> {
    let enum_name = &ast.ident;

    // Check that each variant has one and only one category
    for v in variants {
        let att_names: Vec<String> = v
            .attrs
            .iter()
            .map(|a| format!("{}", a.path.segments[0].ident))
            .collect();
        let mut n = 0;
        if att_names.contains(&"physical".to_string()) {
            n += 1;
        }
        if att_names.contains(&"personal".to_string()) {
            n += 1;
        }
        if att_names.contains(&"operational".to_string()) {
            n += 1;
        }
        if n == 0 {
            return Err(syn::Error::new(v.span(),format!("Variant {} has no attributes... must have either 'physical', 'personal' or 'operational'", v.ident)));
        }
        if n > 1 {
            return Err(syn::Error::new(v.span(),format!("Variant {} has too many attributes... it must have ONLY ONE between 'physical', 'personal' or 'operational'", v.ident)));
        }
    }

    // Sort variants
    let each_physical_variant = variants
        .iter()
        .filter(|v| contains_attr(v, "physical"))
        .map(|x| {
            let v_ident = x.ident.clone();
            quote! {
                Self::#v_ident{..}
            }
        });

    let each_personal_variant = variants
        .iter()
        .filter(|v| contains_attr(v, "personal"))
        .map(|x| {
            let v_ident = x.ident.clone();
            quote! {
                Self::#v_ident{..}
            }
        });
    let each_operational_variant = variants
        .iter()
        .filter(|v| contains_attr(v, "operational"))
        .map(|x| {
            let v_ident = x.ident.clone();
            quote! {
                Self::#v_ident{..}
            }
        });

    let is_personal_docstring =
        format!(" Checks whether a [`{}`] is of kind `Personal`", enum_name);
    let is_physical_docstring =
        format!(" Checks whether a [`{}`] is of kind `Physical`", enum_name);
    let is_operational_docstring = format!(
        " Checks whether a [`{}`] is of kind `Operational`",
        enum_name
    );

    Ok(quote!(
        // impl #enum_name {
            #[doc = #is_physical_docstring]
            pub fn is_physical (self: &'_ Self) -> ::core::primitive::bool{
                match self {
                    #(
                        | #each_physical_variant => true,
                    )*
                    | _ => false,

                }
            }

            #[doc = #is_personal_docstring]
            pub fn is_personal (self: &'_ Self) -> ::core::primitive::bool{
                match self {
                    #(
                        | #each_personal_variant => true,
                    )*
                    | _ => false,

                }
            }

            #[doc = #is_operational_docstring]
            pub fn is_operational (self: &'_ Self) -> ::core::primitive::bool{
                match self {
                    #(
                        | #each_operational_variant => true,
                    )*
                    | _ => false,

                }
            }
        // }
    ))
}
