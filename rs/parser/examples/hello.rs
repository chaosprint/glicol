// use pest::Parser;
// use pest::iterators::Pairs;
use glicol_parser::*;

fn main() {
    println!("{:?}", get_ast("a: [0.1 => 0.2, 1/2 - 100_ms => 0.3]"));
    // println!("{:?}", get_ast("o: arrange ~t1 1 ~t2 3"));
    // println!("{:?}", get_ast("o: envperc 1.0 2.0"));
    // get_ast(input);
    // let line = GlicolParser::parse(Rule::block, input);
    // println!("{:?}", line);
    // }
}