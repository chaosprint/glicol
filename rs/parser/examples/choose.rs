// choose is quite unique as it takes unlimited number of notes
use glicol_parser::*;

fn main() {
    println!("{:?}", get_ast("o: choose 60 50 80 70"));
}