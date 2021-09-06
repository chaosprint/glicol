#![allow(warnings)]
use glicol_macro::*;
use glicol_synth::{SimpleGraph, GlicolNodeData};
use glicol_parser::{Rule, GlicolParser};
use pest::Parser;
use pest::iterators::Pairs;
use std::{collections::HashMap};

// pub mod macros; use macros::*;
// register_node![Plate, Kick];


// pub mod amplfo; use amplfo::AmpLFO;
pub mod plate; use plate::Plate;
pub mod kick; use kick::*;

pub fn make_node_ext<const N: usize>(
    name: &str,
    paras: &mut Pairs<Rule>,
    pos: (usize, usize),
    samples_dict: &HashMap<String, &'static[f32]>,
    sr: usize,
    bpm: f32,
) -> Option<GlicolNodeData<N>> {
    let n = match name {
        // "amplfo" => 1,
        "plate" => 1,
        "kick" => 1,
        _ => return None
    };
    let mut pv = vec![];
    for i in 0..n {
        // let mut v;
        let mut p = match paras.next() {
            Some(v) => v.as_str(),
            None => return None
        };
        // while p.is_some() {
        //     v = p.unwrap();
        //     p = v.clone().into_inner().next();
        // };

        // no modulation here so far
        match p.parse::<f32>() {
            Ok(v) => pv.push(v),
            Err(_) => return None
        };
    };
    
    let node = match name {
        // "amplfo" => amplfo!(N => pv[0]),
        "plate" => plate!(N => pv[0]), // only one para is supported
        "kick" => kick!(N => pv[0]),
        _ => unimplemented!()
    };
    
    Some(node)
}