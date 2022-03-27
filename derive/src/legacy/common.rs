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



#[allow(dead_code)]
pub fn path_to_string(path: &syn::Path)->String{    
    format!("{}", path.segments.iter().next().unwrap().ident)
}

#[allow(dead_code)]
pub fn path_doc_name(path : &syn::Path)->String{
    match path_to_string(path).as_bytes(){
        b"f64" | b"f32" | b"Float" => "number".to_string(),
        b"usize" => "integer".to_string(),
        b"String"=>"string".to_string(),
        _ => format!("{}", path_to_string(path))
    }
}

#[allow(dead_code)]
pub fn get_docs(attrs: &std::vec::Vec<syn::Attribute>) -> String {
    let mut ret = String::new();

    for at in attrs {
        if let Some(segment) = at.path.segments.iter().next(){
            let segment_ident = format!("{}",segment.ident);
            if "doc" == segment_ident {
                let mut doc = format!("{}",at.tokens.clone());
                // Get rid of the annoying '=' and '"'
                doc.remove(0);
                doc.remove(1);
                doc.remove(doc.len()-1);                
                                
                let doc = doc.replace("\\\\", "\\");
                let doc = doc.replace("\\\"", "\"");

                ret.push_str(&format!("{}\n",doc));
            }
        }
    }

    ret
}


#[allow(dead_code)]
pub fn object_location(path: &syn::Path)->Result<&'static str,String>{
    let typename = path_to_string(path);
    object_location_str(typename)
}

#[allow(dead_code)]
pub fn object_location_str(typename: String)->Result<&'static str,String>{    
    let typename_bytes = typename.as_bytes();
    match typename_bytes {
        b"Space" => Ok("spaces"),
        b"Substance" => Ok("substances"),
        b"Surface" => Ok("surfaces"),
        b"Fenestration" => Ok("fenestrations"),
        b"Material" => Ok("materials"),
        b"Construction" => Ok("constructions"),
        b"HVAC" => Ok("hvacs"),
        b"Luminaire" => Ok("luminaires"),
        b"Building" => Ok("buildings"),
        _ => Err(format!("Unexpected type '{}' when object_location_str()", typename))
    }
}


#[allow(dead_code)]
pub fn object_has_api(typename: String)->bool{    
    let typename_bytes = typename.as_bytes();
    match typename_bytes {
        b"Space" => true,
        b"Substance" => false,
        b"Surface" => true,
        b"Fenestration" => true,
        b"Material" => false,
        b"Construction" => false,
        b"HVAC" => true,
        b"Luminaire" => true,
        b"Building" => false,
        _ => false
    }
}



#[allow(dead_code)]
pub fn get_path_from_type(ty:&syn::Type)->Result<syn::Path,String>{
    if let syn::Type::Path(t) = ty {
        Ok(t.path.clone())
    }else{
        Err("Object is not a path".to_string())
    }
}

#[allow(dead_code)]
pub fn get_path_from_traitobject(ty:&syn::Type)->Result<syn::Path,String>{
    if let syn::Type::TraitObject(t) = ty {
        let parambounds = t.bounds.iter().next().clone().unwrap();
            if let syn::TypeParamBound::Trait(tr) = parambounds{
                // panic!("asd");
                Ok(tr.path.clone())
            }else{
                Err(format!("Not Trait but LifetEime!"))
            }    
    }else{
        Err("Object is not a TraitObject".to_string())
    }
}



// https://stackoverflow.com/questions/55271857/how-can-i-get-the-t-from-an-optiont-when-using-syn
#[allow(dead_code)]
pub fn path_is_option(path: &syn::Path) -> bool {    
    // ty.leading_colon.is_none()
    path.segments.len() == 1
    && path_to_string(path) == "Option"
}

#[allow(dead_code)]
pub fn path_is_vec(path: &syn::Path) -> bool {    
    // ty.leading_colon.is_none()
    path.segments.len() == 1
    && path_to_string(path) == "Vec"
}

#[allow(dead_code)]
pub fn path_is_rc(path: &syn::Path) -> bool {    
    // ty.leading_colon.is_none()
    path.segments.len() == 1
    && path_to_string(path) == "Rc"
}

#[allow(dead_code)]
pub fn path_is_float(path: &syn::Path) -> bool {    
    // ty.leading_colon.is_none()
    path.segments.len() == 1
    && (path_to_string(path) == "Float" || path_to_string(path) == "f32" || path_to_string(path) == "Float")
}
#[allow(dead_code)]
pub fn path_is_usize(path: &syn::Path) -> bool {    
    // ty.leading_colon.is_none()
    path.segments.len() == 1
    && path_to_string(path) == "usize"
}

#[allow(dead_code)]
pub fn path_is_string(path: &syn::Path) -> bool {    
    path.segments.len() == 1
    && path.segments.iter().next().unwrap().ident == "String"
}

#[allow(dead_code)]
pub fn path_is_other_object(path:&syn::Path)->bool{
    !path_is_float(path) &&
    !path_is_string(path) &&
    !path_is_rc(path) &&
    !path_is_vec(path) &&
    !path_is_usize(path) &&
    !path_is_option(path)
}



// https://stackoverflow.com/questions/55271857/how-can-i-get-the-t-from-an-optiont-when-using-syn
#[allow(dead_code)]
pub fn extract_type_from_option(ty: &syn::Type) -> Result<syn::Type,String> {
    
    match ty {
        syn::Type::Path(typepath) /*if typepath.qself.is_none() && path_is_option(typepath.path) */=> {
            extract_type_from_path(&typepath.path)
        }
        _ => Err(format!("TODO: error handling 3")),
    }
}

// https://stackoverflow.com/questions/55271857/how-can-i-get-the-t-from-an-optiont-when-using-syn
#[allow(dead_code)]
pub fn extract_type_from_path(path: &syn::Path) -> Result<syn::Type,String> {
    
    // Get the first segment of the path (there is only one, in fact: "Option"):
    let type_params = &path.segments[0].arguments;//.iter().first().unwrap().arguments;
    // It should have only on angle-bracketed param ("<String>"):
    let generic_arg = match type_params {
        syn::PathArguments::AngleBracketed(params) => &params.args[0],//.iter().first().unwrap(),
        _ => return Err(format!("TODO: error handling 1... found {}", path_to_string(path))),
    };
    // This argument must be a type:
    match generic_arg {
        syn::GenericArgument::Type(ty) => Ok(ty.clone()),
        _ => return Err(format!("TODO: error handling 2... found {}", path_to_string(path)))
    }
}