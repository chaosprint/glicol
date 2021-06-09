use glicol_macro::*;
use glicol_synth::{SimpleGraph, GlicolNodeData};
use glicol_parser::{Rule, GlicolParser};
use pest::Parser;
use pest::iterators::Pairs;
use std::{collections::HashMap};

pub mod amplfo; use amplfo::AmpLFO;

pub fn make_node_ext(
    name: &str,
    paras: &mut Pairs<Rule>,
    samples_dict: &HashMap<String, &'static[f32]>,
    sr: usize,
    bpm: f32,
) -> Option<GlicolNodeData> {
    let n = match name {
        "amplfo" => 1,
        _ => return None
    };
    let mut pv = vec![];
    for i in 0..n {
        let p = paras.next();
        match p {
            Some(v) => {
                match v.to_string().parse::<f32>() {
                    Ok(v) => pv.push(v),
                    Err(_) => return None
                };
            },
            None => return None
        };
    };
    let node = match name {
        "amplfo" => amplfo!(pv[0]),
        _ => unimplemented!()
    };
    Some(node)
}