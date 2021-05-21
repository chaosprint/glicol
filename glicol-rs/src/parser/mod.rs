// #[macro_use]
use pest_derive::*;

#[derive(Parser)]
#[grammar = "parser/glicol.pest"]
pub struct GlicolParser;

use super::{Pairs, Para, EngineError};
/// This function process the struct Para
/// 
/// 
pub fn process_parameters(mut paras: &mut Pairs<Rule>, num_paras: usize) -> Result<(Vec<Para>, Vec<String>), EngineError> {
    let mut processed_paras = Vec::<Para>::new();
    let mut refs = vec![];
    for _ in 0..num_paras {
        let para = paras.next();
        let mut pos = (0, 0);
        match para {
            Some(p) => {
                pos = (p.as_span().start(), p.as_span().end());
                let key = p.as_str();
                match key.parse::<f32>() {
                    Ok(v) => processed_paras.push(Para::Number(v)),
                    Err(_) => {
                        if key.contains("~") {
                            refs.push(key.to_string());
                            processed_paras.push(Para::Ref(key.to_string()))
                        } else if key.contains("\\") {
                            processed_paras.push(Para::Symbol(key.to_string()))
                        } else {
                            return Err(EngineError::ParameterError(pos))
                        }
                    }
                }                
                // return Ok(processed_paras)
            },
            None => return Err(EngineError::ParameterError(pos))
        // .chars().filter(|c| !c.is_whitespace()).collect();
        };
    };
    return Ok((processed_paras, refs))
}