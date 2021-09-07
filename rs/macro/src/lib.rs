use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use std::collections::HashMap;

#[proc_macro]
pub fn make_graph(input: TokenStream) -> TokenStream {
    // let code = &input.to_string();
    // let mut N: usize = 64;
    let mut code: String = "".to_owned();
    let mut variable = vec![];
    
    let mut i = input.into_iter();
    let mut f = i.next();
    while f.is_some() {
        let item = f.unwrap().to_string();
        
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
            // i.next();
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
        SimpleGraph::<N>::new(format!(#code, #(#variable),*).as_str())
    );
    o.into()
}


#[proc_macro]
pub fn make_node(input: TokenStream) -> TokenStream {
    // let code = &input.to_string();
    // let mut N: usize = 64;
    let mut code: String = "".to_owned();
    let mut name = Ident::new("A", Span::call_site());
    let mut macroname = Ident::new("a", Span::call_site());
    let mut variable = vec![];
    let mut paras = TokenStream2::new();
    let mut behavior = TokenStream2::new();
    
    let mut i = input.into_iter();
    let mut f = i.next();
    while f.is_some() {
        let raw = f.clone().unwrap();
        let item = f.unwrap().to_string();
        // println!("{}", &item);
        if &item == "#" {
            code.push_str("{}");
            code.push_str(" ");
            f = i.next();
            variable.push(Ident::new(&f.unwrap().to_string(), Span::call_site()));
        } else if &item == "@" {
            f = i.next();
            // println!("hi {}", &f.unwrap());
            let namestr = &f.unwrap().to_string();
            name = Ident::new(namestr, Span::call_site());
            macroname = Ident::new(&namestr.to_lowercase(), Span::call_site());

        } else if item.contains("(") {
            // println!("raw {:?}", raw); // raw is tokentree
            let procmacrots = TokenStream::from(raw.clone());
            paras = TokenStream2::from(procmacrots);
            // paras = item.replace(&['(', ')'][..], "");
        } else if item.contains("{") {
            // println!("raw {:?}", raw); // raw is tokentree
            // let procmacrots = TokenStream::from(raw.clone());
            behavior = match raw {
                TokenTree::Group(g) => TokenStream2::from(g.stream()),
                _ => unimplemented!()
            }
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
            // i.next();
        } else {
            code.push_str(&item);
            code.push_str(" ");
        }
        f = i.next();
    }
    // println!("code: {} \n\nnodename: {:?}  \n\nvariable {:?}  \n\nparas {:?} \n\nbehavior {:?}",code, name, variable, paras, behavior);
    // // let code = "num is {}";
    let o = quote!(
        // use glicol_macro::*;
        use glicol_synth::{SimpleGraph, mono_node, GlicolNodeData};
        use glicol_parser::{Rule, GlicolParser};
        use pest::Parser;
        use dasp_graph::{Buffer, Input, Node, NodeData, BoxedNodeSend};

        pub struct #name<const N: usize> {
            graph: SimpleGraph<N>
        }
        
        impl<const N: usize> #name<N> {
            pub fn new #paras -> GlicolNodeData<N> {
                #behavior
                let graph = SimpleGraph::<N>::new(format!(#code, #(#variable),*).as_str());
                mono_node!( N, Self { graph } )
            }
        }

        impl<const N: usize> Node<N> for #name<N> {
            fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {       
                let mut input = [0.0; N];
                for i in 0..N {
                    input[i] = inputs[0].buffers()[0][i];
                }
                // println!("inputs {:?}", input);
                let out = self.graph.next_block(&mut input);
                for i in 0..N {
                    output[0][i] = out[i];
                    // output[1][i] = out[i+N];
                }
                // println!("out {:?}", out);
            }
        }
        
        #[macro_export]
        macro_rules! #macroname{
            ($size: expr => $data: expr) => {
                #name::<$size>::new($data)
            };
        }
    );
    o.into()
}

#[proc_macro]
pub fn register_extensions(input: TokenStream) -> TokenStream {
    println!("register input {:?}", input);
    let mut stream = input.into_iter();
    let mut token = stream.next();
    // let mut lib = HashMap::<String, u8>::new();
    let mut key_up = vec![];
    let mut key_low_str = vec![];
    let mut key_low = vec![];
    let mut para_num = vec![];
    // let mut key = "".to_owned();
    while token.is_some() {
        match token.unwrap() {
            TokenTree::Group(_) => {},
            TokenTree::Ident(item) => { 
                key_up.push( item.clone() ); 
                key_low_str.push(item.clone().to_string().to_lowercase());
                key_low.push( Ident::new(&item.to_string().to_lowercase(), Span::call_site()) );
                
            },
            TokenTree::Punct(_) => { },
            TokenTree::Literal(item) => {
                para_num.push(item.to_string().parse::<u8>().unwrap()); 
            },
        }
        token = stream.next();
    };
    let o = quote!(
        #( pub mod #key_low ; )*
        #( use #key_low :: * ; )*
        pub fn make_node_ext<const N: usize>(
            name: &str,
            paras: &mut Pairs<Rule>,
            pos: (usize, usize),
            samples_dict: &HashMap<String, &'static[f32]>,
            sr: usize,
            bpm: f32,
        ) -> Option<GlicolNodeData<N>> {
            let n = match name {
                #( #key_low_str => #para_num,  )*
                _ => return None
            };
            let mut pv = vec![];
            for _ in 0..n {
                // let mut v;
                let mut p = match paras.next() {
                    Some(v) => v.as_str(),
                    None => return None
                };
                // no modulation here so far
                match p.parse::<f32>() {
                    Ok(v) => pv.push(v),
                    Err(_) => return None
                };
            };
            let node = match name {
                // only one para is supported
                #( #key_low_str => #key_low!(N => pv[0]), )*
                _ => unimplemented!()
            };
            
            Some(node)
        }
    );
    println!("oooo {:?}", o.to_string());
    o.into()
}