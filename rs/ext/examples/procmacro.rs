use glicol_macro::*;
use glicol_synth::{SimpleGraph};
use glicol_parser::{Rule, GlicolParser};
use pest::Parser;
// use quote::*;

fn main() {
    let num = 0.1;
    let mut g = make_graph!{
        out: ~input >> add 0.1;
    };
    println!("{:?}", g.next_block(&mut [0.0; 128]));
}