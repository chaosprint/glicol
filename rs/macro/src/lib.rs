use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};

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
        println!("{}", &item);
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
            println!("raw {:?}", raw); // raw is tokentree
            let procmacrots = TokenStream::from(raw.clone());
            paras = TokenStream2::from(procmacrots);
            // paras = item.replace(&['(', ')'][..], "");
        } else if item.contains("{") {
            println!("raw {:?}", raw); // raw is tokentree
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
    println!("code: {} \n\nnodename: {:?}  \n\nvariable {:?}  \n\nparas {:?} \n\nbehavior {:?}",code, name, variable, paras, behavior);
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