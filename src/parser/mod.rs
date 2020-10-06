// #[macro_use]
use pest_derive::*;

#[derive(Parser)]
#[grammar = "parser/glicol.pest"]
pub struct GlicolParser;