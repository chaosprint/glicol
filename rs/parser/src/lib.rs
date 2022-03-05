use pest::Parser;
use pest::error::Error;
use pest_derive::*;
// use pest::iterators::Pair;
use pest::error::ErrorVariant;
use std::collections::HashMap;

use glicol_macros::one_para_number_or_ref;

#[derive(Parser)]
#[grammar = "glicol.pest"]
pub struct GlicolParser;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum GlicolPara<'a> {
    Number(f32),
    Reference(&'a str),
    Symbol(&'a str),
    // Seq(&'static str),
}

pub fn get_num(para: GlicolPara) -> f32 {
    match para {
        GlicolPara::Number(v) => v,
        _ => 0.0
    }
}

pub fn get_ast<'a>(code: &'a str) -> Result<HashMap<&'a str, (Vec<&'a str>, Vec<Vec<GlicolPara>>)>, Error<Rule>> {
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
                                    Rule::delayn =>  one_para_number_or_ref!("delayn"),
                                    Rule::delayms =>  one_para_number_or_ref!("delayms"),
                                    Rule::imp =>  one_para_number_or_ref!("imp"),
                                    Rule::tri =>  one_para_number_or_ref!("tri"),
                                    Rule::squ => one_para_number_or_ref!("squ"),
                                    Rule::saw => one_para_number_or_ref!("saw"),
                                    Rule::onepole => one_para_number_or_ref!("onepole"),
                                    Rule::sin => one_para_number_or_ref!("sin"),
                                    Rule::mul => one_para_number_or_ref!("mul"),
                                    Rule::add => one_para_number_or_ref!("add"),
                                    Rule::seq => {
                                        println!("node {:?}", node.as_str());
                                        let paras = node.into_inner().next().unwrap();
                                        println!("paras {:?}", paras.as_str());
                                        chain_node_names.push("seq");
                                        chain_paras.push(vec![GlicolPara::Number(paras.as_str().parse::<f32>().unwrap())]);
                                    },
                                    Rule::sp => {
                                        println!("node {:?}", node.as_str());
                                        let paras = node.into_inner().next().unwrap();
                                        println!("paras {:?}", paras.as_str());
                                        chain_node_names.push("sp");
                                        chain_paras.push(vec![GlicolPara::Symbol(paras.as_str())]);
                                    },
                                    Rule::constsig => one_para_number_or_ref!("constsig"),
                                    Rule::lpf => {
                                        println!("node {:?}", node.as_str());
                                        let mut iter = node.into_inner();
                                        let p1 = iter.next().unwrap();
                                        let p2 = iter.next().unwrap();
                                        chain_node_names.push("lpf");
                                        chain_paras.push(vec![
                                            match p1.as_rule() {
                                                Rule::number => 
                                                    GlicolPara::Number(p1.as_str().parse::<f32>().unwrap())
                                                ,
                                                Rule::reference => 
                                                    GlicolPara::Reference(p1.as_str())
                                                ,
                                                _ => unimplemented!()
                                            },
                                            GlicolPara::Number(p2.as_str().parse::<f32>().unwrap())
                                        ]);
                                        // println!("chain_paras, {:?}", chain_paras);
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
// macro_rules! get_paras {
//     ($rule: expr) => {
//         {
//             println!("node {:?}", node.as_str()); //"sin 440"
//             let paras = node.into_inner().next().unwrap();
//             println!("paras {:?}", paras.as_str());//"440"                                        
//             chain_node_names.push($rule);
//             match paras.as_rule() {
//                 Rule::number => {
//                     chain_paras.push(vec![GlicolPara::Number(paras.as_str().parse::<f32>().unwrap())]);
//                 },
//                 Rule::reference => {
//                     chain_paras.push(vec![GlicolPara::Reference(paras.as_str())]);
//                 },
//                 _ => {}
//             }
//         }
//     };
// }

// pub(crate) use get_paras;