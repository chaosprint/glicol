use pest::Parser;
use pest::error::Error;
use pest_derive::*;
use pest::iterators::Pair;
use pest::error::ErrorVariant;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "glicol2.pest"]
pub struct GlicolParser;

pub fn get_glicol_ast<'a>(code: &'a str) -> Result<HashMap<&'a str, (Vec<&'a str>, Vec<Pair<'a, Rule>>)>, Error<Rule>> {
    let mut block = match GlicolParser::parse(Rule::block, code) {
        Ok(v) => v,
        Err(e) => {
            println!("in location: {:?}; line_col: {:?}", e.location, e.line_col);
            match &e.variant {
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
                },
                _ => {panic!("unknonw parsing error")}
            }
            return Err(e)
        }
    };
    let lines = block.next().unwrap(); // this can be a comment though, but we call it a line
    let mut ast = HashMap::new();
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
                                        chain_paras.push(paras);
                                    },
                                    Rule::mul => {
                                        println!("node {:?}", node.as_str());
                                        let paras = node.into_inner().next().unwrap();
                                        println!("paras {:?}", paras.as_str());
                                        chain_node_names.push("mul");
                                        chain_paras.push(paras);
                                    },
                                    Rule::add => {
                                        println!("node {:?}", node.as_str());
                                        let paras = node.into_inner().next().unwrap();
                                        println!("paras {:?}", paras.as_str());
                                        chain_node_names.push("add");
                                        chain_paras.push(paras);
                                    },
                                    Rule::seq => {
                                        println!("node {:?}", node.as_str());
                                        let paras = node.into_inner().next().unwrap();
                                        println!("paras {:?}", paras.as_str());
                                        chain_node_names.push("seq");
                                        chain_paras.push(paras);
                                    },
                                    Rule::sp => {
                                        println!("node {:?}", node.as_str());
                                        let paras = node.into_inner().next().unwrap();
                                        println!("paras {:?}", paras.as_str());
                                        chain_node_names.push("sp");
                                        chain_paras.push(paras);
                                    },
                                    Rule::constsig => {
                                        println!("node {:?}", node.as_str());
                                        let paras = node.into_inner().next().unwrap();
                                        println!("paras {:?}", paras.as_str());
                                        chain_node_names.push("constsig");
                                        chain_paras.push(paras);
                                    },
                                    Rule::lpf => {
                                        println!("node {:?}", node.as_str());
                                        let paras = node.into_inner().next().unwrap();
                                        // let mut p1 =  paras.as_str();
                                        // let p2 = paras.into_inner().next().unwrap().as_str();
                                        // println!("paras1 {:?}", p1);
                                        // println!("paras1 {:?}", p2);
                                        // p1.push_str(p2);

                                        // let paras2 = paras1.into_inner().next().unwrap();
                                        // println!("paras2 {:?}", paras2.as_str());
                                        // chain_node_names.push("lpf");
                                        chain_paras.push(paras);
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
    Ok(ast)
}