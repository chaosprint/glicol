// choose is quite unique as it takes unlimited number of notes
use glicol_parser::*;

fn main() {
    println!("{:?}", get_ast(r#"o: psampler "'bd'@0.0 'sd'@0.5""#));
}