use pest_derive::*;

#[derive(Parser)]
#[grammar = "parser/glicol.pest"]
pub struct GlicolParser;

use super::{Pairs, EngineError, Para};
/// This function process the struct Para
/// 
/// seperate  1. numbers, symbols 2. refs
/// 
/// make sure the paras + refs is correct
/// 
/// TODOs:
/// What if some paras are not modulable? how to report as engine error? NoneModulableError ✔
/// 
/// What if we need a number or ref but provide with a symbol?  ParaTypeError
/// 
/// What if the paras are endless, such as in the `seq` node? We bypass this function and only send the str to Seq
pub fn process_parameters(paras: &mut Pairs<Rule>, mut modulable: Vec<Para>) -> Result<(Vec<Para>, Vec<String>), EngineError> {
    let mut refs = vec![];
    for i in 0..modulable.len() {
        let para = paras.next();
        let mut pos = (0, 0);
        match para {
            Some(p) => {
                pos = (p.as_span().start(), p.as_span().end());
                let key = p.as_str();
                match key.parse::<f32>() {
                    Ok(v) => modulable[i] = Para::Number(v),
                    Err(_) => {
                        if key.contains("~") {
                            if modulable[i] != Para::Modulable { 
                                return Err(EngineError::NotModuableError(pos)) 
                            } else {
                                refs.push(key.to_string());
                            }
                        } else if key.contains("\\") {
                            modulable[i] =  Para::Symbol(key.to_string())
                        } else {
                            return Err(EngineError::ParameterError(pos))
                        }
                    }
                }
            },
            None => return Err(EngineError::InsufficientParameter(pos))
        // .chars().filter(|c| !c.is_whitespace()).collect();
        };
    };
    return Ok((modulable, refs))
}

// fn para_type(para: &str) -> Para {
// }