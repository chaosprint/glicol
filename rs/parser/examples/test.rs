// use pest::Parser;
// use pest::iterators::Pairs;
use glicol_parser::*;

fn main() {
    println!("{:?}", get_ast("o: mix ~a ~b ~c"));
    // get_ast(input);
    // let line = GlicolParser::parse(Rule::block, input);
    // println!("{:?}", line);
    // }
}