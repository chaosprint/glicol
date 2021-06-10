use glicol_macro::*;
use glicol_synth::{SimpleGraph};
// use glicol_parser::{Rule, GlicolParser};
// use pest::Parser;
// use quote::*;

fn main() {
    // let num = 0.2;
    let mut g = make_graph!{
        ~test: lpf 400;
        out:  ~test >> add 42;
    };
    println!("{:?}", g.next_block(&mut [5.0; 128]));
    println!("{:?}", g.next_block(&mut [5.0; 128]));
}