#![allow(warnings)]
use glicol_macro::*;
use glicol_synth::{SimpleGraph, GlicolNodeData, GlicolError};
use glicol_parser::{Rule, GlicolParser};
use pest::Parser;
use pest::iterators::Pairs;
use std::{collections::HashMap};

pub fn preprocessor(chain_name:&str, node_name: &str, paras: &mut Pairs<Rule>) -> Result<(String, String), GlicolError> {
    
    let mut inplace_code = String::new();
    let mut appendix_code = String::new();

    let (target_paras, mut inplace, mut appendix) = match node_name {
        "mul" => (vec![1.0], "mul ~mulmod_CHAIN_NAME", "~mulmod_CHAIN_NAME: const_sig PARA_0"),        
        _ => (vec![], "", "")
    };

    inplace_code = inplace.replace("CHAIN_NAME", chain_name);
    appendix_code = appendix.replace("CHAIN_NAME", chain_name);
    
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

    Ok( (inplace_code, appendix_code) )
}

fn process_para(default: f32, input: &str) -> Result<String, GlicolError> {
    if input == "_" {
        return Ok(format!("{}", default))
    } else if input.parse::<f32>().is_ok() {
        return Ok(input.to_owned())
    } else {
        panic!();
    }
}

// def_node!("mul", [Modulable(100.0)], {
//         let freq = args[0];
//     }
//     CHAIN_NAME: SOURCE >> mul ~modulation >> SINK;
//     ~modulation: const_sig PARA1
// );

register_extensions! {
    Plate: 1,
    Kick: 2,
    Bd: 1,
    Hh: 1,
    Sn: 1,
    Ks: 3,
    Sawsynth: 2,
    Squsynth: 2,
    Trisynth: 2,
}

// remember to regitster on glicol_parser too!
// write the documentation (node description, parameter names and number, etc.) on glicol-js/glicol-docs.json