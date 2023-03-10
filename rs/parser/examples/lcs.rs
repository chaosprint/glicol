use glicol_parser::*;
use pest::Parser;
use pest::error::ErrorVariant;
// use lcs_diff::*;

// fn main() {
//     let a = [1, 3, 5];
//     let b = [3, 4, 5];
//     // println!("result {:?}", diff(&a, &b));
//     let result = diff(&a, &b);
//     for r in result {
//         println!("result {:?}", r);
//     }
// }

fn main() {
    let mut block = match GlicolParser::parse(Rule::block, r#"out: sin 440 >> mul 0.1 >> add 0.1; b: seq 60 _60 >> sp \808;"#) {
        Ok(v) => v,
        Err(e) => {
            // println!("in location: {:?}; line_col: {:?}", e.location, e.line_col);

            match e.variant {
                ErrorVariant::ParsingError{ positives, negatives } => { 

                    if positives.len() != 0 {
                        print!("\n\nexpecting ");
                        for possible in positives { print!("{:?} ", possible) }
                        print!("\n\n");
                    }
                    if negatives.len() != 0 {
                        print!("\n\nunexpected element: ");
                        for possible in negatives { print!("{:?} ", possible) }
                        print!("\n\n");
                    }
                    panic!();
                }
                _ => {panic!("unknonw parsing error")}
            }
            
        }
    };
    let lines = block.next().unwrap(); // this can be a comment though, but we call it a line
    let mut ast = std::collections::HashMap::new();
    for line in lines.into_inner() {
        match line.as_rule() {
            Rule::line => {
                println!("each line {:?}", line.as_str());
                let mut key = "";
                let mut chain_node_names = vec![];
                let mut chain_paras = vec![];
                for line_component in line.into_inner() {
                    match line_component.as_rule() {
                        Rule::reference => {
                            println!("ref {:?}", line_component.as_str());
                            key = line_component.as_str();
                        },
                        Rule::chain => {
                            println!("chain {:?}", line_component.as_str());
                            let chain = line_component;
                            for node_pair in chain.into_inner() {
                                let node = node_pair.into_inner().next().unwrap();
                                match node.as_rule() {
                                    Rule::sin => {
                                        println!("node {:?}", node.as_str()); //"sin 440"
                                        let paras = node.into_inner().next().unwrap();
                                        println!("paras {:?}", paras.as_str());//"440"
                                        chain_node_names.push("sin");
                                        chain_paras.push(paras.as_str());
                                    },
                                    Rule::mul => {
                                        println!("node {:?}", node.as_str());
                                        let paras = node.into_inner().next().unwrap();
                                        println!("paras {:?}", paras.as_str());
                                        chain_node_names.push("mul");
                                        chain_paras.push(paras.as_str());
                                    },
                                    Rule::add => {
                                        println!("node {:?}", node.as_str());
                                        let paras = node.into_inner().next().unwrap();
                                        println!("paras {:?}", paras.as_str());
                                        chain_node_names.push("add");
                                        chain_paras.push(paras.as_str());
                                    },
                                    Rule::seq => {
                                        println!("node {:?}", node.as_str());
                                        let paras = node.into_inner().next().unwrap();
                                        println!("paras {:?}", paras.as_str());
                                        chain_node_names.push("seq");
                                        chain_paras.push(paras.as_str());
                                    },
                                    Rule::sp => {
                                        println!("node {:?}", node.as_str());
                                        let paras = node.into_inner().next().unwrap();
                                        println!("paras {:?}", paras.as_str());
                                        chain_node_names.push("sp");
                                        chain_paras.push(paras.as_str());
                                    },
                                    _ => {}
                                }
                            }
                            // println!("chain.into_inner(); {:?}", chain.into_inner());
                        },
                        _ => {}
                    }
                }
                ast.insert(key, (chain_node_names, chain_paras));
            },
            _ => {},
        };
    }
    println!("ast {:?}", ast);
}