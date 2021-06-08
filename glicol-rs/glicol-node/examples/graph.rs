use glicol::*;
use glicol_macro::*;

fn main() {
    let mut e = make_graph!{
        out: sin 441.0 >> mul 0.5;
    };
    println!("{:?}", e.next_block().unwrap());
}