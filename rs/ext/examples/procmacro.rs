use glicol_macro::*;
use glicol_audio::{SimpleGraph};
use glicol_parser::{Rule, GlicolParser};
use pest::Parser;
// use quote::*;

fn main() {
    let num = 0.1;
    let mut g = make_graph!{
        out: sin 440.0 >> mul #num;
    };
    println!("{:?}", g.next_block(&mut [0.0; 128]));
}