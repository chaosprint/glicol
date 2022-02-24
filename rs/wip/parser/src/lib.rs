use pest::Parser;
use pest_derive::*;

#[derive(Parser)]
#[grammar = "glicol.pest"]
pub struct GlicolParser;

pub fn get_ast(input: &str) { // -> HashMap<&str, Vec<(&str, &str, u8)> 
    let line = GlicolParser::parse(Rule::block, input);
    println!("{:?}", line);
}