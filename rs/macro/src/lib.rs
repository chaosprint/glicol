use proc_macro::{TokenStream};
use quote::quote;
use proc_macro2::{Ident, Span};

#[proc_macro]
pub fn make_graph(input: TokenStream) -> TokenStream {
    // let code = &input.to_string();
    let mut code: String = "".to_owned();
    let mut variable = vec![];
    
    let mut i = input.into_iter();
    let mut f = i.next();
    while f.is_some() {
        let item = f.unwrap().to_string();
        // println!("{}", item);
        if &item == "#" {
            code.push_str("{}");
            code.push_str(" ");
            f = i.next();
            variable.push(Ident::new(&f.unwrap().to_string(), Span::call_site()));
        } else if &item == ">" {
            code.push_str(&item);
            code.push_str(&item);
            i.next();
        } else if &item == "~" {
            code.push_str(&item);
            f = i.next();
            code.push_str(&f.unwrap().to_string());
            code.push_str(" ");
        } else if &item == "-" {
            code.push_str(&item);
            f = i.next();
            code.push_str(&f.unwrap().to_string());
        } else {
            code.push_str(&item);
            code.push_str(" ");
        }
        f = i.next();
    }
    // println!("{} {:?}",code, variable);
    // // let code = "num is {}";
    let o = quote!(
        // println!(#code, #(#variable),*);
        SimpleGraph::new(format!(#code, #(#variable),*).as_str())
    );
    o.into()
}