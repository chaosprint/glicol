use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2, TokenTree as TokenTree2};

#[proc_macro]
pub fn def_node(all_defs: TokenStream) -> TokenStream {
    let mut this_node_name = "".to_owned();
    let mut names_all = vec![];
    let mut args_all = vec![];
    let mut paras_all = vec![];
    let mut variables = Vec::<String>::new(); // variables example: vec![ TokenStreams(a=a, b=b), TokenStreams(att=att) ]
    let mut variables_all = Vec::<TokenStream2>::new();
    let mut graph_all = vec![];
    // let mut name = Ident::new("A", Span::call_site());
    // let mut macroname = Ident::new("a", Span::call_site());
    // let mut paras = TokenStream2::new();
    // let mut varname = vec![];
    // let mut variable = vec![];
    // let mut behavior = TokenStream2::new();
    
    let mut all_defs_iter = all_defs.into_iter();
    let defs_group = all_defs_iter.next().unwrap();
    let defs_stream = match defs_group {
        TokenTree::Group(g) => TokenStream2::from(g.stream()),
        _ => panic!("not a token group")
    };

    let mut defs_stream_iter = defs_stream.into_iter();
    let mut def_next = defs_stream_iter.next();
    while def_next.is_some() {
        let raw = def_next.clone().unwrap();
        let item = def_next.unwrap().to_string();
        // println!("item {}", &item);
        if item.contains("\"") {
            this_node_name = item.clone().replace("\"", "");
            names_all.push(item.clone().replace("\"", ""));
            // println!("\n\n\n this node names is {:?}", &this_node_name);
            defs_stream_iter.next();
        } else if item.contains("{"){
            let info = match raw {
                TokenTree2::Group(g) => TokenStream2::from(g.stream()),
                _ => panic!("not a token group")
            };
            let mut info_inner = info.into_iter();
            let mut i = info_inner.next();
            let mut count = 0;
            while i.is_some() {
                let id = i.clone().unwrap().to_string();
                if id == "args".to_owned() {
                    info_inner.next();
                    i = info_inner.next();
                    args_all.push(i.clone().unwrap());
                    variables.push("".to_owned());
                    count = variables.len();
                } else if id == "paras".to_owned() {
                    info_inner.next();
                    i = info_inner.next();
                    // println!("\n\nparas as strtng{:?} \n\n", i.clone().unwrap().to_string());
                    let paras_str = match i.clone().unwrap() {
                        TokenTree2::Group(g) => TokenStream2::from(g.stream()),
                        _ => panic!("not a token group")
                    };
                    paras_all.push(paras_str)
                } else if id == "graph".to_owned() {
                    info_inner.next();
                    i = info_inner.next();
                    let graph = i.clone();
                    let mut s = "".to_owned();

                    let graph_code_str = match graph.unwrap() {
                        TokenTree2::Group(g) => TokenStream2::from(g.stream()),
                        _ => panic!("not a token group")
                    };
                
                    let mut graph_iter = graph_code_str.into_iter();
                    let mut ele = graph_iter.next();
                    println!("ele {:?}", ele.clone());
                    while ele.is_some() {
                        let raw = ele.unwrap();
                        let ele_str = raw.clone().to_string();
                        if ele_str == "#" {
                            s.push_str("{");
                            let v = graph_iter.next().unwrap();
                            println!("count {}", count);
                            if variables[count-1].find(&format!(",{}={}",&v.to_string(),&v.to_string())).is_none() {
                                variables[count-1].push_str(&format!(",{}={}",&v.to_string(),&v.to_string()));
                            }                            
                            // let current_token_stream = variables[count];
                            s.push_str(&v.to_string());
                            s.push_str("}");
                            s.push_str(" ");
                        } else if ele_str == "~" {
                            s.push_str(&ele_str);
                            ele = graph_iter.next();
                            let next = ele.unwrap().to_string();
                            // println!("next ele is {}", &next);
                            if &next == "input" {
                                // println!("found input ********************");
                                s.push_str(&next);
                                s.push_str(" ");
                            } else {
                                let q = this_node_name.clone();
                                s.push_str(&q);
                                s.push_str(&next);
                                s.push_str("chain_name");
                                s.push_str(" ");
                            }
                        } else if ele_str == "-" {
                            s.push_str(&ele_str);
                        } else if ele_str == ";" {
                            s.push_str(&ele_str);
                            s.push_str("\n");
                        } else if ele_str == ">" {
                            graph_iter.next();
                            s.push_str(">> ");
                        } else {                      
                            s.push_str(&ele_str);
                            s.push_str(" ");
                        }
                        ele = graph_iter.next();
                    }
                    graph_all.push(s)
                } else {
                    i = info_inner.next();
                }
            }
        } else {

        }
        println!("names_all {:?}\n\n", names_all);
        // println!("{:?}\n\n", args_all);
        // println!("{:?}\n\n", paras_all);
        variables_all = variables.clone().into_iter().map(|x|x.parse().unwrap()).collect();
        println!("variables {:?}\n\n", variables);
        // println!(" variables_all {:?}\n\n", variables_all );
        // println!("{:?}\n\n", graph_all);
        // let raw = f.clone().unwrap();
        // let item = f.unwrap().to_string();
        // if item.contains("{") & !item.contains(":") {
        //     println!("raw {:?}", raw); // raw is tokentree
        //     // let procmacrots = TokenStream::from(raw.clone());
        //     behavior = match raw {
        //         TokenTree::Group(g) => TokenStream2::from(g.stream()),
        //         _ => unimplemented!()
        //     };
        // } else if item.contains("{") & item.contains(":") { // this block is glicol syntax
        //     let glicol_code = match raw {
        //         TokenTree::Group(g) => TokenStream2::from(g.stream()),
        //         _ => unimplemented!()
        //     };
        //     // println!("glicol item is {:?}", glicol_code.clone());
        //     let mut glicol_code_iter = glicol_code.into_iter();
        //     let mut element = glicol_code_iter.next();
        //     // to calculate the args
        //     // to get the output chains into reference and added
        //     while element.is_some() {
        //         let item = element.unwrap().to_string();
        //         if &item == "#" {
        //             element = glicol_code_iter.next();
        //             let var = element.clone().unwrap().to_string();
                    
        //             code.push_str("{");
        //             code.push_str(&var);
        //             code.push_str("}");
        //             code.push_str(" ");
        //             if !varname.contains(&var) {
        //                 varname.push(var.clone());
        //                 variable.push(Ident::new(&element.unwrap().to_string(), Span::call_site()));
        //             }
        //         }
        //         element = glicol_code_iter.next();
        //     }
        // } else {

        // }
        def_next = defs_stream_iter.next();
    }
    let o = quote!(

        pub fn get_args(paras: &mut Pairs<Rule>, modulation_info: Vec<Para>) -> Vec<String> {
            let mut result = vec![];
            for info in modulation_info {
                let s = paras.as_str().to_owned();
                let para = paras.next();
                if para.is_none() {
                    panic!(s); 
                    // insufficient para
                }
                match info {
                    Para::Modulable => {
                        // check if it is a mod?
                        // can be a number
                        let p = para.unwrap();
                        if !p.as_str().contains("~") {
                            // reture Err
                        }
                        result.push(p.as_str().to_owned())
                    },
                    Para::Number(v) => {
                        // must be number
                        let p = para.unwrap();
                        if !p.as_str().parse::<f32>().is_ok() {
                            // return Err()
                        } else {
                            result.push(p.as_str().to_owned())
                        }
                    },
                    _ => {}
                }
            }
            result
        }
        
        // paras: &mut Pairs<Rule>, 

        pub fn findname(name: &str, paras: &mut Pairs<Rule>) -> Vec<String> {
            let target_paras = match name {
                #( #names_all => {
                    vec!#args_all
                }, )*
                _ => { unimplemented!("no such a name...") }
            };
            get_args(paras, target_paras)
        }

        pub fn preprocess2(mut code: &mut String) -> Result<String, GlicolError> {
            let mut target_code = code.clone();
            let lines = match GlicolParser::parse(Rule::block, &mut code) {
                Ok(mut res) => {
                    if res.as_str() < &mut target_code {
                        return Err(GlicolError::ParsingIncompleteError(res.as_str().len()))
                        // unimplemented!("half parsing {}", res.as_str());
                    }
                    res.next().unwrap()
                },
                Err(e) => { 
                    return Err(GlicolError::ParsingError(e))
                    // unimplemented!("parsing error {:?} code: {}", e, target_code)
                }
            };
            let mut processed_code = "".to_owned();
            let mut appendix_full = "".to_owned();
            let mut current_ref_name = "".to_owned();
        
            for line in lines.into_inner() {
                let inner_rules = line.into_inner();
                for element in inner_rules {
                    match element.as_rule() {
                        Rule::reference => {
                            current_ref_name = element.as_str().to_owned();
                            processed_code.push_str("\n");
                            processed_code.push_str(&current_ref_name);
                            processed_code.push_str(": ");
                            // println!("current_ref_name {:?}", current_ref_name);
                        },
                        Rule::chain => {
                            let mut node_str_list = vec![];
                            for node in element.into_inner() {
                                let mut name_and_paras = node.into_inner();
                                let name_and_paras_str: String = name_and_paras.as_str().to_string();
                                let node_name = name_and_paras.next().unwrap();
                                let name = node_name.as_str().clone();
                                let mut paras = name_and_paras.clone(); // the name is ripped aboves
                                if vec![#(#names_all,)*].contains(&name) {
                                    // panic!(paras.as_str().to_owned());
                                    let args = findname(&name, &mut paras);
                                    // panic!(name.to_owned());
                                    let appendix_body = match name {
                                        #( #names_all => {
                                            #paras_all
                                            // #graph_all
                                            // #( #(#variables = #variables),* ),*
                                            // panic!(attack, decay);
                                            // #(#variables = #variables ),*
                                            format!(#graph_all #variables_all)
                                        }, )*
                                        _ => { unimplemented!() }
                                    };
                                    
                                    // let appendix_body = match name {
                                    //     "sawsynth" => "output: saw ~pitch >> mul ~env;~trigger: ~input;~pitch: ~trigger >> mul 261.626;~env: ~trigger >> envperc 0.001 0.1;",
                                    //     _ => {unimplemented!()}
                                    // };
        
                                    let mut appendix = appendix_body.replace("~input", &node_str_list.join(" >> "));
                                    let mut ref_receiver = "~".to_owned();
                                    ref_receiver.push_str(&current_ref_name.replace("~", ""));
                                    ref_receiver.push_str("_");
                                    ref_receiver.push_str(name);
                                    let mut appendix = appendix.replace("output", &ref_receiver);
                                    // println!("current ref {}", &current_ref_name);
                                    let mut appendix = appendix.replace("chain_name", &current_ref_name.replace("~", "tilde"));
                                    appendix_full.push_str(&appendix);
                                    appendix_full.push_str("\n\n");
                                    node_str_list = vec![ref_receiver];
                                } else {
                                    let mut s = name.to_owned();
                                    s.push_str(" ");
                                    s.push_str(paras.as_str());
                                    node_str_list.push(s);
                                };
                            }
                            processed_code.push_str(&node_str_list.join(" >> "));
                            processed_code.push_str("\n\n");
                            processed_code.push_str(&appendix_full);
                        },
                        _ => {}
                    }
                };
                
            };
            Ok(processed_code)
        }

        pub fn preprocessor(chain_name:&str, node_name: &str, paras: &mut Pairs<Rule>, source: Vec<String>) -> Result<(String, String, Vec<String>), GlicolError> {
            
            let mut inplace_code = String::new();
            let mut appendix_code = String::new();
            let mut to_sink = vec![];
        
            let (target_paras, mut inplace, mut appendix) = match node_name {
                // "mul" => (vec![1.0], "mul ~mulmod_CHAIN_NAME", "~mulmod_CHAIN_NAME: const_sig PARA_0"),
                "bd" => (vec![0.3], "sin ~pitchCHAIN_NAME >> mul ~envbCHAIN_NAME >> mul 0.8", "~envbCHAIN_NAME: ~triggerbCHAIN_NAME >> envperc 0.01 PARA_0;~env_pitchCHAIN_NAME: ~triggerbCHAIN_NAME >> envperc 0.01 0.1;~pitchCHAIN_NAME: ~env_pitchCHAIN_NAME >> mul 50 >> add 60;~triggerbCHAIN_NAME: SOURCE;"),
                _ => {
                    inplace_code = node_name.to_owned();
                    inplace_code.push_str(" ");
                    inplace_code.push_str(paras.as_str());
                    println!("inplace_code {}", inplace_code);
                    to_sink = source;
                    to_sink.push(inplace_code.clone());
                    return Ok((inplace_code, appendix_code, to_sink));
                    (vec![], "", "")
                }
            };
        
            inplace_code = inplace.replace("CHAIN_NAME", &chain_name);
            appendix_code = appendix.replace("CHAIN_NAME", &chain_name);
            appendix_code = appendix_code.replace("SOURCE", &source.join(" >> "));
        
            for i in 0..target_paras.len() {
                match paras.next() {
                    Some(v) => {
                        let p = process_para(target_paras[i], v.as_str())?;
                        let current_para = format!("PARA_{}", i);
                        inplace_code = inplace_code.replace(&current_para, &format!("{}", p) );
                        appendix_code = appendix_code.replace(&current_para, &format!("{}", p) );
                    }
                    None => { return Err(GlicolError::InsufficientParameter((0,0))) }
                }
            }
            // panic!(appendix_code);
        
            match node_name {
                "bd" => {
                    to_sink = inplace_code.split(">>").map(|a|a.to_owned()).collect()
                },
                _ => {}
            }
        
            Ok( (inplace_code, appendix_code, to_sink) )
        }
        
        fn process_para(default: f32, input: &str) -> Result<String, GlicolError> {
            if input == "_" {
                return Ok(format!("{}", default))
            } else if input.parse::<f32>().is_ok() {
                return Ok(input.to_owned())
            } else {
                return Ok(input.to_owned())
            }
        }
        
    );
    o.into()
}


