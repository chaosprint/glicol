// choose is quite unique as it takes unlimited number of notes
use glicol_parser::*;

fn main() {
    println!("{:?}", get_ast(r#"o: saw 100 >> lpf "60@0.0 72@0.5" 1.0"#));
}