use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use crate::legacy::common::object_location_str;

fn exposed_api_fieldname(field: &syn::Field, alias: &Option<String>)->String{    
    let fieldname = &field.ident.clone().unwrap();    
    match alias {
        Some(a)=>format!("{}", &a[1..a.len()-1]),
        None => format!("{}", fieldname)
    }
}

pub fn register_type(object_name:&syn::Ident)->TokenStream2{

    let name_str = format!("{}",object_name);
    quote!(
        engine.register_type_with_name::<std::rc::Rc<Self>>(#name_str);
    )
}

pub fn register_access_from_model(object_name:&syn::Ident)->TokenStream2{
    let name_str = format!("{}",object_name);
    let name_str_lower = name_str.to_lowercase();
    
    let location_str = object_location_str(format!("{}",object_name)).unwrap();
    let location = syn::Ident::new(location_str, proc_macro2::Span::call_site());

    // Errors
    let not_found_err = format!("Could not find {} '{{}}'", object_name);
    let out_of_bounds_err = format!("Trying to access {} number {{}}... but the last index is {{}}", object_name);
    let negative_index_err = format!("Impossible to get {} using a negative index ({{}} was given)", object_name);
    
    quote!(
        // get by name
        let new_mod = std::rc::Rc::clone(model);  
        let new_state = std::rc::Rc::clone(state);      
        engine.register_result_fn(#name_str_lower, move |name: &str | {
            for s in new_mod.#location.iter(){                
                if s.name == name {                                
                    return Ok(std::rc::Rc::clone(s))
                }
            }
            return Err(format!(#not_found_err, name).into());
        });

        // Get by index
        let new_mod = std::rc::Rc::clone(model);  
        // let new_state = std::rc::Rc::clone(state);      
        engine.register_result_fn(#name_str_lower, move |index: rhai::INT| {

            let len = new_mod.#location.len();
            if index < 0 {
                return Err(format!(#negative_index_err, index).into())
            }
            if index >= len as i64 {
                return Err(format!(#out_of_bounds_err, index, len - 1).into());
            } 
            Ok(std::rc::Rc::clone(&new_mod.#location[index as usize]))
        });

    )
}

pub fn get_api_docs( physical_fields: &Vec<(syn::Field, Option<String>)>, operational_fields: &Vec<(syn::Field, Option<String>)>)-> String {
    
    let mut ret = format!("\n\n## API\n\nThe following properties are available for simulating control algorithms");

    // Open table
    ret = format!("{}\n\n
| Property | Getter | Setter |
|----------|--------|--------|", ret);
    
    let get_row = |field: &syn::Field, alias: &Option<String>, is_physical: bool|{
        let api_fieldname = exposed_api_fieldname(field, alias);
        let mut this_ret = format!("| `{}` | Yes  ", api_fieldname);
        if is_physical {
            this_ret = format!("{} | Research mode |", this_ret);
        }else{
            this_ret = format!("{} | Yes |", this_ret);
        }

        // return
        this_ret
    };

    // Getters
    let proc_doc = |fields : &Vec<(syn::Field, Option<String>)>, is_physical: bool| -> String {
        let mut this_ret = String::new();
        for (field, alias) in fields {                                        
            this_ret = format!("{}\n{}", this_ret, get_row(field, alias, is_physical));                 
        }
        this_ret
    };
   
    ret = format!("{}{}",ret,proc_doc(operational_fields, false));
    ret = format!("{}{}",ret,proc_doc(physical_fields, true));
    
    ret
}


pub fn api_field_setters(object_name:&syn::Ident, physical_fields: &Vec<(syn::Field, Option<String>)>, operational_fields: &Vec<(syn::Field, Option<String>)>)->TokenStream2{

    let mut operationals = quote!();

    for (field, alias) in operational_fields.iter() {
        // Getter
        let setter = api_field_setter(object_name, field, alias);        
        operationals = quote!(
            #operationals

            #setter
        )
    }

    let mut physicals = quote!();

    for (field, alias) in physical_fields.iter() {
        // Getter
        let setter = api_field_setter(object_name, field, alias);        
        physicals = quote!(
            #physicals

            #setter
        )
    }


    // return
    quote!(
        #operationals
        
        if research_mode {
            #physicals
        }
    )
}

fn api_field_setter(object_name:&syn::Ident, field: &syn::Field, alias: &Option<String>)->TokenStream2{
    
    let fieldname = &field.ident.clone().unwrap();
    let api_fieldname = exposed_api_fieldname(field, alias);

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




pub fn api_field_getters(object_name:&syn::Ident, physical_fields: &Vec<(syn::Field, Option<String>)>, operational_fields: &Vec<(syn::Field, Option<String>)>)->TokenStream2{

    let mut ret = quote!();

    for (field, alias) in operational_fields.iter() {
        // Getter
        let getter = api_field_getter(object_name, field, alias);        
        ret = quote!(
            #ret

            #getter
        )
    }

    for (field, alias) in physical_fields.iter() {
        // Getter
        let getter = api_field_getter(object_name, field, alias);        
        ret = quote!(
            #ret

            #getter
        )
    }


    // return
    ret
}

fn api_field_getter(object_name:&syn::Ident, field: &syn::Field, alias: &Option<String>)->TokenStream2{
    
    let fieldname = &field.ident.clone().unwrap();    
    let api_fieldname = exposed_api_fieldname(field, alias);
    

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

pub fn classify_api_fields(fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>)->(Vec<(syn::Field, Option<String>)>, Vec<(syn::Field, Option<String>)>){    
    let mut physical_fields:Vec<(syn::Field, Option<String>)> = Vec::new();
    let mut operational_fields:Vec<(syn::Field, Option<String>)> = Vec::new();
    
    fn get_alias(attr: &syn::Attribute)->Option<String>{
        let mut alias : Option<String> = None;
        attr.tokens.clone().into_iter().for_each(|token|{
            if let proc_macro2::TokenTree::Group(g) = token {                            
                g.stream().into_iter().for_each(|literal|{
                    if let proc_macro2::TokenTree::Literal(lit) = literal{                        
                        alias = Some(format!("{}", lit));
                    }
                })
                
            }
        });
        alias
    }

    // Classify fields
    for f in fields.iter(){                                
        if let syn::Type::Path(_) = &f.ty {
            let this_field = f.clone();
            
            for attr in this_field.attrs.iter(){
                let name = format!("{}",attr.path.segments[0].ident);                
                let alias = get_alias(attr);
                
                if name == "physical" {
                    physical_fields.push((this_field, alias));  
                    break;                  
                    
                }else if name == "operational"{
                    operational_fields.push((this_field, alias));
                    break;
                }                
            }
             
                         
        }else{
            panic!("Unknonwn syn::Type")
        }
    }
    (physical_fields, operational_fields)
}