/// This is just a proof of concept
#[proc_macro]
pub fn make_node(input: TokenStream) -> TokenStream {
    // let code = &input.to_string();
    // let mut N: usize = 64;
    let mut code: String = "".to_owned();
    let mut name = Ident::new("A", Span::call_site());
    let mut macroname = Ident::new("a", Span::call_site());
    let mut varname = vec![];
    let mut variable = vec![];
    // let mut paras = TokenStream2::new();
    let mut behavior = TokenStream2::new();
    
    let mut i = input.into_iter();
    let mut f = i.next();
    while f.is_some() {
        let raw = f.clone().unwrap();
        let item = f.unwrap().to_string();
        // println!("{}", &item);
        if &item == "#" {
            f = i.next();
            let var = f.clone().unwrap().to_string();
            
            code.push_str("{");
            code.push_str(&var);
            code.push_str("}");
            code.push_str(" ");
            if !varname.contains(&var) {
                varname.push(var.clone());
                variable.push(Ident::new(&f.unwrap().to_string(), Span::call_site()));
            }
        } else if &item == "@" {
            f = i.next();
            // println!("hi {}", &f.unwrap());
            let namestr = &f.unwrap().to_string();
            name = Ident::new(namestr, Span::call_site());
            macroname = Ident::new(&namestr.to_lowercase(), Span::call_site());

        // } else if item.contains("(") {
        //     // println!("raw {:?}", raw); // raw is tokentree
        //     let procmacrots = TokenStream::from(raw.clone());
        //     paras = TokenStream2::from(procmacrots);
        //     // paras = item.replace(&['(', ')'][..], "");
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
    let o = quote!(

        pub struct #name<const N: usize> {
            graph: SimpleGraph<N>
        }
        
        impl<const N: usize> #name<N> {
            pub fn new(args: Vec<f32>) -> GlicolNodeData<N> {
                #behavior
                let graph = SimpleGraph::<N>::new(format!(#code, #(#variable = #variable),*).as_str());
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
        
        // this is just ok for one-parameter node
        #[macro_export]
        macro_rules! #macroname{
            ($size: expr => $para:expr) => {
                #name::<$size>::new($para)
            };
        }
    );
    o.into()
}

#[proc_macro]
pub fn register_extensions(input: TokenStream) -> TokenStream {
    // println!("register input {:?}", input);
    let mut stream = input.into_iter();
    let mut token = stream.next();
    // let mut lib = HashMap::<String, u8>::new();
    let mut key_up = vec![];
    let mut key_low_str = vec![];
    let mut key_low = vec![];
    let mut para_num = vec![];
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
                let n = item.clone().to_string().parse::<u8>().unwrap();
                para_num.push(n);
            },
        }
        token = stream.next();
    };
    let o = quote!(
        pub mod nodes;
        use nodes::*;

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

            // if paras.as_str() == "_" {
            //     let node = match name {
            //         #( #key_low_str => #key_low!( N => args ), )*
            //         _ => unimplemented!()
            //     };
            // }

            let mut args: Vec<f32> = paras.as_str().split(" ").filter(|c| c != &"").map(|x|x.parse::<f32>().unwrap()).collect();
            // println!("args {:?}", args);
            // assert_eq!(args.len(), n as usize);
            let node = match name {
                #( #key_low_str => #key_low!( N => args ), )*
                _ => unimplemented!()
            };
            
            Some(node)
        }
    );
    // println!("o into {:?}", o.to_string());
    o.into()
}