use glicol_parser::*;

fn main() {
    println!("{:?}", get_ast("o: seq 60 _60 __60 ~a >> mul 0.5"));
}