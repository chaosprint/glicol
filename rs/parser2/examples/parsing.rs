use glicol_parser::{nodes, single_chain};

fn main() {
    println!("{:?}", nodes("sin 440"));
    println!("{:?}", single_chain("out: sin 440 >> mul 0.1 >> add 0.3"));
}