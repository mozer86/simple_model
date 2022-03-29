

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

