use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use crate::field::Field;

#[derive(Clone)]
pub struct VariantData {
    pub ident: syn::Ident,
    pub attributes: Vec<String>,
    pub fields: Vec<Field>,
    pub docs: String,
    // pub ty : syn::Type,
}

#[derive(Clone)]
enum Variant {
    Unit(VariantData),
    Unnamed(VariantData),
    // Named // unimplemented!()
}

impl Variant {
    fn data(&self) -> VariantData {
        match self {
            Self::Unit(d) => d.clone(),
            Self::Unnamed(d) => d.clone(),
        }
    }

    pub fn new(variant: syn::Variant) -> Self {
        let ident = variant.ident.clone();

        let attributes: Vec<String> = variant
            .attrs
            .iter()
            .map(|a| format!("{}", a.path.segments[0].ident))
            .collect();

        let docs = crate::docs::get_docs(&variant.attrs);

        let mut data = VariantData {
            ident,
            attributes,
            fields: Vec::new(),
            docs,
        };
        match &variant.fields {
            syn::Fields::Unit => Self::Unit(data),
            syn::Fields::Unnamed(fields) => {
                data.fields = fields
                    .unnamed
                    .iter()
                    .map(|x| Field::new(x.clone()))
                    .collect();
                Self::Unnamed(data)
            }
            syn::Fields::Named(_named) => {
                panic!("syn::Fields::Named have not been implemented");
            }
        }
    }

    fn resolve(&self, object_name: &syn::Ident) -> TokenStream2 {
        let self_ident = self.data().ident;
        let self_ident_bytes = format!("{}", self_ident);
        match self {
            Self::Unit(_) => {
                // If it is unnamed (e.g., Boundary::Ground) then just return it
                quote!(
                    #self_ident_bytes => {
                        #object_name::#self_ident
                    },
                )
            }
            Self::Unnamed(data) => {
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
                for (i, f) in data.fields.iter().enumerate() {
                    let this_resol = f.get_resolution(&field_value, false);
                    let this_iden =
                        syn::Ident::new(&format!("field{}", i), proc_macro2::Span::call_site());
                    in_match = quote!(
                        #in_match

                        let field_value = scanner.scan_token();
                        let #this_iden = #this_resol;
                    );
                    if i == 0 {
                        fieldlist = quote!(#this_iden);
                    } else {
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
    pub ident: syn::Ident,
    variants: Vec<Variant>,
    docs: String,
}

impl EnumObject {
    pub fn new(ident: syn::Ident, stru: syn::DataEnum, docs: String) -> Self {
        let variants: Vec<Variant> = stru
            .variants
            .iter()
            .map(|x| Variant::new(x.clone()))
            .collect();

        Self {
            variants,
            ident,
            docs,
        }
    }

    pub fn gen_from_bytes(&self) -> TokenStream2 {
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
            let this_resolve = var.resolve(object_name);
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

        let from_bytes_docstring = format!(
            " Creates a new [`{}`] by scanning bytes (i.e., a `&[u8]`) acquired from a text file",
            object_name
        );
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

    pub fn gen_docs(&self) -> String {
        let mut ret = String::new();

        // // Title
        ret += &format!("# {}\n\n", &self.ident);

        // Written doc
        ret += &format!("{}\n\n", &self.docs);

        // Document variants
        for variant in self.variants.iter() {
            ret += &format!("## {}\n\n", &variant.data().ident);

            match variant {
                Variant::Unnamed(s) => {
                    let v_ident = &s.ident;
                    ret += &format!("```rs\n{}::{}(", &self.ident, v_ident);
                    let mut i = 0;
                    for field in s.fields.iter() {
                        if let Field::State(_) = field {
                            continue;
                        }
                        if i != 0 {
                            ret += ","
                        }

                        ret += &field.get_documentation_type().to_string();
                        i += 1;
                    }
                    ret += ")\n```\n\n";
                }
                Variant::Unit(s) => {
                    let v_ident = &s.ident;
                    ret += &format!("```rs\n{}::{}()\n```\n\n", &self.ident, v_ident);
                }
            }
            ret += &format!("{}\n\n", &variant.data().docs);
        }

        // Api access.
        let object_name_str = format!("{}", &self.ident);
        if crate::object_has_api(object_name_str.clone()) {
            let name_str_lower = object_name_str.to_lowercase();
            ret += &format!("\n\n## API Access\n\n```rs\n// by name\n let my_{} = {}(string);\n// by index\nlet my_{} = {}(int);```", name_str_lower, name_str_lower, name_str_lower, name_str_lower)
        }

        ret
    }

    pub fn gen_group_behaviour(&self) -> TokenStream2 {
        let object_name = &self.ident;
        let object_docs = &self.docs;
        let err_colon_colon = format!("Expecting '::' after group name '{}'", object_name);

        let mut from_bytes = quote!();
        let mut name_fn = quote!();

        let mut doc_string = format!("# {}\n\n{}\n\n", object_name, object_docs);

        for variant in self.variants.iter() {
            let ident = variant.data().ident;
            let ident_str = format!("{}", ident);
            let variant_docs = &variant.data().docs;

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
        if crate::object_has_api(object_name_str.clone()) {
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

                let filename = format!("auto-{}.md", #object_name_str).to_lowercase();
                let full_filename = format!("{}/{}", dir, filename);
                summary.push_str(&format!("- [{}](./{})\n",#object_name_str, filename));

                std::fs::write(&full_filename, doc)?;
                Ok(())

            }
        );

        quote!(
            impl #object_name {

                #from_bytes

                #name_fn

                #print_doc
            }
        )
    }

    pub fn gen_group_api(&self) -> TokenStream2 {
        let object_name = self.ident.clone();
        let name_str = format!("{}", &object_name);
        let name_str_lower = name_str.to_lowercase();

        let location_str = crate::object_location(name_str).unwrap_or_else(|| {
            panic!(
                "Trying to gen a Group API of '{}', which is not registered in Model",
                &object_name
            )
        });
        let location = syn::Ident::new(location_str, proc_macro2::Span::call_site());
        let not_found_err = format!("Could not find {} '{{}}'", &object_name);
        let negative_index_err = format!(
            "Impossible to get {} using a negative index ({{}} was given)",
            &object_name
        );
        let out_of_bounds_err = format!(
            "Trying to access {} number {{}}... but the last index is {{}}",
            &object_name
        );

        let mut name_match_statement = quote!();
        let mut index_match_statement = quote!();

        for v in self.variants.iter() {
            let v_ident = v.data().ident.clone();

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

        quote!(
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
        )
    }
}
