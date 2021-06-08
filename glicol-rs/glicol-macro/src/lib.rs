use proc_macro::{TokenStream};
use quote::quote;

#[proc_macro]
pub fn make_graph(input: TokenStream) -> TokenStream {

    let code = &input.to_string();
    let sr: usize = 44100;
    // let mut isref = false;
    // let mut inmain = false; // in main is used to tell if a ref is chain name or ref para
    // let mut tokens = Vec::new();
    // let keywords = vec!["sin", "mul", "add"];
    // let mut i = input.into_iter();
    // let mut f = i.next();
    // while f.is_some() {
    //     let name;
    //     let item = f.unwrap();
    //     tokens.push(item.to_string());
    //     let s = tokens.last().unwrap();

    //     if s.contains("~") {
    //         isref = true;
    //         println!("next is ref {:?}", &item);


    //     } else if s.parse::<f32>().is_ok() {
    //         println!("number {:?}", &item);


    //     } else if s == ":" {
    //         name = match isref {
    //             true => format!("~{}", &tokens[tokens.len()-2]),
    //             false => tokens[tokens.len()-2].clone(),              
    //         };
    //         println!("name {:?}", name);


    //     } else if keywords.contains(&s.as_str()) {
    //         println!("keyword {:?}", &item);


    //     } else {
    //         println!("unknown syntax element {:?}", &item);
    //     }


    //     f = i.next();
    // }
    
    TokenStream::from(quote!(
        Engine::new(#sr).set_code(#code)
    ))
}